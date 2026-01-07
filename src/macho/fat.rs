// File Purpose: "Which Mach-O should be parsed?"

use std::error::Error;

use super::constants;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FatHeader {
    pub magic: u32,     // FAT_MAGIC or FAT_MAGIC_64 (numeric, after parsing)
    pub nfat_arch: u32, // number of structs that follow (host-native, after parsing)
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FatArch32 {
    pub cputype: i32,
    pub cpusubtype: i32,
    pub offset: u32,
    pub size: u32,
    pub align: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FatArch64 {
    pub cputype: i32,
    pub cpusubtype: i32,
    pub offset: u64,
    pub size: u64,
    pub align: u32,
    pub reserved: u32,
}

#[repr(C)]
#[derive(Debug)]
pub enum FatArch {
    Arch32(FatArch32),
    Arch64(FatArch64),
}

pub fn read_fat_archs(
    data: &[u8],          // Entire file contents
    header: &FatHeader,   // Previously-parsed fat header
    is_fat_header_64: bool,          // Whether this is a fat_arch_64
    needs_swap: bool,     // Whether we must swap big-endian-on-disk fields into host order
) -> Result<Vec<FatArch>, Box<dyn Error>> {
    let mut archs = Vec::new();
    let mut offset = std::mem::size_of::<FatHeader>();

    println!(
        "[fat] read_fat_archs: is_fat_header_64={}, needs_swap={}, nfat_arch={}",
        is_fat_header_64, needs_swap, header.nfat_arch
    );

    for i in 0..header.nfat_arch {
        if is_fat_header_64 {
            let bytes = &data[offset..offset + std::mem::size_of::<FatArch64>()];

            let mut arch: FatArch64 =
                unsafe { std::ptr::read_unaligned(bytes.as_ptr() as *const _) };

            if needs_swap {
                arch.cputype = i32::from_be(arch.cputype);
                arch.cpusubtype = i32::from_be(arch.cpusubtype);
                arch.offset = u64::from_be(arch.offset);
                arch.size = u64::from_be(arch.size);
                arch.align = u32::from_be(arch.align);
                arch.reserved = u32::from_be(arch.reserved);
            }

            println!(
                "[fat] arch[{i}] (64): cputype={}, cpusubtype={}, offset={}, size={}, align={}",
                arch.cputype, arch.cpusubtype, arch.offset, arch.size, arch.align
            );

            archs.push(FatArch::Arch64(arch));
            offset += std::mem::size_of::<FatArch64>();
        } else {
            let bytes = &data[offset..offset + std::mem::size_of::<FatArch32>()];

            let mut arch: FatArch32 =
                unsafe { std::ptr::read_unaligned(bytes.as_ptr() as *const _) };

            if needs_swap {
                arch.cputype = i32::from_be(arch.cputype);
                arch.cpusubtype = i32::from_be(arch.cpusubtype);
                arch.offset = u32::from_be(arch.offset);
                arch.size = u32::from_be(arch.size);
                arch.align = u32::from_be(arch.align);
            }

            println!(
                "[fat] arch[{i}] (32): cputype={}, cpusubtype={}, offset={}, size={}, align={}",
                arch.cputype, arch.cpusubtype, arch.offset, arch.size, arch.align
            );

            archs.push(FatArch::Arch32(arch));
            offset += std::mem::size_of::<FatArch32>();
        }
    }

    Ok(archs)
}

pub fn read_fat_header(data: &[u8]) -> Result<FatHeader, Box<dyn Error>> {
    use std::mem::size_of;

    if data.len() < size_of::<FatHeader>() {
        return Err("File too small to be a fat binary".into());
    }

    let raw_magic_bytes: [u8; 4] = data[0..4].try_into()?;
    let raw_nfat_arch_bytes: [u8; 4] = data[4..8].try_into()?;

    println!(
        "[fat] raw magic bytes = {:02x} {:02x} {:02x} {:02x}",
        raw_magic_bytes[0], raw_magic_bytes[1], raw_magic_bytes[2], raw_magic_bytes[3],
    );
    println!(
        "[fat] raw nfat_arch bytes = {:02x} {:02x} {:02x} {:02x}",
        raw_nfat_arch_bytes[0], raw_nfat_arch_bytes[1], raw_nfat_arch_bytes[2], raw_nfat_arch_bytes[3],
    );

    let is_fat = matches!(
        raw_magic_bytes,
        constants::FAT_MAGIC | constants::FAT_CIGAM | constants::FAT_MAGIC_64 | constants::FAT_CIGAM_64
    );
    if !is_fat {
        return Err("Not a fat Mach-O binary".into());
    }

    // FAT headers are big-endian on disk. So the correct interpretation for the numeric fields is BE.
    let magic: u32 = u32::from_be_bytes(data[0..4].try_into()?);
    let nfat_arch_be: u32 = u32::from_be_bytes(data[4..8].try_into()?);

    
    let nfat_arch_le_interpretation: u32 = u32::from_le_bytes(data[4..8].try_into()?);

    println!(
        "[fat] nfat_arch interpreted as BE = {}, as LE = {}",
        nfat_arch_be, nfat_arch_le_interpretation
    );

    Ok(FatHeader {
        magic,
        nfat_arch: nfat_arch_be,
    })
}
