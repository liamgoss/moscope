// File Purpose: Handle LC_DYLD_INFO and LC_DYLD_INFO_ONLY and fixups

/*
    There's two types of fixups: classic and chained
    Classic:
        LC_DYLD_INFO
        rebase opcodes
        bind/weak/lazy opcodes
        ULEB streams

    Chained:
        LC_DYLD_CHAINED_FIXUPS
        pointer chains embedded in place
        no opcodes

    From what I can tell looking @ the source code of dyld.
    EX: for rebases,
    Classic --> dyld/dyld3/MachOAnalyzer::forEachRebase
    Chained --> dyld/dyld3/MachOLoaded::ChainedFixupPointerOnDisk::isRebase
    

*/
use std::error::Error;
use std::mem;
use colored::Colorize;
use crate::macho::constants::*;
use crate::macho::memory_image::MachOMemoryImage;
use crate::macho::segments::ParsedSegment;
use crate::macho::utils::{byte_array_to_string, read_sleb, read_uleb};
use crate::macho::{sections::ParsedSection, symtab::ParsedSymbol};
use crate::reporting::dyld::FixupReport;


// from mach-o/loader.h
#[derive(Debug, Clone, Copy)]
pub struct DYLDInfoCommand {
    pub cmd: u32,                   // LC_DYLD_INFO or LC_DYLD_INFO_ONLY
    pub cmdsize: u32,               // sizeof(struct dyld_info_command)
    // Adjust for ASLR
    pub rebase_off: u32,            // file offset to rebase info
    pub rebase_size: u32,           // size of rebase info
    // External symbols from other libraries
    // primarily going to be __DATA_CONST/__got
    pub bind_off: u32,              // file offset to binding info
    pub bind_size: u32,             // size of binding info
    // Handle weak symbols (that may not exist)
    // __la_symbol_ptr and also __got
    pub weak_bind_off: u32,         // file offset to weak binding info
    pub weak_bind_size: u32,        // size of the weak binding info
    // Delay binding symbols until they're used first 
    // __la_symbol_ptr holds lazy pointers
    pub lazy_bind_off: u32,         // file offset to lazy binding
    pub lazy_bind_size: u32,        // size of lazy binding info
    // Lists symbols binary provides to others
    pub export_off: u32,            // file offset to lazy binding info
    pub export_size: u32,           // size of lazy binding info
}

#[derive(Debug, Clone)]
pub struct ThreadedBindEntry {
    dylib_ordinal: i32,
    symbol_name: String,
    bind_type: u8,
    addend: i64
}

#[derive(Debug, Clone)]
pub enum Fixup {
    Rebase {
        addr: u64, // virtual address of the rebased pointer
    },
    Bind {
        addr: u64, // where to write the symbol address
        symbol: String, // the symbol itself
        addend: i64, // addend )
    },
    WeakBind {
        addr: u64,
        symbol: String,
        addend: i64,
    },
    LazyBind {
        addr: u64,
        symbol: String,
        addend: i64,
    },
}

impl Fixup  {
    pub fn build_report(&self) -> FixupReport {
        match self {
            Fixup::Rebase { addr } => FixupReport {
                kind: "rebase".into(),
                addr: *addr,
                addr_hex: format!("{:#x}", addr),
                symbol: None,
                addend: None,
            },

            Fixup::Bind { addr, symbol, addend } => FixupReport {
                kind: "bind".into(),
                addr: *addr,
                addr_hex: format!("{:#x}", addr),
                symbol: Some(symbol.clone()),
                addend: Some(*addend),
            },

            Fixup::WeakBind { addr, symbol, addend } => FixupReport {
                kind: "weak_bind".into(),
                addr: *addr,
                addr_hex: format!("{:#x}", addr),
                symbol: Some(symbol.clone()),
                addend: Some(*addend),
            },

            Fixup::LazyBind { addr, symbol, addend } => FixupReport {
                kind: "lazy_bind".into(),
                addr: *addr,
                addr_hex: format!("{:#x}", addr),
                symbol: Some(symbol.clone()),
                addend: Some(*addend),
            },
        }
    }

    pub fn parse(
        dyld_info: &DYLDInfoCommand,
        segments: &[ParsedSegment],
        symbols: &[ParsedSymbol],
        slide: u64, // ASLR slide
        memory: &MachOMemoryImage,
        data: &[u8], // mach-o bytes
    ) -> Result<Vec<Fixup>, Box<dyn Error>> {
        let mut fixups = Vec::new();

        // parse each type of classic fixup 
        Self::parse_rebase(dyld_info, segments, slide, data, &mut fixups)?;
        Self::parse_bind(dyld_info, segments, symbols, slide, data, memory, &mut fixups)?;
        
        Ok(fixups)
    }

    fn parse_rebase(
        dyld_info: &DYLDInfoCommand,
        segments: &[ParsedSegment],
        slide: u64, // ASLR slide
        data: &[u8], // mach-o bytes
        fixups: &mut Vec<Fixup>
    ) -> Result<(), Box<dyn Error>> {
        let start = dyld_info.rebase_off as usize;
        let end = start + dyld_info.rebase_size as usize;
        let stream = &data[start..end];

        let mut cursor = 0;
        let mut address: u64 = 0;
        let mut type_ = 0;

        // https://github.com/apple-opensource/dyld/blob/master/dyld3/MachOAnalyzer.cpp#L1444
        // Apple's dyld/dyld3/MachOAnalyzer::forEachRebase handles the rebasing
        

        while cursor < stream.len() {
            let opcode = stream[cursor];
            cursor += 1;
            match opcode & REBASE_OPCODE_MASK {
                REBASE_OPCODE_SET_TYPE_IMM => { // 0x10
                    type_ = opcode & REBASE_IMMEDIATE_MASK; // SET_TYPE_IMM
                }
                REBASE_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB => { // 0x20
                    let seg_index = (opcode & REBASE_IMMEDIATE_MASK) as usize;
                    println!("cursor={} opcode=0x{:02x} segment_index={} segment_offset={} next bytes={:?}",
                        cursor, opcode, seg_index, address, &stream[cursor..cursor+10.min(stream.len()-cursor)]);
                    let offset = read_uleb(stream, &mut cursor)?;
                    let seg = &segments[seg_index];
                    address = seg.vmaddr + offset;
                }

                REBASE_OPCODE_ADD_ADDR_ULEB => { // 0x30
                    println!("cursor={} opcode=0x{:02x} segment_index={} segment_offset={} next bytes={:?}",
                        cursor, opcode, 0, address, &stream[cursor..cursor+10.min(stream.len()-cursor)]);
                    let delta = read_uleb(stream, &mut cursor)?;
                    address = address.checked_add(delta)
                        .ok_or("address overflow during ADD_ADDR_ULEB")?;

                }

                REBASE_OPCODE_ADD_ADDR_IMM_SCALED => { // 0x40
                    let scale = (opcode & REBASE_IMMEDIATE_MASK) as u64;
                    address += scale * 8; // 8 --> scale * pointer size
                }
                REBASE_OPCODE_DO_REBASE_IMM_TIMES => { // 0x50
                    let count = (opcode & REBASE_IMMEDIATE_MASK) as u64;
                    for _ in 0..count {
                        fixups.push(Fixup::Rebase { addr: address + slide });
                        address += 8; // 64 bit pointer size
                    }
                }
                REBASE_OPCODE_DO_REBASE_ULEB_TIMES => { // 0x60
                    println!("cursor={} opcode=0x{:02x} segment_index={} segment_offset={} next bytes={:?}",
                        cursor, opcode, 0, address, &stream[cursor..cursor+10.min(stream.len()-cursor)]);
                    let count = read_uleb(stream, &mut cursor)?;
                    for _ in 0..count {
                        fixups.push(Fixup::Rebase { addr: address + slide });
                        address += 8;
                    }
                }

                REBASE_OPCODE_DO_REBASE_ADD_ADDR_ULEB => { // 0x70
                    fixups.push(Fixup::Rebase { addr: address + slide });
                    println!("cursor={} opcode=0x{:02x} segment_index={} segment_offset={} next bytes={:?}",
                        cursor, opcode, 0, address, &stream[cursor..cursor+10.min(stream.len()-cursor)]);
                    let skip = read_uleb(stream, &mut cursor)?;
                    address = address.checked_add(skip + 8)
                        .ok_or("address overflow during DO_REBASE_ADD_ADDR_ULEB")?;
                }

                REBASE_OPCODE_DO_REBASE_ULEB_TIMES_SKIPPING_ULEB => { // 0x80
                    println!("cursor={} opcode=0x{:02x} segment_index={} segment_offset={} next bytes={:?}",
                        cursor, opcode, 0, address, &stream[cursor..cursor+10.min(stream.len()-cursor)]);
                    let count = read_uleb(stream, &mut cursor)?;
                    println!("cursor={} opcode=0x{:02x} segment_index={} segment_offset={} next bytes={:?}",
                        cursor, opcode, 0, address, &stream[cursor..cursor+10.min(stream.len()-cursor)]);
                    let skip = read_uleb(stream, &mut cursor)?;
                    for _ in 0..count {
                        fixups.push(Fixup::Rebase { addr: address + slide });
                        address = address.checked_add(skip + 8)
                            .ok_or("address overflow during DO_REBASE_ULEB_TIMES_SKIPPING_ULEB")?;
                    }
                }
                REBASE_OPCODE_DONE => { // 0x00
                    break;
                }

                _ => {
                    return Err(format!("Unknown rebase opcode 0x:{:02x}", opcode).into());
                }
            }
        }
        

        Ok(())
    }


    fn parse_bind(
        dyld_info: &DYLDInfoCommand,
        segments: &[ParsedSegment],
        symbols: &[ParsedSymbol],
        slide: u64, 
        data: &[u8], 
        memory: &MachOMemoryImage,
        fixups: &mut Vec<Fixup>
    ) -> Result<(), Box<dyn Error>> {

        let start = dyld_info.bind_off as usize;
        let end = start + dyld_info.bind_size as usize;
        let stream = &data[start..end];

        let mut cursor = 0;
        
        let mut segment_index: usize = 0;
        let mut segment_offset: u64 = 0;
        let mut symbol_name: Option<String> = None;
        let mut addend: i64 = 0;
        let mut bind_type: u8 = BIND_TYPE_POINTER; // 1
        let mut dylib_ordinal: i32 = 1;

        let mut threaded = false;
        let mut threaded_table_size: usize = 0;
        let mut threaded_bind_table: Vec<ThreadedBindEntry> = Vec::new();

        while cursor < stream.len() {
            let opcode = stream[cursor];
            cursor += 1;

            println!("cursor={} opcode=0x{:02x}", cursor-1, opcode);

            match opcode & BIND_OPCODE_MASK {

                BIND_OPCODE_SET_DYLIB_ORDINAL_IMM => {
                    // This opcode tells which dylib a symbol comes from
                    dylib_ordinal = (opcode & BIND_IMMEDIATE_MASK) as i32;
                }

                BIND_OPCODE_SET_DYLIB_ORDINAL_ULEB => {
                    // Ordinal is encoded as a ULEB128 immediately after the opcode
                    println!("cursor={} opcode=0x{:02x} segment_index={} segment_offset={} next bytes={:?}",cursor, opcode, segment_index, segment_offset, &stream[cursor..cursor+10.min(stream.len()-cursor)]);
                    dylib_ordinal = read_uleb(stream, &mut cursor)? as i32;
                }

                BIND_OPCODE_SET_DYLIB_SPECIAL_IMM => {
                    // The value is a 4 bit signed immediate:
                    // 0 --> self
                    // -1 --> MAIN_EXECUTABLE
                    // -2 --> FLAT_LOOKUP
                    // -3 --> WEAK_LOOKUP
                    let imm = (opcode & BIND_IMMEDIATE_MASK) as i8;
                    dylib_ordinal = match imm {
                        0 => BIND_SPECIAL_DYLIB_SELF,
                        1 => BIND_SPECIAL_DYLIB_MAIN_EXECUTABLE,
                        2 => BIND_SPECIAL_DYLIB_FLAT_LOOKUP,
                        3 => BIND_SPECIAL_DYLIB_WEAK_LOOKUP,
                        _ => return Err(format!("Invalid special dyld immediate {imm}").into())
                    };
                }

                BIND_OPCODE_SET_SYMBOL_TRAILING_FLAGS_IMM => {
                    let mut name_bytes: Vec<u8> = Vec::new();
                    while cursor < stream.len() && stream[cursor] != 0 {
                        name_bytes.push(stream[cursor]);
                        cursor += 1;
                    }
                    cursor += 1; // null byte terminating name
                    symbol_name = Some(String::from_utf8(name_bytes)?);
                }

                BIND_OPCODE_SET_TYPE_IMM => {
                    bind_type = opcode & BIND_IMMEDIATE_MASK; 
                }

                BIND_OPCODE_SET_ADDEND_SLEB => {
                    addend = read_sleb(stream, &mut cursor)?;
                }

                BIND_OPCODE_SET_SEGMENT_AND_OFFSET_ULEB => {
                    segment_index = (opcode & BIND_IMMEDIATE_MASK) as usize;
                    println!("cursor={} opcode=0x{:02x} segment_index={} segment_offset={} next bytes={:?}",cursor, opcode, segment_index, segment_offset, &stream[cursor..cursor+10.min(stream.len()-cursor)]);
                    segment_offset = read_uleb(stream, &mut cursor)?;
                }

                BIND_OPCODE_ADD_ADDR_ULEB => {
                    println!("cursor={} bytes={:?}", cursor, &stream[cursor..cursor+10.min(stream.len()-cursor)]);
                    let raw = read_uleb(stream, &mut cursor)?;
                    let delta = raw as i64; // interpret as signed
                    println!("ADD_ADDR_ULEB: segment_index={}. delta={}, cursor={}", segment_index, delta, cursor);
                    segment_offset = (segment_offset as i64)
                        .checked_add(delta)
                        .ok_or("segment_offset overflow during ADD_ADDR_ULEB")? as u64;
                }

                BIND_OPCODE_DO_BIND => {
                    let seg = &segments[segment_index];
                    let addr = seg.vmaddr + segment_offset;

                    if threaded {
                        if threaded_bind_table.len() >= threaded_table_size {
                            return Err("Threaded bind table overflow".into());
                        }

                        threaded_bind_table.push(ThreadedBindEntry {
                            dylib_ordinal,
                            symbol_name: symbol_name.clone().unwrap_or_default(),
                            bind_type,
                            addend,
                        });
                    } else {
                        if let Some(name) = &symbol_name {
                            fixups.push(Fixup::Bind {
                                addr: addr + slide,
                                symbol: name.clone(),
                                addend,
                            });
                        }
                    }

                    segment_offset += 8;
                }

                BIND_OPCODE_DO_BIND_ADD_ADDR_ULEB => {
                    println!("cursor={} opcode=0x{:02x} segment_index={} segment_offset={} next bytes={:?}",cursor, opcode, segment_index, segment_offset, &stream[cursor..cursor+10.min(stream.len()-cursor)]);
                    let skip = read_uleb(stream, &mut cursor)?;
                    let seg = &segments[segment_index];
                    let addr = seg.vmaddr + segment_offset;

                    if let Some(name) = &symbol_name {
                        fixups.push(Fixup::Bind {
                            addr: addr + slide,
                            symbol: name.clone(),
                            addend,
                        });
                    }

                    segment_offset += 8 + skip;
                }

                BIND_OPCODE_DO_BIND_ADD_ADDR_IMM_SCALED => {
                    let skip = (opcode & BIND_IMMEDIATE_MASK) as u64 * 8;
                    let seg = &segments[segment_index];
                    let addr = seg.vmaddr + segment_offset;

                    if let Some(name) = &symbol_name {
                        fixups.push(Fixup::Bind {
                            addr: addr + slide,
                            symbol: name.clone(),
                            addend,
                        });
                    }

                    segment_offset += 8 + skip;
                }

                BIND_OPCODE_DO_BIND_ULEB_TIMES_SKIPPING_ULEB => {
                    println!("cursor={} opcode=0x{:02x} segment_index={} segment_offset={} next bytes={:?}",cursor, opcode, segment_index, segment_offset, &stream[cursor..cursor+10.min(stream.len()-cursor)]);
                    let count = read_uleb(stream, &mut cursor)?;
                    println!("cursor={} opcode=0x{:02x} segment_index={} segment_offset={} next bytes={:?}",cursor, opcode, segment_index, segment_offset, &stream[cursor..cursor+10.min(stream.len()-cursor)]);
                    let skip = read_uleb(stream, &mut cursor)?;

                    let seg = &segments[segment_index];
                    let mut addr = seg.vmaddr + segment_offset;

                    for _ in 0..count {
                        if let Some(name) = &symbol_name {
                            fixups.push(Fixup::Bind {
                                addr: addr + slide,
                                symbol: name.clone(),
                                addend,
                            });
                        }
                        addr += 8 + skip;
                    }

                    segment_offset = addr - seg.vmaddr;
                }

                BIND_OPCODE_THREADED => {
                    // what is threaded bind:
                    //  instead of encoding every DO_BIND, it sets up a threaded chain of pointers that dyld can follow
                    //  not to be confused with multithreading 

                    // 0xD0 (BIND_OPCODE_THREADED) is a prefix opcode that switches dyld into "threaded bind mode"
                    // sub opcodes:
                    //      BIND_SUBOPCODE_THREADED_SET_BIND_ORDINAL_TABLE_SIZE_ULEB = 0x00
                    //          sets the size of the bind ordinal table
                    //      BIND_SUBOPCODE_THREADED_APPLY = 0x01
                    //          applies the threaded binds using the table
                    // lower nibble (retrieved via BIND_IMMEDIATE_MASK) indicates thread type
                    //      1 --> threaded by seg/offset
                    //      2 --> threaded by absolute pointer

                    threaded = true;

                    let subopcode = opcode & BIND_IMMEDIATE_MASK;

                    match subopcode {

                        BIND_SUBOPCODE_THREADED_SET_BIND_ORDINAL_TABLE_SIZE_ULEB => {
                            println!("cursor={} opcode=0x{:02x} segment_index={} segment_offset={} next bytes={:?}",cursor, opcode, segment_index, segment_offset, &stream[cursor..cursor+10.min(stream.len()-cursor)]);
                            let size = read_uleb(stream, &mut cursor)? as usize;
                            threaded_table_size = size;
                            threaded_bind_table = Vec::with_capacity(size);
                        }

                        BIND_SUBOPCODE_THREADED_APPLY => {
                            let seg = &segments[segment_index];
                            let mut addr = seg.vmaddr + segment_offset;

                            // Each pointer in memory contains the bind ordinal index and next pointer delta
                            // bits 0..15   --> ordinal index
                            // bits 16..31  --> next delta in 8 byte units
                            // rest         --> flags

                            loop {
                                let raw = memory.read_u64(addr)
                                    .ok_or("Invalid VM read during threaded bind")?;

                                // non arm64 layout, no ptr auth for right now
                                let ordinal_index = (raw & 0xFFFF) as usize;
                                let delta = ((raw >> 16) & 0xFFFF) as u64;

                                if ordinal_index >= threaded_bind_table.len() {
                                    return Err("Threaded ordinal out of bounds".into());
                                }

                                let entry = &threaded_bind_table[ordinal_index];

                                fixups.push(Fixup::Bind {
                                    addr: addr + slide,
                                    symbol: entry.symbol_name.clone(),
                                    addend: entry.addend,
                                });

                                if delta == 0 {
                                    break;
                                }

                                addr += delta * 8;
                            }

                            segment_offset = addr - seg.vmaddr;
                        }

                        _ => return Err("Invalid threaded subopcode".into())
                    }
                }

                BIND_OPCODE_DONE => break,

                _ => {
                    return Err(format!("Unknown bind opcode 0x:{:02x}", opcode).into());
                }
            }
        }

        Ok(())
    }
}


pub fn print_fixups_summary(fixups: &[Fixup]) {
    if fixups.is_empty() {
        return;
    }

    println!();
    println!("{}", "Fixups".green().bold());
    println!("--------------------------------------------------------------------------------");
    println!(
        "{:<18} {:<12} {:<30} {:<12}",
        "Address", "Type", "Symbol", "Addend"
    );
    println!("--------------------------------------------------------------------------------");

    for f in fixups {
        match f {
            Fixup::Rebase { addr } => {
                let addr_str = format!("0x{:016x}", addr);
                let kind = "rebase".yellow();
                println!("{:<18} {:<12} {:<30} {:<12}", addr_str, kind, "", "");
            }
            Fixup::Bind { addr, symbol, addend } => {
                let addr_str = format!("0x{:016x}", addr);
                let kind = "bind".yellow();
                let sym = symbol.magenta();
                let add = format!("{}", addend).cyan();
                println!("{:<18} {:<12} {:<30} {:<12}", addr_str, kind, sym, add);
            }
            Fixup::WeakBind { addr, symbol, addend } => {
                let addr_str = format!("0x{:016x}", addr);
                let kind = "weak_bind".yellow();
                let sym = symbol.magenta();
                let add = format!("{}", addend).cyan();
                println!("{:<18} {:<12} {:<30} {:<12}", addr_str, kind, sym, add);
            }
            Fixup::LazyBind { addr, symbol, addend } => {
                let addr_str = format!("0x{:016x}", addr);
                let kind = "lazy_bind".yellow();
                let sym = symbol.magenta();
                let add = format!("{}", addend).cyan();
                println!("{:<18} {:<12} {:<30} {:<12}", addr_str, kind, sym, add);
            }
        }
    }

    println!("--------------------------------------------------------------------------------");
}