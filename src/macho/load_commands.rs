use super::constants;
use super::utils;

// File Purpose: "What load commands are present in a given binary?"




#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct load_command {
    pub cmd: u32, // type of load command
    pub cmdsize: u32, // total size of command in bytes
    /*
        ^ cmd size MUST be:
            - a multiple of 4 bytes for 32 bit
            - a multiple of 8 bytes for 64 bit
     */
}