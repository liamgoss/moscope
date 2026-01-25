use std::fs;
use std::path::Path;

use moscope::macho::fat::{FatArch, FatKind, read_fat_archs, read_fat_header};
use moscope::macho::header::{MachHeader32, MachHeader64, MachOHeader, MachOSlice, read_thin_header};
use moscope::macho::load_commands::{LoadCommand, read_load_commands};
use moscope::macho::constants::{
    cpu_type_name,
    cpu_subtype_name,
    filetype_name,
    MH_EXECUTE, 
};



/*
===============================
======== Thin Binaries ========
=============================== 
*/

#[test]
fn parses_thin_arm64_binary() {
    let path = Path::new("tests/samples/hello_arm64");
    let data = fs::read(path).expect("failed to read hello_arm64");

    // Thin binaries should NOT parse as fat
    let header = read_fat_header(&data);
    assert!(header.is_err(), "thin binary misclassified as fat");
}

#[test]
fn parses_thin_x86_64_binary() {
    let path = Path::new("tests/samples/hello_x86_64");
    let data = fs::read(path).expect("failed to read hello_x86_64");

    let header = read_fat_header(&data);
    assert!(header.is_err(), "thin binary misclassified as fat");
}

/*
========================================
======== Fat/Universal Binaries ========
======================================== 
*/

#[test]
fn parses_fat_binary_header() {
    let path = Path::new("tests/samples/hello_fat");
    let data = fs::read(path).expect("failed to read hello_fat");

    let header = read_fat_header(&data)
        .expect("failed to parse fat header");

    assert_eq!(header.nfat_arch, 2);
    assert!(
        matches!(header.kind, FatKind::Fat32BE | FatKind::Fat64BE),
        "unexpected fat kind: {:?}",
        header.kind
    );
}

#[test]
fn parses_fat_binary_archs() {
    let path = Path::new("tests/samples/hello_fat");
    let data = fs::read(path).expect("failed to read hello_fat");

    let header = read_fat_header(&data).unwrap();
    let archs = read_fat_archs(&data, &header)
        .expect("failed to parse fat archs");

    assert_eq!(archs.len(), 2);
}

#[test]
fn fat_binary_cpu_types_and_subtypes() {
    let path = Path::new("tests/samples/hello_fat");
    let data = fs::read(path).expect("failed to read hello_fat");

    let header = read_fat_header(&data).unwrap();
    let archs = read_fat_archs(&data, &header).unwrap();

    let mut has_seen_arm = false;
    let mut has_seen_x86 = false;

    for arch in archs {

        let (cputype, cpusubtype) = match arch {
            FatArch::Arch32(a) => (a.cputype, a.cpusubtype),
            FatArch::Arch64(a) => (a.cputype, a.cpusubtype),
        };

        println!("cputype is {:?} and cpusubtype is {:?}", cpusubtype, cpusubtype);
        let cpu = cpu_type_name(cputype);
        let subtype = cpu_subtype_name(cputype, cpusubtype);

            match cpu {
                "ARM" => {
                    assert!(
                        subtype.starts_with("arm64"),
                        "unexpected ARM subtype: {}", subtype
                    );
                    has_seen_arm = true;
                } 
                "x86" => {
                    assert!(subtype.starts_with("x86_64"),
                    "unexpected x86 subtype: {}", subtype
                    );
                    has_seen_x86 = true;
                }

                other => panic!("Unexpected CPU type: {}", other),
            }
    }

    assert!(has_seen_arm, "arm64 slice was not detected");
    assert!(has_seen_x86, "x86 slice was not detected");
}


#[test]
fn fat_binary_filetype_is_execute() {
    let path = Path::new("tests/samples/hello_fat");
    let data = fs::read(path).expect("failed to read hello_fat");

    let header = read_fat_header(&data).unwrap();
    let archs = read_fat_archs(&data, &header).unwrap();

    for current_arch in archs {

        let slice: MachOSlice = match current_arch {
            FatArch::Arch64(a) => MachOSlice {
                offset: a.offset, // u64 offset
                size: Some(a.size),
            },

            /*
            You may think that the below match arm is unreachable...but alas it's not
            This is matching the fat table, not the slices inside the fat
            So the slices / binaries in the fat may be 64 bit but the 
                fat table can be 32 bit because Apple defaults the fat table
                to 32 bit unless the offsets exceed 4GB

            fat header
            ├─ fat_arch (32-bit offsets) (<-- by default, based on offsets)
            │   ├─ Mach-O 64-bit slice (arm64)
            │   └─ Mach-O 64-bit slice (x86_64)
             */
            FatArch::Arch32(a) => MachOSlice { 
                offset: a.offset as u64, 
                size: Some(a.size as u64),
            },
        };

        let macho = read_thin_header(&data, &slice).expect("Failed to read Mach-O header");

        let filetype = match macho.header {
            MachOHeader::Header64(h) => h.filetype,
            _ => unreachable!(), // The sample binary is 64 bit, but Rust demands completeness so we can use this cool macro I just learned about
            // https://web.archive.org/web/20251112150750/https://doc.rust-lang.org/std/macro.unreachable.html 
        };

        assert_eq!(filetype, MH_EXECUTE);
        
    }
}


#[test]
fn fat_binary_has_load_commands() {
    let path = Path::new("tests/samples/hello_fat");
    let data = fs::read(path).expect("failed to read hello_fat");

    let header = read_fat_header(&data).unwrap();
    let archs = read_fat_archs(&data, &header).unwrap();

    for current_arch in archs {

        let slice: MachOSlice = match current_arch {
            FatArch::Arch64(a) => MachOSlice {
                offset: a.offset, // u64 offset
                size: Some(a.size),
            },

            FatArch::Arch32(a) => MachOSlice { 
                offset: a.offset as u64, 
                size: Some(a.size as u64),
            },
        };

        let macho = read_thin_header(&data, &slice).expect("Failed to read Mach-O header");

        let (header_size, ncmds, word_size, is_be) = match &macho.header {
            MachOHeader::Header32(h) => (
                std::mem::size_of::<MachHeader32>(),
                h.ncmds,
                32,
                macho.kind.is_be(),
            ),
            MachOHeader::Header64(h) => (
                std::mem::size_of::<MachHeader64>(),
                h.ncmds,
                64,
                macho.kind.is_be(),
            ),
        };

        let load_command_offset = slice.offset as usize + header_size;
        let load_commands = read_load_commands(&data, load_command_offset as u32, ncmds, word_size, is_be).unwrap();
        assert!(!load_commands.is_empty(), "No load commands found");
        assert_eq!(ncmds, load_commands.len() as u32);
            
    }
}

