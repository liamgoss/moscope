// File Purpose: Handle LC_DYLD_INFO and LC_DYLD_INFO_ONLY and fixups

// from mach-o/loader.h
#[derive(Debug, Clone, Copy)]
struct LC_DYLD_INFO {
    pub cmd: u32,                   // LC_DYLD_INFO or LC_DYLD_INFO_ONLY
    pub cmdsize: u32,               // sizeof(struct dyld_info_command)
    // Adjust for ASLR
    pub rebase_off: u32,            // file offset to rebase info
    pub rebase_size: u32,           // size of rebase info
    // External symbols from other libraries
    // primarily going to be __DATA_CONST/__got
    pub bind_off: u32,              // file offset to binding info
    pub bind_size: u32,             // size of binding info
    // Handle weak symbols (that may have multiple definitions)
    // __la_symbol_ptr and also __got
    pub weak_bind_off: u32,         // file offset to weak binding info
    pub weak_bind_size: u32,        // size of the weak binding info
    // Delay binding symbols until they're used first 
    // __la_symbol_ptr holds lazy pointers
    pub lazy_bind_off: u32,         // file offset to lazy binding
    pub lazy_bind_size: u32,        // size of lazy binding info
    // Lists symbols binary provides to others
    pub export_off: u32,            // file offset to lazy binding info
    pub export_size: u32,           // size of lazy binding info
}
