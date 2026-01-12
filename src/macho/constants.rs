// File Purpose: Mach-O and Fat (Universal) binary constants.
// Constants were taken from the wikipedia page on Dec 16, 2025
// https://web.archive.org/web/20250000000000*/https://en.wikipedia.org/wiki/Mach-O



//
// ------------------------------------------------------------
// Mach-O magic numbers (on-disk byte order)
// ------------------------------------------------------------
// These are the first 4 bytes as they appear in the file.
//
// "$(xcrun --sdk macosx --show-sdk-path)/usr/include/mach-o/fat.h"

/// 32-bit Mach-O, big-endian (MH_MAGIC = 0xfeedface)
pub const MH_MAGIC: [u8; 4] = [0xFE, 0xED, 0xFA, 0xCE];

/// 32-bit Mach-O, little-endian (MH_CIGAM = 0xcefaedfe)
pub const MH_CIGAM: [u8; 4] = [0xCE, 0xFA, 0xED, 0xFE];

/// 64-bit Mach-O, big-endian (MH_MAGIC_64 = 0xfeedfacf)
pub const MH_MAGIC_64: [u8; 4] = [0xFE, 0xED, 0xFA, 0xCF];

/// 64-bit Mach-O, little-endian (MH_CIGAM_64 = 0xcffaedfe)
pub const MH_CIGAM_64: [u8; 4] = [0xCF, 0xFA, 0xED, 0xFE];

//
// ------------------------------------------------------------
// Fat / Universal binary magic numbers
// ------------------------------------------------------------
//

/// Fat binary (32 bit offsets/sizes in fat arch table), big-endian
pub const FAT_MAGIC: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];

/// Fat binary (32 bit offsets/sizes in fat arch table), little-endian
pub const FAT_CIGAM: [u8; 4] = [0xBE, 0xBA, 0xFE, 0xCA];

/// Fat binary (64 bit offsets/sizes in fat arch table), big-endian
pub const FAT_MAGIC_64: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBF];

/// Fat binary (64 bit offsets/sizes in fat arch table), little-endian
pub const FAT_CIGAM_64: [u8; 4] = [0xBF, 0xBA, 0xFE, 0xCA];

pub const FAT_HEADER_SIZE: usize = 8;
pub const FAT_ARCH32_SIZE: usize = 20;
pub const FAT_ARCH64_SIZE: usize = 32;
pub const MACH_HEADER32_SIZE: usize = 28;
pub const MACH_HEADER64_SIZE: usize = 32;


//
// ------------------------------------------------------------
// CPU architecture ABI flags
// ------------------------------------------------------------

/// CPU uses a 64-bit ABI
pub const CPU_ARCH_ABI64: i32 = 0x0100_0000;


/// CPU uses a 64-bit ABI with 32-bit pointers
pub const CPU_ARCH_ABI64_32: i32 = 0x0200_0000;

//
// ------------------------------------------------------------
// CPU types
// ------------------------------------------------------------

pub const CPU_TYPE_VAX: i32        = 0x00000001;
pub const CPU_TYPE_ROMP: i32       = 0x00000002;
pub const CPU_TYPE_NS32032: i32    = 0x00000004;
pub const CPU_TYPE_NS32332: i32    = 0x00000005;
pub const CPU_TYPE_MC680X0: i32    = 0x00000006;
pub const CPU_TYPE_X86: i32        = 0x00000007;
pub const CPU_TYPE_MIPS: i32       = 0x00000008;
pub const CPU_TYPE_NS32352: i32    = 0x00000009;
pub const CPU_TYPE_HPPA: i32       = 0x0000000B;
pub const CPU_TYPE_ARM: i32        = 0x0000000C;
pub const CPU_TYPE_MC88000: i32    = 0x0000000D;
pub const CPU_TYPE_SPARC: i32      = 0x0000000E;
pub const CPU_TYPE_I860_BIG: i32   = 0x0000000F;
pub const CPU_TYPE_I860_LITTLE: i32= 0x00000010; // or DEC Alpha
pub const CPU_TYPE_RS6000: i32     = 0x00000011;
pub const CPU_TYPE_POWERPC: i32    = 0x00000012;
pub const CPU_TYPE_RISCV: i32      = 0x00000018;

/// Common combined CPU types
pub const CPU_TYPE_X86_64: i32 = CPU_TYPE_X86 | CPU_ARCH_ABI64;
pub const CPU_TYPE_ARM64: i32  = CPU_TYPE_ARM | CPU_ARCH_ABI64;

//
// ------------------------------------------------------------
// CPU subtype masks
// ------------------------------------------------------------
/// Mask for extracting the subtype capability bits

// pub const CPU_SUBTYPE_MASK: i32 = 0xff00_0000;
// This mask value ^ exceeds i32 value  
// so we gotta two's comp. it 
pub const CPU_SUBTYPE_MASK: i32 = -0x0100_0000;

//
// ------------------------------------------------------------
// ARM CPU subtypes
// ------------------------------------------------------------

pub const CPU_SUBTYPE_ARM_ALL: i32      = 0x00000000;
pub const CPU_SUBTYPE_ARM_A500: i32     = 0x00000001;
pub const CPU_SUBTYPE_ARM_A500_2: i32   = 0x00000002;
pub const CPU_SUBTYPE_ARM_A440: i32     = 0x00000003;
pub const CPU_SUBTYPE_ARM_M4: i32       = 0x00000004;
pub const CPU_SUBTYPE_ARM_V4T: i32      = 0x00000005;
pub const CPU_SUBTYPE_ARM_V6: i32       = 0x00000006;
pub const CPU_SUBTYPE_ARM_V5TEJ: i32    = 0x00000007;
pub const CPU_SUBTYPE_ARM_XSCALE: i32   = 0x00000008;
pub const CPU_SUBTYPE_ARM_V7: i32       = 0x00000009;
pub const CPU_SUBTYPE_ARM_V7F: i32      = 0x0000000A;
pub const CPU_SUBTYPE_ARM_V7S: i32      = 0x0000000B;
pub const CPU_SUBTYPE_ARM_V7K: i32      = 0x0000000C;
pub const CPU_SUBTYPE_ARM_V8: i32       = 0x0000000D;
pub const CPU_SUBTYPE_ARM_V6M: i32      = 0x0000000E;
pub const CPU_SUBTYPE_ARM_V7M: i32      = 0x0000000F;
pub const CPU_SUBTYPE_ARM_V7EM: i32     = 0x00000010;

// ------------------------------------------------------------
// ARM64 CPU subtypes (from <mach/machine.h>)
// ------------------------------------------------------------

/// Pointer authentication ABI flag (arm64e)
//pub const CPU_SUBTYPE_PTRAUTH_ABI: i32 = 0x8000_0000;
pub const CPU_SUBTYPE_PTRAUTH_ABI: i32 = i32::MIN;

/// ARM64 subtypes
pub const CPU_SUBTYPE_ARM64_ALL: i32 = 0;
pub const CPU_SUBTYPE_ARM64_V8: i32  = 1;
pub const CPU_SUBTYPE_ARM64E: i32    = 2;


//
// ------------------------------------------------------------
// x86 CPU subtypes
// ------------------------------------------------------------

pub const CPU_SUBTYPE_X86_ALL: i32       = 0x00000003;
pub const CPU_SUBTYPE_X86_486: i32       = 0x00000004;
pub const CPU_SUBTYPE_X86_486SX: i32     = 0x00000084;
pub const CPU_SUBTYPE_X86_PENTIUM_M5: i32= 0x00000056;
pub const CPU_SUBTYPE_X86_CELERON: i32   = 0x00000067;
pub const CPU_SUBTYPE_X86_CELERON_MOBILE: i32 = 0x00000077;
pub const CPU_SUBTYPE_X86_PENTIUM_3: i32 = 0x00000008;
pub const CPU_SUBTYPE_X86_PENTIUM_3_M: i32=0x00000018;
pub const CPU_SUBTYPE_X86_PENTIUM_3_XEON: i32=0x00000028;
pub const CPU_SUBTYPE_X86_PENTIUM_4: i32 = 0x0000000A;
pub const CPU_SUBTYPE_X86_ITANIUM: i32   = 0x0000000B;
pub const CPU_SUBTYPE_X86_ITANIUM_2: i32 = 0x0000001B;
pub const CPU_SUBTYPE_X86_XEON: i32      = 0x0000000C;
pub const CPU_SUBTYPE_X86_XEON_MP: i32   = 0x0000001C;

//
// ------------------------------------------------------------
// Mach-O file types
// ------------------------------------------------------------

pub const MH_OBJECT: u32      = 0x00000001;
pub const MH_EXECUTE: u32     = 0x00000002;
pub const MH_FVMLIB: u32      = 0x00000003;
pub const MH_CORE: u32        = 0x00000004;
pub const MH_PRELOAD: u32     = 0x00000005;
pub const MH_DYLIB: u32       = 0x00000006;
pub const MH_DYLINKER: u32    = 0x00000007;
pub const MH_BUNDLE: u32      = 0x00000008;
pub const MH_DYLIB_STUB: u32  = 0x00000009;
pub const MH_DSYM: u32        = 0x0000000A;
pub const MH_KEXT_BUNDLE: u32 = 0x0000000B;
pub const MH_FILESET: u32     = 0x0000000C;

//
// ------------------------------------------------------------
// Mach-O header flags
// ------------------------------------------------------------

pub const MH_NOUNDEFS: u32                = 1 << 0;
pub const MH_INCRLINK: u32                = 1 << 1;
pub const MH_DYLDLINK: u32                = 1 << 2;
pub const MH_BINDATLOAD: u32              = 1 << 3;
pub const MH_PREBOUND: u32                = 1 << 4;
pub const MH_SPLIT_SEGS: u32              = 1 << 5;
pub const MH_LAZY_INIT: u32               = 1 << 6;
pub const MH_TWOLEVEL: u32                = 1 << 7;
pub const MH_FORCE_FLAT: u32              = 1 << 8;
pub const MH_NOMULTIDEFS: u32             = 1 << 9;
pub const MH_NOFIXPREBINDING: u32         = 1 << 10;
pub const MH_PREBINDABLE: u32             = 1 << 11;
pub const MH_ALLMODSBOUND: u32            = 1 << 12;
pub const MH_SUBSECTIONS_VIA_SYMBOLS: u32 = 1 << 13;
pub const MH_CANONICAL: u32               = 1 << 14;
pub const MH_WEAK_DEFINES: u32             = 1 << 15;
pub const MH_BINDS_TO_WEAK: u32           = 1 << 16;
pub const MH_ALLOW_STACK_EXECUTION: u32   = 1 << 17;
pub const MH_ROOT_SAFE: u32               = 1 << 18;
pub const MH_SETUID_SAFE: u32             = 1 << 19;
pub const MH_NO_REEXPORTED_DYLIBS: u32    = 1 << 20;
pub const MH_PIE: u32                     = 1 << 21;
pub const MH_DEAD_STRIPPABLE_DYLIB: u32   = 1 << 22;
pub const MH_HAS_TLV_DESCRIPTORS: u32     = 1 << 23;
pub const MH_NO_HEAP_EXECUTION: u32       = 1 << 24;
pub const MH_APP_EXTENSION_SAFE: u32      = 1 << 25;
pub const MH_NLIST_OUTOFSYNC_WITH_DYLDINFO: u32 = 1 << 26;
pub const MH_SIM_SUPPORT: u32             = 1 << 27;
pub const MH_DYLIB_IN_CACHE: u32          = 1 << 31;

pub fn cpu_type_name(cputype: i32) -> &'static str {
    match cputype & !CPU_ARCH_ABI64 {
        CPU_TYPE_X86 => "x86",
        CPU_TYPE_ARM => "ARM",
        CPU_TYPE_POWERPC => "PowerPC",
        CPU_TYPE_RISCV => "RISC-V",
        _ => "Unknown",
    }
}

pub fn cpu_subtype_name(cputype: i32, cpusubtype: i32) -> &'static str {
    // cputype: The CPU architecture type from the Mach-O header
    //   - Contains the base CPU type (ARM, x86, etc.) in the lower bits
    //   - May have CPU_ARCH_ABI64 (0x01000000) flag set in upper bits for 64-bit architectures
    //   - Example: CPU_TYPE_ARM64 = 0x0100000C (ARM with 64-bit ABI flag)
    //
    // cpusubtype: The specific CPU variant/generation
    //   - Contains capability flags in the upper bits (masked by CPU_SUBTYPE_MASK)
    //   - Contains the actual subtype value in the lower bits
    //   - Example for arm64e: 0x80000002 = CPU_SUBTYPE_PTRAUTH_ABI | CPU_SUBTYPE_ARM64E
    
    //println!("cputype: {}, cpusubtype: {}", cputype, cpusubtype);
    
    match cputype {
        // CPU_TYPE_ARM64 = 0x0100000C (CPU_TYPE_ARM | CPU_ARCH_ABI64)
        CPU_TYPE_ARM64 => {
            // CPU_SUBTYPE_PTRAUTH_ABI = 0x80000000 (i32::MIN as signed value)
            // This is the high bit that indicates PAC support (arm64e)
            if (cpusubtype & CPU_SUBTYPE_PTRAUTH_ABI) != 0 {
                // If the pointer auth bit is set, this is arm64e regardless of other subtype bits
                "arm64e"
            } else {
                // For non-arm64e variants, extract the actual subtype value
                // CPU_SUBTYPE_MASK = 0xff000000 (capability bits, ignore)
                // !CPU_SUBTYPE_MASK = 0x00ffffff (keeps only the subtype value)
                let subtype = cpusubtype & !CPU_SUBTYPE_MASK;
                
                match subtype {
                    CPU_SUBTYPE_ARM64_V8 => "arm64", // 1
                    CPU_SUBTYPE_ARM64_ALL => "arm64 (generic)", // 0
                    _ => "ARM64 (unknown subtype)",
                }
            }
        },
        
        // CPU_TYPE_ARM = 0x0000000C
        // This matches 32-bit ARM architectures (older iOS devices, some embedded systems)
        CPU_TYPE_ARM => {
            // For 32-bit ARM, we just extract the subtype without checking special flags
            let subtype = cpusubtype & !CPU_SUBTYPE_MASK;
            
            match subtype {
                // CPU_SUBTYPE_ARM_V7 = 9 (ARMv7 architecture - iPhone 5s and earlier)
                CPU_SUBTYPE_ARM_V7 => "ARMv7",
                
                // CPU_SUBTYPE_ARM_V8 = 13 (ARMv8 in 32-bit mode)
                CPU_SUBTYPE_ARM_V8 => "ARMv8",
                
                _ => "ARM (unknown subtype)",
            }
        },
        
        // CPU_TYPE_X86_64 = 0x01000007 (CPU_TYPE_X86 | CPU_ARCH_ABI64)
        // Intel/AMD 64-bit x86 architecture
        CPU_TYPE_X86_64 => "x86_64",
        
        // CPU_TYPE_X86 = 0x00000007
        // Intel/AMD 32-bit x86 architecture (i386)
        CPU_TYPE_X86 => "x86",
        
        // Any CPU type we don't recognize
        _ => "Unknown",

        // There's a lot more cpusubtypes defined above from wikipedia, IDK if we should have them all defined here or not
        // Pros:....completeness
        // Cons:....???
        // TODO

    }
}

pub fn filetype_name(filetype: u32) -> &'static str {
    // Pulling these strings from Ghidra's docs 
    // Why Ghidra docs and not also Wikipedia you ask? --> Ghidra's entries are more verbose
    // https://web.archive.org/web/20251224153001/https://ghidra.re/ghidra_docs/api/ghidra/app/util/bin/format/macho/MachHeaderFileTypes.html
    match filetype {
        MH_OBJECT        => "Relocatable Object File [[MH_OBJECT]]", // Dear reader: don't confuse [[*]] with markdown formatting, I just think it's visually appealing
        MH_EXECUTE       => "Demand Paged Executable File [[MH_EXECUTE]]",
        MH_FVMLIB        => "Fixed VM Shared Library File [[MH_FVMLIB]]",
        MH_CORE          => "Core File [[MH_CORE]]",
        MH_PRELOAD       => "Preloaded Executable File [[MH_PRELOAD]]",
        MH_DYLIB         => "Dynamically Bound Shared Library [[MH_DYLIB]]",
        MH_DYLINKER      => "Dynamic Linker Editor [[MH_DYLINKER]]",
        MH_BUNDLE        => "Dynamically Bound Bundle File [[MH_BUNDLE]]",
        MH_DYLIB_STUB    => "Shared Library Stub for Static Linking Only, No Section Contents [[MH_DYLIB_STUB]]",
        MH_DSYM          => "Linking Only, No Section Contents, Companion File w/ Only Debug Sections [[MH_DSYM]]",
        MH_KEXT_BUNDLE   => "x86_64 kext (Kernel Extension) [[MH_KEXT_BUNDLE]]",
        MH_FILESET      => "Kernel Cache Fileset [[MH_FILESET]]",
        _ => "Unknown File Type",
    }
}