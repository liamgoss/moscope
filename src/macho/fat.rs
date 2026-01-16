// File Purpose: "Which Mach-O should be parsed?"
use std::error::Error;
use crate::macho::constants;
use crate::macho::utils;



/*
+-------------------+
| fat_header        |  <-- global container metadata
+-------------------+
| fat_arch[0]       |  <-- where Mach-O #0 lives
+-------------------+
| fat_arch[1]       |  <-- where Mach-O #1 lives
+-------------------+
| ...               |
+-------------------+
| Mach-O #0 bytes   |  <-- offset from fat_arch[0]
+-------------------+
| Mach-O #1 bytes   |  <-- offset from fat_arch[1]
+-------------------+
*/





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

pub fn read_fat_archs(
    data: &[u8],            // Entire file contents
    header: &FatHeader,     // Previously-parsed fat header
) -> Result<Vec<FatArch>, Box<dyn Error>> {
    let mut archs = Vec::new();
    let mut offset: usize = constants::FAT_HEADER_SIZE; // Start on the on disk fat header


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
            let cputype: i32 = utils::bytes_to(header.kind.is_be(), &data[base + 0..])?;
            let cpusubtype: i32 = utils::bytes_to(header.kind.is_be(), &data[base + 4..])?;
            let arch_offset: u64 = utils::bytes_to(header.kind.is_be(), &data[base + 8..])?;
            let size: u64 = utils::bytes_to(header.kind.is_be(), &data[base + 16..])?;
            let align: u32 = utils::bytes_to(header.kind.is_be(), &data[base + 24..])?;
            let reserved: u32 = utils::bytes_to(header.kind.is_be(), &data[base + 28..])?;

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
            let cputype: i32 = utils::bytes_to(header.kind.is_be(), &data[base + 0..])?;
            let cpusubtype: i32 = utils::bytes_to(header.kind.is_be(), &data[base + 4..])?;
            let arch_offset: u32 = utils::bytes_to(header.kind.is_be(), &data[base + 8..])?;
            let size: u32 = utils::bytes_to(header.kind.is_be(), &data[base + 12..])?;
            let align: u32 = utils::bytes_to(header.kind.is_be(), &data[base + 16..])?;
            

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

    let kind: FatKind = match classify_fat_magic(raw_magic_bytes) {
        Some(kind) => kind,
        None => return Err("Not a valid fat Mach-O binary".into()),
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


/*
============================
======== UNIT TESTS ========
============================ 
*/

#[cfg(test)]
mod tests {
    use super::*;
    use crate::macho::constants::*;

    #[test]
    fn read_fat_header_32_be() {
        let data = [
            // FAT_MAGIC
            0xCA, 0xFE, 0xBA, 0xBE,
            // nfat_arch = 2 (to be read as BE)
            0x00, 0x00, 0x00, 0x02,
        ];

        let header = read_fat_header(&data).unwrap();

        assert_eq!(header.kind, FatKind::Fat32BE);
        assert_eq!(header.nfat_arch, 2);
    }

    #[test]
    fn read_fat_header_64_be() {
        let data = [
            // FAT_MAGIC_64
            0xCA, 0xFE, 0xBA, 0xBF,
            // nfat_arch = 2 (to be read as BE)
            0x00, 0x00, 0x00, 0x02,
        ];

        let header = read_fat_header(&data).unwrap();

        assert_eq!(header.kind, FatKind::Fat64BE);
        assert_eq!(header.nfat_arch, 2);
    }

    #[test]
    fn read_fat_header_rejects_invalid_magic() {
        let data = [
            0xDE, 0xAD, 0xBE, 0xEF,
            0x00, 0x00, 0x00, 0x01,
        ];

        assert!(read_fat_header(&data).is_err());
    }

    #[test]
    fn read_single_fat_arch_32_be() {

        // DIY Fat header to be a FatArch32 with 1 nfat_arch and it's an x86

        let mut data = Vec::new();

        // FAT Header
        data.extend_from_slice(&FAT_MAGIC); // 0xCAFEBABE
        data.extend_from_slice(&1u32.to_be_bytes()); // nfat_arch = 1

        // fat_arch_32 as if it's an x86 binary
        data.extend_from_slice(&constants::CPU_TYPE_X86.to_be_bytes());
        data.extend_from_slice(&0u32.to_be_bytes()); // cpusubtype
        data.extend_from_slice(&0x1000u32.to_be_bytes()); // offset to binary
        data.extend_from_slice(&0x2000u32.to_be_bytes()); // size of binary
        data.extend_from_slice(&0x4u32.to_be_bytes());    // align

        let header = read_fat_header(&data).unwrap();
        let archs = read_fat_archs(&data, &header).unwrap();

        assert_eq!(archs.len(), 1);

        match &archs[0] {
            FatArch::Arch32(arch) => {
                assert_eq!(arch.cputype, CPU_TYPE_X86);
                assert_eq!(arch.offset, 0x1000); // 4096
                assert_eq!(arch.size, 0x2000); // 8192
                assert_eq!(arch.align, 4); // alignment
            }

            _ => panic!("Expected Arch32"),
        }
    }


    // bounds checking verification
    #[test]
    fn read_fat_arch_truncated_fails() {
        let mut data = Vec::new();

        data.extend_from_slice(&constants::FAT_MAGIC);
        data.extend_from_slice(&1u32.to_be_bytes());

        // Only part of fat_arch_32 (too short)
        data.extend_from_slice(&[0x00; 8]);

        let header = read_fat_header(&data).unwrap();
        let archs = read_fat_archs(&data, &header);

        assert!(archs.is_err());
    }

}