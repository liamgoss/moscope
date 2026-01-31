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
use moscope::macho::sections::SectionKind;
use moscope::macho::dylibs;
use moscope::macho::rpaths;
use moscope::macho::symtab;
use moscope::macho::symtab::DYSymtabCommand;
use moscope::macho::utils::{bytes_to,byte_array_to_string};
use moscope::macho::memory_image::MachOMemoryImage;
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
use std::collections::HashMap;

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

    // Flags for output filtering
    #[arg(long, default_value_t = 4)]
    min_string_length: usize,

    #[arg(long)]
    max_strings: Option<usize>,

    #[arg(long)]
    no_symbols: bool,

    #[arg(long)]
    no_segments: bool,

    #[arg(long)]
    no_loadcmds: bool,

    #[arg(long)]
    no_header: bool,

    #[arg(long)]
    no_strings: bool,
    
    #[arg(long)]
    no_dylibs: bool,

    #[arg(long)]
    no_rpaths: bool,

    #[arg(long)]
    max_symbols: Option<usize>,

    #[arg(long)]
    include_debug_symbols: bool,

    // String filtering
    /// Filter strings by regex pattern (e.g., "^http", "\.dylib$", "password")
    #[arg(long)]
    string_pattern: Option<String>,

    /// Only extract strings from specific sections (comma-separated)
    /// Example: --string-sections __cstring,__const
    #[arg(long, value_delimiter = ',')]
    string_sections: Option<Vec<String>>,

    /// Skip string extraction from specific sections (comma-separated)
    /// Example: --skip-sections __objc_methtype
    #[arg(long, value_delimiter = ',')]
    skip_sections: Option<Vec<String>>,

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

    let min_len = cli.min_string_length;
    let max_strings_count = cli.max_strings;
    let max_symbols_count = cli.max_symbols;

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
    // all_* is to handle the reports for BOTH slices 
    let mut architecture_reports = Vec::new();
    let mut all_parsed_headers = Vec::new();
    let mut all_parsed_segments = Vec::new();
    let mut all_parsed_dylibs = Vec::new();
    let mut all_parsed_rpaths = Vec::new();
    let mut all_load_commands = Vec::new();
    let mut all_parsed_symbols: Vec<Vec<symtab::ParsedSymbol>> = Vec::new();
    let mut all_parsed_strings: Vec<Vec<symtab::ParsedString>> = Vec::new();

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
        let mut parsed_symbols: Vec<symtab::ParsedSymbol> = Vec::new();
        let mut parsed_strings = Vec::new();

        // LC_SYMTAB doesn't contain symbols it just declares info
        // So we need to keep track of it so we can get all the symbols
        let mut symtab_cmd: Option<symtab::SymtabCommand> = None;
        let mut dysymtab_cmd: Option<symtab::DYSymtabCommand> = None;

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

                LC_SYMTAB => {
                    let cmd = symtab::SymtabCommand {
                        cmd: lc.cmd,
                        cmdsize: lc.cmdsize,
                        symoff: bytes_to(is_be, &data[lc.offset as usize + 8 .. lc.offset as usize + 12])?,
                        nsyms: bytes_to(is_be, &data[lc.offset as usize + 12 .. lc.offset as usize + 16])?,
                        stroff: bytes_to(is_be, &data[lc.offset as usize + 16 .. lc.offset as usize + 20])?,
                        strsize: bytes_to(is_be, &data[lc.offset as usize + 20 .. lc.offset as usize + 24])?,
                    };

                    symtab_cmd = Some(cmd);   
                }

                LC_DYSYMTAB => {
                    let off = lc.offset as usize;
                    let cmd = symtab::DYSymtabCommand {
                        cmd: lc.cmd,
                        cmdsize: lc.cmdsize,
                        ilocalsym:       bytes_to(is_be, &data[off +  8 .. off + 12])?,
                        nlocalsym:       bytes_to(is_be, &data[off + 12 .. off + 16])?,
                        iextdefsym:      bytes_to(is_be, &data[off + 16 .. off + 20])?,
                        nextdefsym:      bytes_to(is_be, &data[off + 20 .. off + 24])?,
                        iundefsym:       bytes_to(is_be, &data[off + 24 .. off + 28])?,
                        nundefsym:       bytes_to(is_be, &data[off + 28 .. off + 32])?,
                        tocoff:          bytes_to(is_be, &data[off + 32 .. off + 36])?,
                        ntoc:            bytes_to(is_be, &data[off + 36 .. off + 40])?,
                        modtaboff:       bytes_to(is_be, &data[off + 40 .. off + 44])?,
                        nmodtab:         bytes_to(is_be, &data[off + 44 .. off + 48])?,
                        extrefsymoff:    bytes_to(is_be, &data[off + 48 .. off + 52])?,
                        nextrefsyms:     bytes_to(is_be, &data[off + 52 .. off + 56])?,
                        indirectsymoff:  bytes_to(is_be, &data[off + 56 .. off + 60])?,
                        nindirectsyms:   bytes_to(is_be, &data[off + 60 .. off + 64])?,
                        extreloff:       bytes_to(is_be, &data[off + 64 .. off + 68])?,
                        nextrel:         bytes_to(is_be, &data[off + 68 .. off + 72])?,
                        locreloff:       bytes_to(is_be, &data[off + 72 .. off + 76])?,
                        nlocrel:         bytes_to(is_be, &data[off + 76 .. off + 80])?,
                    };

                    dysymtab_cmd = Some(cmd);
                }
                _ => {}
            }
        }

        // now we take a look @ our symtab_cmd and parse symbols
        if let Some(symtab) = symtab_cmd {
            let sym_base = symtab.symoff as usize;
            let stroff = slice.offset as usize + symtab.stroff as usize; // have to add the fat offset otherwise we just read garbage
            let strsize = symtab.strsize as usize;

            // report up to N symbols where N is defined by the --max_symbols flag
            for i in 0..symtab.nsyms {

                let size = if thin_header.kind.is_64() {
                    symtab::NList64::SIZE
                } else {
                    symtab::NList32::SIZE
                };

                let offset = slice.offset as usize + sym_base + (i as usize) * size; // have to add the fat offset otherwise we just read garbage

                let symbol = if thin_header.kind.is_64() {
                    let nlist = symtab::NList64::parse(&data, offset, is_be)?;
                    symtab::ParsedSymbol::from_nlist64(nlist, &data, stroff, strsize)
                } else {
                    let nlist = symtab::NList32::parse(&data, offset, is_be)?;
                    symtab::ParsedSymbol::from_nlist32(nlist, &data, stroff, strsize)
                };

                parsed_symbols.push(symbol);
            }
        }

        // now for indirect symbols ingestion
        let mut indirect_symbols: Option<Vec<u32>> = None;
        if let Some(dysym) = &dysymtab_cmd {
            let base = slice.offset as usize + dysym.indirectsymoff as usize;

            let mut table = Vec::with_capacity(dysym.nindirectsyms as usize);

            for i in 0..dysym.nindirectsyms {
                let off = base + (i as usize * 4);
                let idx: u32 = bytes_to(is_be, &data[off..off+4])?;
                table.push(idx);
            }

            indirect_symbols = Some(table);
        }

        // Strings extraction using the vm addressing instead of file offsets
        //      because our file offsets method fails for dyld extracted binaries
        
        // Build VM image once per slice
        let vm_image = MachOMemoryImage::new(&parsed_segments, &data, slice.offset);

        // Before building report grab the strings
        // Iterate only __cstring sections; each byte is scanned once
        // Real cost of this is not O(n^3) like I thought but it's actually roughly O(C + B + K)
        // C = total number of sections across all segments
        // B = total bytes scanned in __cstring
        // K = number of extracted strings
        for segment in &parsed_segments {
            for section in &segment.sections {
                // Check if we should skip this section
                if let Some(ref skip) = cli.skip_sections {
                    let sectname = byte_array_to_string(&section.sectname);
                    if skip.iter().any(|s| sectname == *s) {
                        continue;
                    }
                }

                // Check if we should only process specific sections
                if let Some(ref only) = cli.string_sections {
                    let sectname = byte_array_to_string(&section.sectname);
                    if !only.iter().any(|s| sectname == *s) {
                        continue;
                    }
                }

                if section.kind == SectionKind::CString && section.size > 0 {
                    if let Some(sec_bytes) = vm_image.read_section(section) {
                        // Use filtered extraction if pattern provided, otherwise normal
                        let extracted_strings = if let Some(ref pattern) = cli.string_pattern {
                            match symtab::extract_filtered_strings(sec_bytes, pattern) {
                                Ok(strings) => strings,
                                Err(e) => {
                                    eprintln!("Invalid regex pattern '{}': {}", pattern, e);
                                    Vec::new()
                                }
                            }
                        } else {
                            symtab::extract_strings(sec_bytes, min_len)
                        };
                        
                        // Attach section info to string
                        for s in extracted_strings {
                            if s.is_empty() { continue; }
                            parsed_strings.push(symtab::ParsedString {
                                value: s,
                                segname: segment.segname.clone(),
                                sectname: section.sectname.clone(),
                            });
                        }
                    }
                }

                // If this section uses indirect symbols
                if let (Some(indirect), Some(_dysym)) = (&indirect_symbols, &dysymtab_cmd) {
                    if section.kind.uses_indirect_symbols() {
                        let start = section.reserved1 as usize;
                        let entry_size = if section.reserved2 != 0 {
                            section.reserved2 as usize
                        } else {
                            8 // arm64 defualt pointer/stub size
                        };

                        let count = (section.size as usize) / entry_size;

                        for i in 0..count {
                            let indirect_index = indirect[start + i];

                            // special dyld values
                            if indirect_index == INDIRECT_SYMBOL_ABS || indirect_index == INDIRECT_SYMBOL_LOCAL {
                                continue; // skip
                            }

                            let sym = &mut parsed_symbols[indirect_index as usize];

                            sym.indirect_sect = Some(byte_array_to_string(&section.sectname));
                            sym.segname = Some(byte_array_to_string(&section.segname));
                            sym.indirect_addr = Some(section.addr + (i as u64) * entry_size as u64); // now the undefined symbols can have an address like otool -Iv
                            
                            if sym.kind == symtab::SymbolKind::Undefined && sym.is_external {
                                sym.kind = match byte_array_to_string(&section.sectname).as_str() {
                                    "__la_symbol_ptr" => symtab::SymbolKind::Lazy,
                                    "__stubs"         => symtab::SymbolKind::Stub,
                                    "__got"           => symtab::SymbolKind::Got,
                                    _                 => sym.kind,
                                };
                            }
                        }
                    }
                }
                
            }
        }

        
        let mut global_sect_index: u8 = 1;
        // Put the section data into the hashmap 
        let mut section_map = HashMap::new();
        for segment in &parsed_segments {
            for section in &segment.sections {
                section_map.insert(global_sect_index, (
                    byte_array_to_string(&segment.segname),
                    byte_array_to_string(&section.sectname),
                ));
                global_sect_index += 1;
            }
        }

        // Use the hashmap to map symbols to the segments/sections they live in 
        // I am using the hashmap because the other way I first thought was going to be quadratic time complexity
        // This should be closer to linear
        for sym in &mut parsed_symbols {
            if let Some(idx) = sym.section.map(|s| s.0) {
                if let Some((segname, sectname)) = section_map.get(&idx) {
                    sym.segname = Some(segname.clone());   // String
                    sym.sectname = Some(sectname.clone()); // String
                }
            }
        }

        // Before building the architecture report, apply max limit if specified
        if let Some(max) = max_strings_count {
            parsed_strings.truncate(max);
        }

        if !cli.include_debug_symbols {  // Take out debug symbols
            parsed_symbols.retain(|sym| !sym.is_debug);
        }

        if let Some(limit) = max_symbols_count {
            parsed_symbols.truncate(limit);
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
            &parsed_symbols,
            &parsed_strings,
            is_json
        );

        architecture_reports.push(arch_report);
        all_parsed_segments.push(parsed_segments);
        all_parsed_dylibs.push(parsed_dylibs);
        all_parsed_rpaths.push(parsed_rpaths);
        all_load_commands.push(load_commands_vec);
        all_parsed_symbols.push(parsed_symbols);
        all_parsed_strings.push(parsed_strings);
        
        // end of this slice
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
                let symbols = &all_parsed_symbols[i];
                let strings = &all_parsed_strings[i];

                if !cli.no_header {
                    header::print_header_summary(header);
                }
                if !cli.no_segments {
                    segments::print_segments_summary(segments);
                }
                if !cli.no_dylibs {
                    dylibs::print_dylibs_summary(dylibs);
                }
                if !cli.no_rpaths {
                    rpaths::print_rpaths_summary(rpaths);
                }
                if !cli.no_loadcmds {
                    load_commands::print_load_commands(load_cmds);
                }
                if !cli.no_symbols {
                    symtab::print_symbols_summary(symbols);
                }
                if !cli.no_strings {
                    symtab::print_strings_summary(strings, min_len, max_strings_count);
                }
            }
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(&macho_report)?;
            println!("{}", json);
        }
    }

    Ok(())
}
