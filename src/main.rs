#![allow(warnings)]
use core::arch;
use std::error::Error;
use std::path::PathBuf;
use std::mem::size_of;


use moscope::macho::constants::*;
use moscope::macho::fat;
use moscope::macho::header;
use moscope::macho::load_commands;
use moscope::macho::rpaths::ParsedRPath;
use moscope::macho::segments;
use moscope::macho::dylibs;
use moscope::macho::rpaths;
use moscope::reporting::macho::build_architecture_report;
use moscope::reporting::macho::{MachOReport, ArchitectureReport, build_macho_report};
use moscope::reporting::header::MachHeaderReport;
use moscope::reporting::load_commands::LoadCommandReport;
use moscope::reporting::segments::SegmentReport;
use moscope::reporting::dylibs::DylibReport;
use moscope::reporting::rpaths::RPathsReport;

use colored::{control, Colorize};
use serde_json::to_string_pretty;
use std::io::IsTerminal;

use clap::{Parser, ValueEnum};


#[derive(Clone, Debug, ValueEnum, PartialEq)]
pub enum OutputFormat {
    Text,
    Json,
}


#[derive(Parser, Debug)]
#[command(
    name = "moscope",
    version,
    about = "Mach-O static analysis and inspection toolkit"
)]
struct Cli {
    /// Path to the Mach-O binary to inspect
    #[arg(value_name = "BINARY")]
    binary: PathBuf,

    // Disable color output
    #[arg(long)]
    pub no_color: bool,

    // JSON or the printed output
    #[clap(value_enum, long, default_value = "text")]
    format: OutputFormat,
}


fn decode_arm64_subtype(cpusubtype: i32) -> &'static str {
    let base = cpusubtype & !CPU_SUBTYPE_MASK;
    let has_ptrauth = (cpusubtype & CPU_SUBTYPE_PTRAUTH_ABI) != 0;

    if has_ptrauth {
        "arm64e"
    } else {
        match base {
            CPU_SUBTYPE_ARM64_ALL |
            CPU_SUBTYPE_ARM64_V8 => "arm64",
            _ =>  "arm64 (unknown subtype)",
        }
    }
}

fn display_arch(cputype: i32, cpusubtype: i32) -> (&'static str, &'static str) {
    let cpu = cpu_type_name(cputype);

    let subtype = match cputype {
        CPU_TYPE_ARM64 => decode_arm64_subtype(cpusubtype),
        _ => cpu_subtype_name(cputype, cpusubtype),
    };

    (cpu, subtype)
}

fn fat_binary_user_decision<'a>(archs: &'a [fat::FatArch]) -> Result<&'a fat::FatArch, Box<dyn Error>> {
    // Prompt user if they want to analyze the Intel or Apple Silicon binary (or whichever of the `n`` binaries present)
    println!("{}", "Available architectures:".green().bold());
    for (i, arch) in archs.iter().enumerate() {
        match arch {
            fat::FatArch::Arch32(a) => {
                let (cpu, sub) = display_arch(a.cputype, a.cpusubtype);
                println!("{i}: {cpu} ({sub})");
            }
            fat::FatArch::Arch64(a) => {
                let (cpu, sub) = display_arch(a.cputype, a.cpusubtype);
                println!("{i}: {cpu} ({sub})");
            }
        }
    }

    use std::io::{self, Write};
    print!("Select architecture index: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let index: usize = input.trim().parse()?;

    Ok(&archs[index])
}


fn main() -> Result<(), Box<dyn Error>> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Disable coloring if desired or if terminal isn't a TTY
    if cli.no_color || !std::io::stdout().is_terminal() {
        control::set_override(false);
    }

    // Read the entire file into memory
    let data = std::fs::read(&cli.binary)
        .map_err(|e| format!("failed to read '{}': {}", cli.binary.display(), e))?;

    // Detect if fat/universal binary
    let fat_header = fat::read_fat_header(&data).ok();
    let is_fat = fat_header.is_some();
    let is_json = cli.format == OutputFormat::Json;

    // Prepare architecture slices
    let arch_slices: Vec<header::MachOSlice> = if let Some(fat_hdr) = &fat_header {
        let archs = fat::read_fat_archs(&data, fat_hdr)?;
        if let OutputFormat::Json = cli.format {
            // If JSON, do all architectures automatically
            archs.iter().map(|arch| match arch {
                fat::FatArch::Arch32(a) => header::MachOSlice { offset: a.offset as u64, size: Some(a.size as u64) },
                fat::FatArch::Arch64(a) => header::MachOSlice { offset: a.offset, size: Some(a.size) },
            }).collect()
        } else {
            // Otherwise, prompt user for selection
            let selected_arch = fat_binary_user_decision(&archs)?;
            vec![match selected_arch {
                fat::FatArch::Arch32(a) => header::MachOSlice { offset: a.offset as u64, size: Some(a.size as u64) },
                fat::FatArch::Arch64(a) => header::MachOSlice { offset: a.offset, size: Some(a.size) },
            }]
        }
    } else {
        vec![header::MachOSlice { offset: 0, size: None }]
    };

    // Store ArchitectureReports and parsed structs for printing
    let mut architecture_reports = Vec::new();
    let mut all_parsed_headers = Vec::new();
    let mut all_parsed_segments = Vec::new();
    let mut all_parsed_dylibs = Vec::new();
    let mut all_parsed_rpaths = Vec::new();
    let mut all_load_commands = Vec::new();

    for slice in arch_slices {
        // Read Mach-O header for this slice
        let thin_header: header::ParsedMachOHeader = header::read_thin_header(&data, &slice)?;
        all_parsed_headers.push(thin_header.header.clone());

        // Determine header variant info
        let (header_size, ncmds, word_size, is_be) = match &thin_header.header {
            header::MachOHeader::Header32(h) => (
                std::mem::size_of::<header::MachHeader32>(),
                h.ncmds,
                32,
                thin_header.kind.is_be(),
            ),
            header::MachOHeader::Header64(h) => (
                std::mem::size_of::<header::MachHeader64>(),
                h.ncmds,
                64,
                thin_header.kind.is_be(),
            ),
        };

        let load_command_offset = slice.offset as usize + header_size;
        let load_commands_vec = load_commands::read_load_commands(&data, load_command_offset as u32, ncmds, word_size, is_be)?;

        let mut parsed_segments = Vec::new();
        let mut parsed_dylibs = Vec::new();
        let mut parsed_rpaths = Vec::new();

        for lc in &load_commands_vec {
            let base_cmd = lc.cmd & !LC_REQ_DYLD;

            match base_cmd {
                LC_ID_DYLIB
                | LC_LOAD_DYLIB
                | LC_LOAD_WEAK_DYLIB
                | LC_REEXPORT_DYLIB
                | LC_LAZY_LOAD_DYLIB
                | LC_LOAD_UPWARD_DYLIB => {
                    parsed_dylibs.push(dylibs::parse_dylib(&data, lc, is_be)?);
                }
                LC_RPATH => {
                    parsed_rpaths.push(rpaths::parse_rpath(&data, lc, is_be)?);
                }
                LC_SEGMENT_64 => {
                    parsed_segments.push(segments::parse_segment_64(&data, lc.offset as usize, is_be)?);
                }
                LC_SEGMENT => {
                    parsed_segments.push(segments::parse_segment_32(&data, lc.offset as usize, is_be)?);
                }
                _ => {}
            }
        }

        
        // Build architecture report for JSON
        let arch_report = build_architecture_report(
            match &thin_header.header {
                header::MachOHeader::Header32(h) => h.cputype,
                header::MachOHeader::Header64(h) => h.cputype,
            },
            match &thin_header.header {
                header::MachOHeader::Header32(h) => h.cpusubtype,
                header::MachOHeader::Header64(h) => h.cpusubtype,
            },
            &thin_header.header,
            &load_commands_vec,
            &parsed_segments,
            &parsed_dylibs,
            &parsed_rpaths,
            is_json

        );

        architecture_reports.push(arch_report);
        all_parsed_segments.push(parsed_segments);
        all_parsed_dylibs.push(parsed_dylibs);
        all_parsed_rpaths.push(parsed_rpaths);
        all_load_commands.push(load_commands_vec);
    }

    // Build final MachOReport
    let macho_report = build_macho_report(is_fat, architecture_reports);

    // Now output
    match cli.format {
        OutputFormat::Text => {
            println!("{}", "Mach-O Report:".green().bold());
            for i in 0..macho_report.architectures.len() {
                let header = &all_parsed_headers[i]; 
                let segments = &all_parsed_segments[i];
                let dylibs = &all_parsed_dylibs[i];
                let rpaths = &all_parsed_rpaths[i];
                let load_cmds = &all_load_commands[i];

                header::print_header_summary(header);
                segments::print_segments_summary(segments);
                dylibs::print_dylibs_summary(dylibs);
                rpaths::print_rpaths_summary(rpaths);
                load_commands::print_load_commands(load_cmds);
            }
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&macho_report)?;
            println!("{}", json);
        }
    }

    Ok(())
}
