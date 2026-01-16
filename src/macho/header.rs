// File Purpose: "what kind of Mach-O file is this?"
use std::error::Error;
use super::constants::*;

use super::utils;
use super::constants;
use colorize::AnsiColor;



/*
Mach-O Header
+----------------+      +---------------+
| mach_header_64 | -->  | Magic Number  |
+----------------+      +---------------+
| load commands  |      |   CPU Type    |
+----------------+      +---------------+
| segments       |      |  CPU Subtype  |
+----------------+      +---------------+
                        |   File Type   |
                        +---------------+
                        | Num Load Cmds |
                        +---------------+
                        | Size of LC's  |
                        +---------------+
                        |     Flags     |
                        +---------------+
                        |    Reserved   |
                        +---------------+

*/

pub struct MachOSlice {
    pub offset: u64, // Where this Mach-O binary begins
    pub size: Option<u64>, // how large is the Mach-O (only really important for fat)
}

pub struct MachOFlag {
    pub mask: u32,
    pub name: &'static str,
}


pub const MACHO_FLAGS: &[MachOFlag] = &[
    MachOFlag { mask: MH_NOUNDEFS, name: "NOUNDEFS" },
    MachOFlag { mask: MH_INCRLINK, name: "INCRLINK" },
    MachOFlag { mask: MH_DYLDLINK, name: "DYLDLINK" },
    MachOFlag { mask: MH_BINDATLOAD, name: "BINDATLOAD" },
    MachOFlag { mask: MH_PREBOUND, name: "PREBOUND" },
    MachOFlag { mask: MH_SPLIT_SEGS, name: "SPLIT_SEGS" },
    MachOFlag { mask: MH_LAZY_INIT, name: "LAZY_INIT" },
    MachOFlag { mask: MH_TWOLEVEL, name: "TWOLEVEL" },
    MachOFlag { mask: MH_FORCE_FLAT, name: "FORCE_FLAT" },
    MachOFlag { mask: MH_NOMULTIDEFS, name: "NOMULTIDEFS" },
    MachOFlag { mask: MH_NOFIXPREBINDING, name: "NOFIXPREBINDING" },
    MachOFlag { mask: MH_PREBINDABLE, name: "PREBINDABLE" },
    MachOFlag { mask: MH_ALLMODSBOUND, name: "ALLMODSBOUND" },
    MachOFlag { mask: MH_SUBSECTIONS_VIA_SYMBOLS, name: "SUBSECTIONS_VIA_SYMBOLS" },
    MachOFlag { mask: MH_CANONICAL, name: "CANONICAL" },
    MachOFlag { mask: MH_WEAK_DEFINES, name: "WEAK_DEFINES" },
    MachOFlag { mask: MH_BINDS_TO_WEAK, name: "BINDS_TO_WEAK" },
    MachOFlag { mask: MH_ALLOW_STACK_EXECUTION, name: "ALLOW_STACK_EXECUTION" },
    MachOFlag { mask: MH_ROOT_SAFE, name: "ROOT_SAFE" },
    MachOFlag { mask: MH_SETUID_SAFE, name: "SETUID_SAFE" },
    MachOFlag { mask: MH_NO_REEXPORTED_DYLIBS, name: "NO_REEXPORTED_DYLIBS" },
    MachOFlag { mask: MH_PIE, name: "PIE" },
    MachOFlag { mask: MH_DEAD_STRIPPABLE_DYLIB, name: "DEAD_STRIPPABLE_DYLIB" },
    MachOFlag { mask: MH_HAS_TLV_DESCRIPTORS, name: "HAS_TLV_DESCRIPTORS" },
    MachOFlag { mask: MH_NO_HEAP_EXECUTION, name: "NO_HEAP_EXECUTION" },
    MachOFlag { mask: MH_APP_EXTENSION_SAFE, name: "APP_EXTENSION_SAFE" },
    MachOFlag { mask: MH_NLIST_OUTOFSYNC_WITH_DYLDINFO, name: "NLIST_OUTOFSYNC_WITH_DYLDINFO" },
    MachOFlag { mask: MH_SIM_SUPPORT, name: "SIM_SUPPORT" },
    MachOFlag { mask: MH_IMPLICIT_PAGEZERO, name: "IMPLICIT_PAGEZERO" },
    MachOFlag { mask: MH_DYLIB_IN_CACHE, name: "DYLIB_IN_CACHE" },
];




#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MachHeader64 {
    pub magic: u32, // mach magic number identifier
    pub cputype: i32, // cpu specifier 
    pub cpusubtype: i32, // machine specifier
    pub filetype: u32, // type of file
    pub ncmds: u32, // number of load commands
    pub sizeofcmds: u32, // the size of all the load commands
    pub flags: u32, // flags
    pub reserved: u32 // reserved
}
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MachHeader32 {
    pub magic: u32, // mach magic number identifier
    pub cputype: i32, // cpu specifier 
    pub cpusubtype: i32, // machine specifier
    pub filetype: u32, // type of file
    pub ncmds: u32, // number of load commands
    pub sizeofcmds: u32, // the size of all the load commands
    pub flags: u32, // flags
    
}

#[repr(C)]
#[derive(Debug)]
pub enum MachOHeader {
    Header32(MachHeader32),
    Header64(MachHeader64),

}
#[derive(Debug)]
pub struct ParsedMachOHeader {
    pub kind: MachOKind,
    pub header: MachOHeader,
}




#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MachOKind {
    Mach32BE,
    Mach32LE,
    Mach64BE,
    Mach64LE,
}

impl MachOKind {
    pub fn is_64(self) -> bool {
        matches!(self, MachOKind::Mach64BE | MachOKind::Mach64LE)
    }

    pub fn is_be(self) -> bool {
        matches!(self, MachOKind::Mach32BE | MachOKind::Mach64BE)
    }
}



pub fn print_header_summary(header: &MachOHeader) {
    match header {
        MachOHeader::Header32(h) => {
            print_common_header(32, h.magic, h.cputype, h.cpusubtype, h.filetype, h.ncmds, h.sizeofcmds, h.flags,);
        }
        MachOHeader::Header64(h) => {
            print_common_header(64, h.magic, h.cputype, h.cpusubtype, h.filetype, h.ncmds, h.sizeofcmds, h.flags,);
        }
    }
}

fn parse_flags(flags: u32) -> Vec<&'static str> {
    // This took quite some time to figure out the best way to do it
    // I mean I could have done a for loop with masking against all MACH_FLAGs but this is 1) more concise and 2) much cooler
    MACHO_FLAGS.iter().filter(|f| flags & f.mask != 0).map(|f| f.name).collect()

    // Dear Reader, I'm still learning Rust, so I try to incorporate new things as I learn them and believe they are reasonably applicable.
    // This comment is to help break this down for my own sanity
    // .iter()                          --> get an iterator over references to each element in the slice
    // .filter(|f| flags & f.mask != 0) --> keep only the elements where this condition is true
    // .map(|f| f.name)                 --> transforms each remaining item from &MachOFlag into &'static str
    // .collect()                       --> consume iterator and turn it into a collection (which I believe Rust infers into our return type of Vec<&'static str>)
}

fn print_common_header(
    bits: u32,
    magic: u32,
    cputype: i32,
    cpusubtype: i32,
    filetype: u32,
    ncmds: u32,
    sizeofcmds: u32,
    flags: u32,
) {
    let named_flags = parse_flags(flags);
    println!();
    println!("{}", "Mach-O Header Summary".b_green());
    println!("----------------------------------------");

    println!("{} 0x{:08x}", "  Magic        :".b_yellow(), magic);

    println!(
        "{} {} ({})",
        "  Architecture :".b_yellow(),
        constants::cpu_type_name(cputype),
        constants::cpu_subtype_name(cputype, cpusubtype),
    );

    println!("{} {}-bit", "  Word size    :".b_yellow(), bits);
    println!("{} {}", "  File type    :".b_yellow(), constants::filetype_name(filetype));
    println!("{} {}", "  Load cmds    :".b_yellow(), ncmds);
    println!("{} {} bytes", "  Cmds size    :".b_yellow(), sizeofcmds);
    println!("{} {}", "  Flags        :".b_yellow(), named_flags.join(", "));
    println!("----------------------------------------");
    println!();
}





pub fn read_thin_header(data: &[u8], slice: &MachOSlice) -> Result<ParsedMachOHeader, Box<dyn Error>> {

    let base = slice.offset as usize;

    if base + constants::MACH_HEADER32_SIZE /* base + 28 */ > data.len() { 
        return Err("File too small for Mach-O header".into());
    }

    fn classify_macho_magic(bytes: [u8; 4]) -> Option<MachOKind> {
        //println!("Attempting to match magic of {:?}", bytes);
        //println!("Valid matches:\n1. {:?}\n2. {:?}\n3. {:?}\n4. {:?}\n", constants::MH_MAGIC, constants::MH_MAGIC_64, constants::MH_CIGAM, constants::MH_CIGAM_64);
        match bytes {
            constants::MH_MAGIC     => Some(MachOKind::Mach32BE),
            constants::MH_MAGIC_64  => Some(MachOKind::Mach64BE),
            constants::MH_CIGAM     => Some(MachOKind::Mach32LE),
            constants::MH_CIGAM_64  => Some(MachOKind::Mach64LE),
            _ => None,
        }
    }

    let raw_magic_bytes: [u8; 4] = data[base..base + 4].try_into()?;

    let kind:MachOKind = match classify_macho_magic(raw_magic_bytes) {
        Some(kind) => kind,
        None => return Err("Not a valid Mach-O binary".into()),
    };

    if kind.is_64() {
        // Mach-O 64 Bit
        // bounds check
        if base + constants::MACH_HEADER64_SIZE > data.len() {
            return Err("File too small for Mach-O header 64-bit".into());
        } 

        let header64 = MachHeader64 {
            magic: utils::bytes_to(kind.is_be(), &data[base + 0..])?,
            cputype: utils::bytes_to(kind.is_be(), &data[base + 4..])?,
            cpusubtype: utils::bytes_to(kind.is_be(), &data[base + 8..])?,
            filetype: utils::bytes_to(kind.is_be(), &data[base + 12..])?,
            ncmds: utils::bytes_to(kind.is_be(), &data[base + 16..])?,
            sizeofcmds: utils::bytes_to(kind.is_be(), &data[base + 20..])?,
            flags: utils::bytes_to(kind.is_be(), &data[base + 24..])?,
            reserved: utils::bytes_to(kind.is_be(), &data[base + 38..])?,
        };

        let header = MachOHeader::Header64(header64);
        print_header_summary(&header);

        Ok(ParsedMachOHeader { kind, header })
    }    else {
        let header32 = MachHeader32 {
            magic: utils::bytes_to(kind.is_be(), &data[base + 0..])?,
            cputype: utils::bytes_to(kind.is_be(), &data[base + 4..])?,
            cpusubtype: utils::bytes_to(kind.is_be(), &data[base + 8..])?,
            filetype: utils::bytes_to(kind.is_be(), &data[base + 12..])?,
            ncmds: utils::bytes_to(kind.is_be(), &data[base + 16..])?,
            sizeofcmds: utils::bytes_to(kind.is_be(), &data[base + 20..])?,
            flags: utils::bytes_to(kind.is_be(), &data[base + 24..])?,
        };

        let header = MachOHeader::Header32(header32);
        print_header_summary(&header);
        Ok(ParsedMachOHeader { kind, header })
    }
}
