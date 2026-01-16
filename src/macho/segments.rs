// File Purpose: Enumerate Segments, Work with sections.rs

use std::error::Error;
use crate::macho::sections::*;
use crate::macho::utils;
use colored::Colorize;

// https://web.archive.org/web/20260107202245/https://developer.apple.com/library/archive/documentation/Performance/Conceptual/CodeFootprint/Articles/MachOOverview.html
// https://web.archive.org/web/20250912084041/https://medium.com/@travmath/understanding-the-mach-o-file-format-66cf0354e3f4
// https://github.com/aidansteele/osx-abi-macho-file-format-reference/blob/master/README.md#table-1-the-sections-of-a__textsegment

// NOTE: I have read through the above 3 resources and compiled what I believe to be the most important ones to know
/*
=======================================
==== Notable Segments and Sections ====
=======================================


__TEXT (Read + Execute)
    Executable code and read-only data. Typically shared across processes.

    __text
        Compiled machine instructions.

    __const
        Read-only constant data that does not require relocation.

    __cstring
        Null-terminated C string literals.
        Duplicate strings are typically coalesced by the linker.

    __stubs
        Small trampoline functions used for calling dynamically
        linked functions. Each stub typically jumps through a
        corresponding symbol pointer.

    __stub_helper
        Helper code used by dyld to resolve lazy symbols at runtime.

    __picsymbol_stub (legacy / transitional)
        Position-independent symbol stubs used by older toolchains.
        Largely superseded by __stubs / __stub_helper in modern binaries.


__DATA (Read + Write)
    Mutable data sections mapped into writable memory.

    __data
        Initialized global and static variables
        (e.g., `int a = 1;`, `static int b = 2;`).

    __const
        Constant data that requires relocation
        (e.g., `char * const p = "foo";`).

    __bss
        Zero-initialized globals and statics.
        Occupies virtual memory but has no backing bytes in the file.

    __common (legacy)
        Uninitialized external globals.
        Largely folded into __bss by modern toolchains.

    __la_symbol_ptr
        Lazy symbol pointers used for functions.
        Initially unresolved and fixed up by dyld on first call.

    __nl_symbol_ptr
        Non-lazy symbol pointers.
        Resolved by dyld at load time.

    __dyld
        Reserved section used internally by the dynamic linker.


__PAGEZERO
    - Unmapped region starting at virtual address 0
    - No read/write/execute permissions
    - Size is typically one page or more
    - Occupies no space in the file
    - Used to trap NULL pointer dereferences
*/


// From /Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX26.0.sdk/usr/include/mach-o/loader.h
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SegmentCommand {   // For 32-bit architectures
    pub cmd: u32,               // LC_SEGMENT
    pub cmdsize: u32,           // includes sizeof section_64 structs
    pub segname: [u8; 16],      // segment name
    pub vmaddr: u32,            // memory address of this segment
    pub vmsize: u32,            // memory size of this segment
    pub fileoff: u32,           // file offset of this segment
    pub filesize: u32,          // amount to map from the file
    pub maxprot: i32,           // maximum VM protection
    pub initprot: i32,          // initial VM protection
    pub nsects: u32,            // number of sections in segment
    pub flags: u32,             // flags
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SegmentCommand64 {   // For 64-bit architectures
    pub cmd: u32,               // LC_SEGMENT_64
    pub cmdsize: u32,           // includes sizeof section_64 structs
    pub segname: [u8; 16],      // segment name
    pub vmaddr: u64,            // memory address of this segment
    pub vmsize: u64,            // memory size of this segment
    pub fileoff: u64,           // file offset of this segment
    pub filesize: u64,          // amount to map from the file
    pub maxprot: i32,           // maximum VM protection
    pub initprot: i32,          // initial VM protection
    pub nsects: u32,            // number of sections in segment
    pub flags: u32,             // flags
}

pub struct ParsedSegment {
    pub segname: [u8; 16],      
    pub vmaddr: u64,   
    pub vmsize: u64,   
    pub fileoff: u64,  
    pub filesize: u64, 
    pub maxprot: i32,  
    pub initprot: i32, 
    // pub nsects: u32,   // redundant, just use sections.len()
    pub flags: u32,    
    pub sections: Vec<ParsedSection>,
}

// The layout in the binary, (I believe) is:
/*
|   segment_command_64  |
|       section_64      |
|       section_64      |
|       section_64      |
|  .... up to `nsects`  |
*/



pub fn parse_segment_32(data: &[u8], offset: usize, is_be: bool) -> Result<ParsedSegment, Box<dyn Error>> {
    use std::mem::size_of;
    if offset + size_of::<SegmentCommand>() > data.len() {
        return Err("Segment command out of bounds".into());
    }
    // start at offset + 8 because segname starts after cmd and cmdsize which are each u32
    let segname: [u8; 16] = data[offset + 8 .. offset + 24].try_into()?;
    let vmaddr_32: u32   = utils::bytes_to(is_be, &data[offset + 24 ..])?;
    let vmsize_32: u32   = utils::bytes_to(is_be, &data[offset + 28 ..])?;
    let fileoff_32: u32  = utils::bytes_to(is_be, &data[offset + 32 ..])?;
    let filesize_32: u32 = utils::bytes_to(is_be, &data[offset + 36 ..])?;
    let maxprot: i32  = utils::bytes_to(is_be, &data[offset + 40 ..])?;
    let initprot: i32 = utils::bytes_to(is_be, &data[offset + 44 ..])?;
    let nsects: u32   = utils::bytes_to(is_be, &data[offset + 48 ..])?;
    let flags: u32    = utils::bytes_to(is_be, &data[offset +  52..])?;

    let vmaddr = vmaddr_32 as u64;
    let vmsize = vmsize_32 as u64;
    let fileoff = fileoff_32 as u64;
    let filesize = filesize_32 as u64;

    // Now we have to parse the sections in this segment
    let mut sections = Vec::with_capacity(nsects as usize);
    let mut sect_offset = offset + size_of::<SegmentCommand>();
    for _ in 0..nsects {
        sections.push(read_section32_from_bytes(&data, is_be, sect_offset)?);
        sect_offset += size_of::<Section>();
    }
    //Ok(ParsedSegment { segname, vmaddr, vmsize, fileoff, filesize, maxprot, initprot, nsects, flags, sections })
    Ok(ParsedSegment { segname, vmaddr, vmsize, fileoff, filesize, maxprot, initprot, flags, sections })
}


pub fn parse_segment_64(data: &[u8], offset: usize, is_be: bool) -> Result<ParsedSegment, Box<dyn Error>> {
    use std::mem::size_of;
    if offset + size_of::<SegmentCommand64>() > data.len() {
        return Err("Segment command out of bounds".into());
    }
    // start at offset + 8 because segname starts after cmd and cmdsize which are each u32
    let segname: [u8; 16] = data[offset + 8 .. offset + 24].try_into()?;
    let vmaddr: u64   = utils::bytes_to(is_be, &data[offset + 24 ..])?;
    let vmsize: u64   = utils::bytes_to(is_be, &data[offset + 32 ..])?;
    let fileoff: u64  = utils::bytes_to(is_be, &data[offset + 40 ..])?;
    let filesize: u64 = utils::bytes_to(is_be, &data[offset + 48 ..])?;
    let maxprot: i32  = utils::bytes_to(is_be, &data[offset + 56 ..])?;
    let initprot: i32 = utils::bytes_to(is_be, &data[offset + 60 ..])?;
    let nsects: u32   = utils::bytes_to(is_be, &data[offset + 64 ..])?;
    let flags: u32    = utils::bytes_to(is_be, &data[offset + 68 ..])?;

    // Now we have to parse the sections in this segment
    let mut sections = Vec::with_capacity(nsects as usize);
    let mut sect_offset = offset + size_of::<SegmentCommand64>();
    for _ in 0..nsects {
        sections.push(read_section64_from_bytes(&data, is_be, sect_offset)?);
        sect_offset += size_of::<Section64>();
    }
    //Ok(ParsedSegment { segname, vmaddr, vmsize, fileoff, filesize, maxprot, initprot, nsects, flags, sections })
    Ok(ParsedSegment { segname, vmaddr, vmsize, fileoff, filesize, maxprot, initprot, flags, sections })
}


pub fn print_segments_summary(segments: &Vec<ParsedSegment>) {
    println!();
    println!("{}", "Segments Summary".green().bold());
    println!("----------------------------------------");

    for seg in segments {
        let seg_name = utils::byte_array_to_string(&seg.segname);

        let vm_start = seg.vmaddr;
        let vm_end   = seg.vmaddr + seg.vmsize;

        let file_start = seg.fileoff;
        let file_end   = seg.fileoff + seg.filesize;
        // rwx is just binary, mask it out below and we can apply coloring to them later if desired (I tried, it's hard to read at the moment)
        // 001 is r -> 1
        // 010 is w -> 2
        // 100 is x -> 4
        
        let prot_r = if seg.initprot & 0x1 != 0 { "R" } else { "-".into() }; 
        let prot_w = if seg.initprot & 0x2 != 0 { "W" } else { "-".into() };
        let prot_x = if seg.initprot & 0x4 != 0 { "X" } else { "-".into() };

        println!();
        println!("{} {}", "Segment".yellow().bold(), seg_name.green().bold());

        println!("{} 0x{:016x} - 0x{:016x} ({:#x} bytes)", "  VM range   :".yellow().bold(), vm_start, vm_end, seg.vmsize);

        println!("{} 0x{:08x} - 0x{:08x} ({:#x} bytes)", "  File range :".yellow().bold(), file_start, file_end, seg.filesize);

        println!("{} {}{}{}", "  Protections:".yellow().bold(), prot_r, prot_w, prot_x);

        println!("{} {}", "  Sections   :".yellow().bold(), seg.sections.len());

        for sect in &seg.sections {
            let sect_name = utils::byte_array_to_string(&sect.sectname);

            let kind_colored = match sect.kind {
                // "Top" ones to focus on coloring
                SectionKind::Code          => format!("{:?}", sect.kind).blue().bold().to_string(),
                SectionKind::Data          => format!("{:?}", sect.kind).blue().bold().to_string(),
                SectionKind::ConstData     => format!("{:?}", sect.kind).green().bold().to_string(),
                SectionKind::CString       => format!("{:?}", sect.kind).green().bold().to_string(),
                SectionKind::Bss           => format!("{:?}", sect.kind).blue().bold().to_string(),

                // Linker / runtime stuffs
                SectionKind::Stub          => format!("{:?}", sect.kind).yellow().bold().to_string(),
                SectionKind::SymbolPointer => format!("{:?}", sect.kind).cyan().bold().to_string(),
                SectionKind::LinkEdit      => format!("{:?}", sect.kind).magenta().bold().to_string(),

                // ObjC + metadata 
                SectionKind::ObjC          => format!("{:?}", sect.kind).green().bold().to_string(),
                SectionKind::ObjCMetadata  => format!("{:?}", sect.kind).green().to_string(),

                // Control-flow and runtime support
                SectionKind::Init          => format!("{:?}", sect.kind).yellow().bold().to_string(),
                SectionKind::Exception     => format!("{:?}", sect.kind).yellow().to_string(),
                SectionKind::Unwind        => format!("{:?}", sect.kind).yellow().to_string(),

                // Debug, Other
                SectionKind::Debug         => format!("{:?}", sect.kind),
                SectionKind::Other         => format!("{:?}", sect.kind),

                // This should stand out, we don't know what it is --> we can add it later if commonly seen
                SectionKind::Unknown       => format!("{:?}", sect.kind).red().bold().to_string(),
            };


            println!("    - {:<16} {:<14} size={:#x}", sect_name, kind_colored, sect.size);
        }
    }

    println!("----------------------------------------");
    println!();
}