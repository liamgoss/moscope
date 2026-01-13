// File Purpose: Mach-O and Fat (Universal) binary constants.
// Constants were taken from the wikipedia page on Dec 16, 2025
// https://web.archive.org/web/20250000000000*/https://en.wikipedia.org/wiki/Mach-O
// NOTE: In several cases, data was taken from this site and given to ChatGPT 5.2 to convert to our `pub const ....` lines to avoid excess manual typing


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

pub const CPU_TYPE_VAX: i32         = 0x00000001;
pub const CPU_TYPE_ROMP: i32        = 0x00000002;
pub const CPU_TYPE_NS32032: i32     = 0x00000004;
pub const CPU_TYPE_NS32332: i32     = 0x00000005;
pub const CPU_TYPE_MC680X0: i32     = 0x00000006;
pub const CPU_TYPE_X86: i32         = 0x00000007;
pub const CPU_TYPE_MIPS: i32        = 0x00000008;
pub const CPU_TYPE_NS32352: i32     = 0x00000009;
pub const CPU_TYPE_HPPA: i32        = 0x0000000B;
pub const CPU_TYPE_ARM: i32         = 0x0000000C;
pub const CPU_TYPE_MC88000: i32     = 0x0000000D;
pub const CPU_TYPE_SPARC: i32       = 0x0000000E;
pub const CPU_TYPE_I860_BIG: i32    = 0x0000000F;
pub const CPU_TYPE_I860_LITTLE: i32 = 0x00000010; // or DEC Alpha
pub const CPU_TYPE_RS6000: i32      = 0x00000011;
pub const CPU_TYPE_POWERPC: i32     = 0x00000012;
pub const CPU_TYPE_RISCV: i32       = 0x00000018;

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

pub const CPU_SUBTYPE_X86_ALL: i32              = 0x00000003;
pub const CPU_SUBTYPE_X86_486: i32              = 0x00000004;
pub const CPU_SUBTYPE_X86_486SX: i32            = 0x00000084;
pub const CPU_SUBTYPE_X86_PENTIUM_M5: i32       = 0x00000056;
pub const CPU_SUBTYPE_X86_CELERON: i32          = 0x00000067;
pub const CPU_SUBTYPE_X86_CELERON_MOBILE: i32   = 0x00000077;
pub const CPU_SUBTYPE_X86_PENTIUM_3: i32        = 0x00000008;
pub const CPU_SUBTYPE_X86_PENTIUM_3_M: i32      = 0x00000018;
pub const CPU_SUBTYPE_X86_PENTIUM_3_XEON: i32   = 0x00000028;
pub const CPU_SUBTYPE_X86_PENTIUM_4: i32        = 0x0000000A;
pub const CPU_SUBTYPE_X86_ITANIUM: i32          = 0x0000000B;
pub const CPU_SUBTYPE_X86_ITANIUM_2: i32        = 0x0000001B;
pub const CPU_SUBTYPE_X86_XEON: i32             = 0x0000000C;
pub const CPU_SUBTYPE_X86_XEON_MP: i32          = 0x0000001C;

//
// ------------------------------------------------------------
// Mach-O file types
// ------------------------------------------------------------
// see filetype_name() below for explanations 
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

pub const MH_NOUNDEFS: u32                      = 1 << 0;     // the object file has no undefined references
pub const MH_INCRLINK: u32                      = 1 << 1;     // the object file is the output of an incremental link against a base file and can't be link edited again
pub const MH_DYLDLINK: u32                      = 1 << 2;     // the object file is input for the dynamic linker and can't be statically link edited again
pub const MH_BINDATLOAD: u32                    = 1 << 3;     // the object file's undefined references are bound by the dynamic linker when loaded.
pub const MH_PREBOUND: u32                      = 1 << 4;     // the file has its dynamic undefined references prebound.
pub const MH_SPLIT_SEGS: u32                    = 1 << 5;     // the file has its read-only and read-write segments split
pub const MH_LAZY_INIT: u32                     = 1 << 6;     // the shared library init routine is to be run lazily via catching memory faults to its writeable segments (obsolete)
pub const MH_TWOLEVEL: u32                      = 1 << 7;     // the image is using two-level name space bindings
pub const MH_FORCE_FLAT: u32                    = 1 << 8;     // the executable is forcing all images to use flat name space bindings
pub const MH_NOMULTIDEFS: u32                   = 1 << 9;     // this umbrella guarantees no multiple definitions of symbols in its sub-images so the two-level namespace hints can always be used.
pub const MH_NOFIXPREBINDING: u32               = 1 << 10;    // do not have dyld notify the prebinding agent about this executable
pub const MH_PREBINDABLE: u32                   = 1 << 11;    // the binary is not prebound but can have its prebinding redone. only used when MH_PREBOUND is not set.
pub const MH_ALLMODSBOUND: u32                  = 1 << 12;    // indicates that this binary binds to all two-level namespace modules of its dependent libraries. only used when MH_PREBINDABLE and MH_TWOLEVEL are both set.
pub const MH_SUBSECTIONS_VIA_SYMBOLS: u32       = 1 << 13;    // safe to divide up the sections into sub-sections via symbols for dead code stripping
pub const MH_CANONICAL: u32                     = 1 << 14;    // the binary has been canonicalized via the unprebind operation
pub const MH_WEAK_DEFINES: u32                  = 1 << 15;    // the final linked image contains external weak symbols
pub const MH_BINDS_TO_WEAK: u32                 = 1 << 16;    // the final linked image uses weak symbols
pub const MH_ALLOW_STACK_EXECUTION: u32         = 1 << 17;    // When this bit is set, all stacks in the task will be given stack execution privilege. Only used in MH_EXECUTE filetypes.
pub const MH_ROOT_SAFE: u32                     = 1 << 18;    // When this bit is set, the binary declares it is safe for use in processes with uid zero
pub const MH_SETUID_SAFE: u32                   = 1 << 19;    // When this bit is set, the binary declares it is safe for use in processes when issetugid() is true
pub const MH_NO_REEXPORTED_DYLIBS: u32          = 1 << 20;    // When this bit is set on a dylib, the static linker does not need to examine dependent dylibs to see if any are re-exported
pub const MH_PIE: u32                           = 1 << 21;    // When this bit is set, the OS will load the main executable at a random address. Only used in MH_EXECUTE filetypes.
pub const MH_DEAD_STRIPPABLE_DYLIB: u32         = 1 << 22;    // Only for use on dylibs. When linking against a dylib that has this bit set, the static linker will automatically not create a LC_LOAD_DYLIB load command to the dylib if no symbols are being referenced from the dylib.
pub const MH_HAS_TLV_DESCRIPTORS: u32           = 1 << 23;    // Contains a section of type S_THREAD_LOCAL_VARIABLES
pub const MH_NO_HEAP_EXECUTION: u32             = 1 << 24;    // When this bit is set, the OS will run the main executable with a non-executable heap even on platforms (e.g. i386) that don't require it. Only used in MH_EXECUTE filetypes.
pub const MH_APP_EXTENSION_SAFE: u32            = 1 << 25;    // The code was linked for use in an application extension.
pub const MH_NLIST_OUTOFSYNC_WITH_DYLDINFO: u32 = 1 << 26;    // The external symbols listed in the nlist symbol table do not include all the symbols listed in the dyld info.
pub const MH_SIM_SUPPORT: u32                   = 1 << 27;    // Allow LC_MIN_VERSION_MACOS and LC_BUILD_VERSION load commands with the platforms macOS, macCatalyst, iOSSimulator, tvOSSimulator and watchOSSimulator.
pub const MH_IMPLICIT_PAGEZERO: u32             = 1 << 28;    // main executable has no __PAGEZERO segment. Instead, loader (xnu) will load program high and block out all memory below it.
pub const MH_DYLIB_IN_CACHE: u32                = 1 << 31;    // Only for use on dylibs. When this bit is set, the dylib is part of the dyld shared cache, rather than loose in the filesystem.

//
// ------------------------------------------------------------
// Load Commands
// ------------------------------------------------------------
pub const LC_REQ_DYLD: u32                  = 0x8000_0000; // When a new LC is added that need to be understood by the dynamic linker, the LC_REQ_DYLD will be BITWISE OR'ed into the LC constant
pub const LC_SEGMENT: u32                   = 0x01; // segment of this file to be mapped
pub const LC_SYMTAB: u32                    = 0x02; // link-edit stab symbol table info
pub const LC_SYMSEG: u32                    = 0x03; // link-edit gdb symbol table info
pub const LC_THREAD: u32                    = 0x04; // thread
pub const LC_UNIXTHREAD: u32                = 0x05; // unix thread (includes a stack)
pub const LC_LOADFVMLIB: u32                = 0x06; // load a specified fixed VM shared library
pub const LC_IDFVMLIB: u32                  = 0x07; // fixed VM shared library identification
pub const LC_IDENT: u32                     = 0x08; // object identification info (obsolete)
pub const LC_FVMFILE: u32                   = 0x09; // fixed VM file inclusion (internal use)
pub const LC_PREPAGE: u32                   = 0x0A; // prepage command (internal use)
pub const LC_DSYMTAB: u32                   = 0x0B; // dynamic link-edit symbol table info
pub const LC_LOAD_DYLIB: u32                = 0x0C; // load a dynamically linked shared library
pub const LC_ID_DYLIB: u32                  = 0x0D; // dynamically linked shared lib ident
pub const LC_LOAD_DYLINKER: u32             = 0x0E; // load a dynamic linker
pub const LC_ID_DYLINKER: u32               = 0x0F; // dynamic linker identification
pub const LC_PREBOUND_DYLIB: u32            = 0x10; // modules prebound for a dynamically linked shared library
pub const LC_ROUTINES: u32                  = 0x11; // image routines
pub const LC_SUB_FRAMEWORK: u32             = 0x12; // sub framework
pub const LC_SUB_UMBRELLA: u32              = 0x13; // sub umbrella
pub const LC_SUB_CLIENT: u32                = 0x14; // sub client
pub const LC_SUB_LIBRARY: u32               = 0x15; // sub library
pub const LC_TWOLEVEL_HINTS: u32            = 0x16; // two-level namespace lookup hints
pub const LC_PREBIND_CKSUM: u32             = 0x17; // prebind checksum
pub const LC_LOAD_WEAK_DYLIB: u32           = 0x18; // BITWISE OR LC_REQ_DYLD // load a dynamically linked shared library that is allowed to be missing
pub const LC_SEGMENT_64: u32                = 0x19; // 64-bit segment of this file to be mapped
pub const LC_ROUTINES_64: u32               = 0x1A; // 64-bit image routines
pub const LC_UUID: u32                      = 0x1B; // the uuid
pub const LC_RPATH: u32                     = 0x1C; // BITWISE OR LC_REQ_DYLD // runpath additions
pub const LC_CODE_SIGNATURE: u32            = 0x1D; // local of code signature
pub const LC_SEGMENT_SPLIT_INFO: u32        = 0x1E; // local of info to split segments
pub const LC_REEXPORT_DYLIB: u32            = 0x1F; // BITWISE OR LC_REQ_DYLD // load and re-export dylib
pub const LC_LAZY_LOAD_DYLIB: u32           = 0x20; // delay load of dylib until first use
pub const LC_ENCRYPTION_INFO: u32           = 0x21; // encrypted segment information
pub const LC_DYLD_INFO: u32                 = 0x22; // compressed dyld information
pub const LC_DYLD_INFO_ONLY: u32            = 0x22; // BITWISE OR LC_REQ_DYLD // compressed dyld information only
pub const LC_LOAD_UPWARD_DYLIB: u32         = 0x23; // BITWISE OR LC_REQ_DYLD // load upward dylib
pub const LC_VERSION_MIN_MACOSX: u32        = 0x24; // build for MacOSX min OS version
pub const LC_VERSION_MIN_IPHONEOS: u32      = 0x25; // build for iPhoneOS min OS version
pub const LC_FUNCTION_STARTS: u32           = 0x26; // compressed table of function start addresses
pub const LC_DYLD_ENVIRONMENT: u32          = 0x27; // string for dyld to treat like environment variable
pub const LC_MAIN: u32                      = 0x28; // BITWISE OR LC_REQ_DYLD // replacement for LC_UNIXTHREAD
pub const LC_DATA_IN_CODE: u32              = 0x29; // table of non-instructions in __text
pub const LC_SOURCE_VERSION: u32            = 0x2A; // source version used to build binary
pub const LC_DYLIB_CODE_SIGN_DRS: u32       = 0x2B; // Code signing DRs copied from linked dylibs
pub const LC_ENCRYPTION_INFO_64: u32        = 0x2C; // 64-bit encrypted segment information
pub const LC_LINKER_OPTION: u32             = 0x2D; // linker options in MH_OBJECT files
pub const LC_LINKER_OPTIMIZATION_HINT: u32  = 0x2E; // optimization hints in MH_OBJECT files
pub const LC_VERSION_MIN_TVOS: u32          = 0x2F; // build for Apple TV min OS version
pub const LC_VERSION_MIN_WATCHOS: u32       = 0x30; // build for Watch min OS version
pub const LC_NOTE: u32                      = 0x31; // arbitrary data included within a Mach-O file
pub const LC_BUILD_VERSION: u32             = 0x32; // build for platform min OS version
pub const LC_DYLD_EXPORTS_TRIE: u32         = 0x33; // BITWISE OR LC_REQ_DYLD // used with linkedit_data_command, payload is trie
pub const LC_DYLD_CHAINED_FIXUPS: u32       = 0x34; // BITWISE OR LC_REQ_DYLD // used with linkedit_data_command
pub const LC_FILESET_ENTRY: u32             = 0x35; // BITWISE OR LC_REQ_DYLD // used with fileset_entry_command
pub const LC_ATOM_INFO: u32                 = 0x36; // used with linkedit_data_command
pub const LC_FUNCTION_VARIANTS: u32         = 0x37; // used with linkedit_data_command
pub const LC_FUNCTION_VARIANT_FIXED: u32    = 0x38; // used with linkedit_data_command
pub const LC_TARGET_TRIPLE: u32             = 0x39; // target triple used to compile



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
                    CPU_SUBTYPE_ARM64_ALL => "arm64 (ARM64_ALL)", // 0
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