use std::error::Error;
use super::constants;

// File Purpose: "what kind of Mach-O file is this?"
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
    println!();
    println!("Mach-O Header Summary");
    println!("----------------------------------------");
    println!("  Magic        : 0x{:08x}", magic);
    println!("  Architecture : {} ({})",
        constants::cpu_type_name(cputype),
        constants::cpu_subtype_name(cputype, cpusubtype),
    );
    println!("  Word size    : {}-bit", bits);
    println!("  File type    : {}", constants::filetype_name(filetype));
    println!("  Load cmds    : {}", ncmds);
    println!("  Cmds size    : {} bytes", sizeofcmds);
    println!("  Flags        : 0x{:08x}", flags);
    println!("----------------------------------------");
    println!();
}




pub fn read_thin_header(data: &[u8], slice: MachOSlice) -> Result<MachOHeader, Box<dyn Error>> {
    use std::mem::size_of;

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

        Ok(header)
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
        Ok(header)
    }
}
