#![allow(warnings)]
use std::error::Error;
use std::path::PathBuf;

use moscope::macho::constants::{LC_SEGMENT, LC_SEGMENT_64};
use moscope::macho::fat;
use moscope::macho::header;
use moscope::macho::constants;
use moscope::macho::load_commands;
use moscope::macho::segments;

use colored::control;
use colored::Colorize;
use std::io::IsTerminal;

use clap::Parser;




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
}


fn decode_arm64_subtype(cpusubtype: i32) -> &'static str {
    let base = cpusubtype & !constants::CPU_SUBTYPE_MASK;
    let has_ptrauth = (cpusubtype & constants::CPU_SUBTYPE_PTRAUTH_ABI) != 0;

    if has_ptrauth {
        "arm64e"
    } else {
        match base {
            constants::CPU_SUBTYPE_ARM64_ALL |
            constants::CPU_SUBTYPE_ARM64_V8 => "arm64",
            _ =>  "arm64 (unknown subtype)",
        }
    }
}

fn display_arch(cputype: i32, cpusubtype: i32) -> (&'static str, &'static str) {
    let cpu = constants::cpu_type_name(cputype);

    let subtype = match cputype {
        constants::CPU_TYPE_ARM64 => decode_arm64_subtype(cpusubtype),
        _ => constants::cpu_subtype_name(cputype, cpusubtype),
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


    Ok(&archs[index]) // Return a reference, fat arch lives as long as `archs` does which is also a reference from earlier so should be fine
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Disable coloring if desired or if terminal isn't a TTY
    if cli.no_color || !std::io::stdout().is_terminal() {
        control::set_override(false);
    }



    println!("Checking for universal binary...");

    // Read the entire file into memory
    // Then all fat header parse & mach-o header parser can work from the same byte slice
    // but...hopefully it's not a large binary (need to change later)
    let data = std::fs::read(&cli.binary)
        .map_err(|e| format!("failed to read '{}': {}", cli.binary.display(), e))?;
    

    // Attempt to parse a FAT (universal) header
    // If successful, the file contains multiple Mach-O images
    // If it fails, the file is a thin/singular Mach-O binary


    let macho_slice = match fat::read_fat_header(&data) {
        Ok(fat_header) => {
            println!("{}", "Fat binary detected:".green().bold());
            //println!("{:?}", fat_header);

            // Parse all architecture entries described by the fat header

            let archs = fat::read_fat_archs(
                &data,
                &fat_header,
            )?;

            // Present architectures to user for selection
            let selected_arch = fat_binary_user_decision(&archs)?;

            let slice: header::MachOSlice = match selected_arch {
                fat::FatArch::Arch32(a) => header::MachOSlice {
                    offset: a.offset as u64, // u32 offset but it's as a u64
                    size: Some(a.size as u64),
                },
                fat::FatArch::Arch64(a) => header::MachOSlice {
                    offset: a.offset, // u64 offset
                    size: Some(a.size),
                },
            };

            slice

            // end of fat binary specific code, proceed to thin binary code

        }
        Err(_) => {
            println!("{}", "No universal binary detected!".yellow().bold());
            header::MachOSlice {
                offset: 0, // Thin binary -> no offset, start right away
                size: None, // Irrelevant
            }
        }
    };

    let thin_header: header::ParsedMachOHeader = header::read_thin_header(&data, &macho_slice)?;

    use std::mem::size_of;

    // Determine the variant we have (word size, endianness, etc.) so we can properly read the load commands
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

    let load_command_offset = macho_slice.offset as usize + header_size;
    let load_commands = load_commands::read_load_commands(&data, load_command_offset as u32, ncmds, word_size, is_be)?;
    load_commands::print_load_commands(&load_commands);

    // Now check load_commands for LOAD_SEGMENT and LOAD_SEGMENT_64
    let mut parsed_segments = Vec::new();
    for lc in load_commands {
        match lc.cmd {
            LC_SEGMENT_64 => {
                let seg = segments::parse_segment_64(&data, lc.offset as usize, is_be)?;
                parsed_segments.push(seg);
            }

            LC_SEGMENT => {
                let seg = segments::parse_segment_32(&data, lc.offset as usize, is_be)?;
                parsed_segments.push(seg);
            }

            _ => {
                () // ignore non LC_SEGMENT* LC's
            }
        }
    }

    segments::print_segments_summary(&parsed_segments);

    Ok(())
}
