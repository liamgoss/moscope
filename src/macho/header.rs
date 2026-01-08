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
use super::constants;

/*
From <mach-o/loader.h>
/* Constant for the magic field of the mach_header (32-bit architectures) */
#define	MH_MAGIC	0xfeedface	/* the mach magic number */
#define MH_CIGAM	0xcefaedfe	/* NXSwapInt(MH_MAGIC) */

/* Constant for the magic field of the mach_header_64 (64-bit architectures) */
#define MH_MAGIC_64 0xfeedfacf /* the 64-bit mach magic number */
#define MH_CIGAM_64 0xcffaedfe /* NXSwapInt(MH_MAGIC_64) */
*/
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