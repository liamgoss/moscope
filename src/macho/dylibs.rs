// File Purpose: What does the binary depend on? 

// From mach-o's loader.h 

use std::error::Error;
use crate::macho::constants::{LC_ID_DYLIB, LC_LAZY_LOAD_DYLIB, LC_LOAD_DYLIB, LC_LOAD_UPWARD_DYLIB, LC_LOAD_WEAK_DYLIB, LC_REEXPORT_DYLIB};
use crate::macho::load_commands::LoadCommand;
use crate::macho::utils;
use colored::Colorize;

/*

dylib_command memory layout 
+-----------------------------+
| cmd (u32)                   |
| cmdsize (u32)               |
| dylib.name.offset (u32)     | <-- offset from its respective command 
| dylib.timestamp (u32)       |
| dylib.current_version (u32) |
| dylib.compat_version (u32)  |
| "path/to/lib.dylib\0"       | <-- variable length, padded
+-----------------------------+

*/
#[derive(Debug, Clone)]
pub enum DylibKind {
    Id,
    Load,
    Weak,
    Reexport,
    Lazy,
    Upward,
    Unknown,
}
// dylib fields:
    /* 
        The `name`` is an lc_str in loader.h so we gotta look at `lc_str`` in loader.h
        union lc_str {
            uint32_t	offset;	/* offset to the string */
        #ifndef __LP64__
            char		*ptr;	/* pointer to the string */
        #endif 
        };

        Thus, we make name here a u32 which is the offset from the 
        start of the load command its used in
    
    name: u32, // offset, library's path name
    timestamp: u32, // library's build time stamp
    current_version: u32, //library's current version number
    compatibility_version: u32, // library's compatibility version number
*/
#[derive(Debug, Clone)]
pub struct ParsedDylib {
    pub path: String,
    pub timestamp: u32,
    pub current_version: u32,
    pub compatibility_version: u32,
    pub kind: DylibKind,
    pub source_lc: LoadCommand,
}


pub fn parse_dylib(data: &[u8], lc: &LoadCommand, is_be: bool) -> Result<ParsedDylib, Box<dyn Error>> {
    // Good ol' bounds checking 
    let base = lc.offset as usize;
    let end = base + lc.cmdsize as usize;

    if end > data.len() {
        return Err("dylib load command exceeds file bounds".into());
    }

    let name_offset: u32 = utils::bytes_to(is_be, &data[base + 8..])?; // start at plus 8 to skip cmd & cmdsize
    let timestamp: u32 = utils::bytes_to(is_be, &data[base + 12..])?;
    let current_version: u32 = utils::bytes_to(is_be, &data[base + 16..])?;
    let compat_version: u32 = utils::bytes_to(is_be, &data[base + 20..])?;

    let string_start = base + name_offset as usize;
    let string_end = base + lc.cmdsize as usize; // Thankfully the cmdsize is given so we know the max size of the string

    
    if string_start >= string_end || string_end > data.len() {
        return Err("Invalid dylib name offset".into());
    }

    let string_bytes = &data[string_start..string_end];

    let first_null_byte = match string_bytes.iter().position(|&byte| byte == 0) {
        Some(pos) => pos,
        None => return Err("Unterminated dylib name string".into()),
    };

    let path = String::from_utf8_lossy(&string_bytes[..first_null_byte]).to_string();

    let kind = match lc.cmd {
        LC_ID_DYLIB => DylibKind::Id,
        LC_LOAD_DYLIB => DylibKind::Load,
        LC_LOAD_WEAK_DYLIB => DylibKind::Weak,
        LC_REEXPORT_DYLIB => DylibKind::Reexport,
        LC_LAZY_LOAD_DYLIB => DylibKind::Lazy,
        LC_LOAD_UPWARD_DYLIB => DylibKind::Upward,
        _ => DylibKind::Unknown,
    };

    Ok(ParsedDylib {
        path: path,
        timestamp: timestamp,
        current_version: current_version,
        compatibility_version: compat_version,
        kind: kind,
        source_lc: *lc,
    })
}

pub fn print_dylibs_summary(dylibs: &Vec<ParsedDylib>) {
    println!("{}", "\nDynamic Libraries".green().bold());
    println!("----------------------------------------");

    for dylib in dylibs {
        let kind = match dylib.kind {
            DylibKind::Id => "ID".yellow().bold(),
            DylibKind::Load => "LOAD".yellow().bold(),
            DylibKind::Weak => "WEAK".yellow().bold(),
            DylibKind::Reexport => "REEXPORT".yellow().bold(),
            DylibKind::Lazy => "LAZY".yellow().bold(),
            DylibKind::Upward => "UPWARD".yellow().bold(),
            DylibKind::Unknown => "UNKNOWN".red().bold(),
        };

        //println!("[{:<8}] {} DEBUG:{:?}", kind, dylib.path, dylib.source_lc.cmd);
        println!("[{:<8}] {}", kind, dylib.path);
    }
}