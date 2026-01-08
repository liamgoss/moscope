// File Purpose: "Which Mach-O should be parsed?"

use std::error::Error;

use crate::macho::constants::{FAT_ARCH64_SIZE, FAT_MAGIC};

use super::constants;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FatHeader {
    pub kind: FatKind,    
    pub nfat_arch: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FatArch32 {
    pub cputype: i32, // Target CPU architecture
    pub cpusubtype: i32, // Specific CPU variant
    pub offset: u32, // File offset where the Mach-O binary begins
    pub size: u32, // Size (in bytes) of the Mach-O binary
    pub align: u32, // Power-of-two alignment of the Macho-O binary
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FatKind {
    Fat32BE,
    Fat32LE,
    Fat64BE,
    Fat64LE,
}

impl FatKind {
    pub fn is_64(self) -> bool {
        matches!(self, FatKind::Fat64BE | FatKind::Fat64LE)
    }

    pub fn is_be(self) -> bool {
        matches!(self, FatKind::Fat32BE | FatKind::Fat64BE)
    }

}


trait FromEndianBytes: Sized {
    const SIZE: usize;

    fn from_be(bytes: &[u8]) -> Result<Self, Box<dyn Error>>;
    fn from_le(bytes: &[u8]) -> Result<Self, Box<dyn Error>>;
}

impl FromEndianBytes for u32 {
    const SIZE: usize = 4;

    fn from_be(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(u32::from_be_bytes(bytes.try_into()?))
    }
    fn from_le(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(u32::from_le_bytes(bytes.try_into()?))
    }
}

impl FromEndianBytes for i32 {
    const SIZE: usize = 4;

    fn from_be(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(i32::from_be_bytes(bytes.try_into()?))
    }
    fn from_le(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(i32::from_le_bytes(bytes.try_into()?))
    }
}

impl FromEndianBytes for u64 {
    const SIZE: usize = 8;

    fn from_be(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(u64::from_be_bytes(bytes.try_into()?))
    }
    fn from_le(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(u64::from_le_bytes(bytes.try_into()?))
    }
}


/*  
    Instead of a ton of:
    
    let cputype_bytes: [u8; 4] = data[offset + 0 .. offset + 4].try_into()?;
    let cputype = if header.kind.is_be() {
        i32::from_be_bytes(cputype_bytes)
    } else {
        i32::from_le_bytes(cputype_bytes)
    };

    For each var and type, we can instead use the trait and implementations to save us the copy and paste hell
*/
fn bytes_to<T: FromEndianBytes>(kind: FatKind, data: &[u8]) -> Result<T, Box<dyn Error>> {
    if data.len() <T::SIZE {
        return Err("buffer too small".into());
    }
    if kind.is_be() {
        T::from_be(&data[..T::SIZE])
    } else {
        T::from_le(&data[..T::SIZE])
    }
}



pub fn read_fat_archs(
    data: &[u8],            // Entire file contents
    header: &FatHeader,     // Previously-parsed fat header
) -> Result<Vec<FatArch>, Box<dyn Error>> {
    let mut archs = Vec::new();
    let mut offset = constants::FAT_HEADER_SIZE; // Start on the on disk fat header


    for i in 0..header.nfat_arch {
        if header.kind.is_64() {
            // ==== fat_arch_64 ====
            // Bounds check
            if offset + constants::FAT_ARCH64_SIZE > data.len() {
                return Err(format!(
                    "fat_arch_64[{}] extends beyond EOF",
                    i
                ).into());
            }

            let base = offset;
            let cputype: i32 = bytes_to(header.kind, &data[base + 0..])?;
            let cpusubtype: i32 = bytes_to(header.kind, &data[base + 4..])?;
            let arch_offset: u64 = bytes_to(header.kind, &data[base + 8..])?;
            let size: u64 = bytes_to(header.kind, &data[base + 16..])?;
            let align: u32 = bytes_to(header.kind, &data[base + 24..])?;
            let reserved: u32 = bytes_to(header.kind, &data[base + 28..])?;

            let arch = FatArch64 { 
                cputype, 
                cpusubtype, 
                offset: arch_offset, 
                size, 
                align, 
                reserved 
            };

            archs.push(FatArch::Arch64(arch));
            offset += constants::FAT_ARCH64_SIZE;
        } else {
            // ==== fat_arch_32 ====
            // bounds check
            if offset + constants::FAT_ARCH32_SIZE > data.len() {
                return Err(format!(
                    "fat_arch_64[{}] extends beyond EOF",
                    i
                ).into());
            }

            let base = offset;
            let cputype: i32 = bytes_to(header.kind, &data[base + 0..])?;
            let cpusubtype: i32 = bytes_to(header.kind, &data[base + 4..])?;
            let arch_offset: u32 = bytes_to(header.kind, &data[base + 8..])?;
            let size: u32 = bytes_to(header.kind, &data[base + 12..])?;
            let align: u32 = bytes_to(header.kind, &data[base + 16..])?;
            

            let arch = FatArch32 { 
                cputype, 
                cpusubtype, 
                offset: arch_offset, 
                size, 
                align, 
            };

            archs.push(FatArch::Arch32(arch));
            offset += constants::FAT_ARCH32_SIZE;

        }
    }

    Ok(archs)
}

pub fn read_fat_header(data: &[u8]) -> Result<FatHeader, Box<dyn Error>> {
    use std::mem::size_of;

    if data.len() < size_of::<FatHeader>() {
        return Err("File too small to be a fat binary".into());
    }

    fn classify_fat_magic(bytes: [u8; 4]) -> Option<FatKind> {
        match bytes {
            constants::FAT_MAGIC    => Some(FatKind::Fat32BE),
            constants::FAT_MAGIC_64 => Some(FatKind::Fat64BE),
            constants::FAT_CIGAM    => Some(FatKind::Fat32LE),
            constants::FAT_CIGAM_64 => Some(FatKind::Fat64LE),
            _ => None,
        }
    }

    let raw_magic_bytes: [u8; 4] = data[0..4].try_into()?;

    let kind = match classify_fat_magic(raw_magic_bytes) {
        Some(kind) => kind,
        None => return Err("Not a fat Mach-O binary".into()),
    };

    
    let nfat_arch = if kind.is_be() {
        u32::from_be_bytes(data[4..8].try_into()?)
    } else {
        u32::from_le_bytes(data[4..8].try_into()?)
    };


    Ok(FatHeader {
        kind,
        nfat_arch,
    })
}
