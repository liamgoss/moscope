// File Purpose: "What load commands are present in a given binary?"
use crate::macho::constants::*;
use crate::macho::utils;
use std::error::Error;
use colored::Colorize;





#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct LoadCommand {
    pub cmd: u32, // type of load command
    pub cmdsize: u32, // total size of command in bytes
    pub offset: u64, // Offset to this load command from start of Mach-O image
    /*
        ^ cmd size's VALUE must be:
            - a multiple of 4 bytes for 32 bit
            - a multiple of 8 bytes for 64 bit
     */
}

pub fn load_comand_name(cmd: u32) -> &'static str {
    /*
        cmd & LC_REQ_DYLD != 0 // flag
        cmd & !LC_REQ_DYLD // base command
     */ 

    let _requires_dyld = (cmd & LC_REQ_DYLD) != 0; // Not using at the moment, prefixing w/ underscore
    let base_cmd = cmd & !LC_REQ_DYLD;

    match base_cmd { // Is there any better way to do this? This feels wrong, is there a way to like reflect the variable names back based on their value?
        // This should be O(1) so I guess it's not inefficient it's just a hassle to type which is fine.
        // According to the interwebs, it may go up to linear time depending on number of elements and compiler optimizations...

        // NOTE: I had ChatGPT 5.2 take my constants.rs snippet and spit this out so I didn't have to type it.
        // I made sure everything is present except LC_REQ_DYLD which was masked off
        LC_SEGMENT                    => "LC_SEGMENT",
        LC_SYMTAB                     => "LC_SYMTAB",
        LC_SYMSEG                     => "LC_SYMSEG",
        LC_THREAD                     => "LC_THREAD",
        LC_UNIXTHREAD                 => "LC_UNIXTHREAD",
        LC_LOADFVMLIB                 => "LC_LOADFVMLIB",
        LC_IDFVMLIB                   => "LC_IDFVMLIB",
        LC_IDENT                      => "LC_IDENT",
        LC_FVMFILE                    => "LC_FVMFILE",
        LC_PREPAGE                    => "LC_PREPAGE",
        LC_DSYMTAB                    => "LC_DSYMTAB",
        LC_LOAD_DYLIB                 => "LC_LOAD_DYLIB",
        LC_ID_DYLIB                   => "LC_ID_DYLIB",
        LC_LOAD_DYLINKER              => "LC_LOAD_DYLINKER",
        LC_ID_DYLINKER                => "LC_ID_DYLINKER",
        LC_PREBOUND_DYLIB             => "LC_PREBOUND_DYLIB",
        LC_ROUTINES                   => "LC_ROUTINES",
        LC_SUB_FRAMEWORK              => "LC_SUB_FRAMEWORK",
        LC_SUB_UMBRELLA               => "LC_SUB_UMBRELLA",
        LC_SUB_CLIENT                 => "LC_SUB_CLIENT",
        LC_SUB_LIBRARY                => "LC_SUB_LIBRARY",
        LC_TWOLEVEL_HINTS             => "LC_TWOLEVEL_HINTS",
        LC_PREBIND_CKSUM              => "LC_PREBIND_CKSUM",
        LC_LOAD_WEAK_DYLIB            => "LC_LOAD_WEAK_DYLIB",
        LC_SEGMENT_64                 => "LC_SEGMENT_64",
        LC_ROUTINES_64                => "LC_ROUTINES_64",
        LC_UUID                       => "LC_UUID",
        LC_RPATH                      => "LC_RPATH",
        LC_CODE_SIGNATURE             => "LC_CODE_SIGNATURE",
        LC_SEGMENT_SPLIT_INFO         => "LC_SEGMENT_SPLIT_INFO",
        LC_REEXPORT_DYLIB             => "LC_REEXPORT_DYLIB",
        LC_LAZY_LOAD_DYLIB            => "LC_LAZY_LOAD_DYLIB",
        LC_ENCRYPTION_INFO            => "LC_ENCRYPTION_INFO",
        LC_DYLD_INFO                  => "LC_DYLD_INFO",
        LC_LOAD_UPWARD_DYLIB          => "LC_LOAD_UPWARD_DYLIB",
        LC_VERSION_MIN_MACOSX         => "LC_VERSION_MIN_MACOSX",
        LC_VERSION_MIN_IPHONEOS       => "LC_VERSION_MIN_IPHONEOS",
        LC_FUNCTION_STARTS            => "LC_FUNCTION_STARTS",
        LC_DYLD_ENVIRONMENT           => "LC_DYLD_ENVIRONMENT",
        LC_MAIN                       => "LC_MAIN",
        LC_DATA_IN_CODE               => "LC_DATA_IN_CODE",
        LC_SOURCE_VERSION             => "LC_SOURCE_VERSION",
        LC_DYLIB_CODE_SIGN_DRS        => "LC_DYLIB_CODE_SIGN_DRS",
        LC_ENCRYPTION_INFO_64         => "LC_ENCRYPTION_INFO_64",
        LC_LINKER_OPTION              => "LC_LINKER_OPTION",
        LC_LINKER_OPTIMIZATION_HINT   => "LC_LINKER_OPTIMIZATION_HINT",
        LC_VERSION_MIN_TVOS           => "LC_VERSION_MIN_TVOS",
        LC_VERSION_MIN_WATCHOS        => "LC_VERSION_MIN_WATCHOS",
        LC_NOTE                       => "LC_NOTE",
        LC_BUILD_VERSION              => "LC_BUILD_VERSION",
        LC_DYLD_EXPORTS_TRIE          => "LC_DYLD_EXPORTS_TRIE",
        LC_DYLD_CHAINED_FIXUPS        => "LC_DYLD_CHAINED_FIXUPS",
        LC_FILESET_ENTRY              => "LC_FILESET_ENTRY",
        LC_ATOM_INFO                  => "LC_ATOM_INFO",
        LC_FUNCTION_VARIANTS          => "LC_FUNCTION_VARIANTS",
        LC_FUNCTION_VARIANT_FIXED     => "LC_FUNCTION_VARIANT_FIXED",
        LC_TARGET_TRIPLE              => "LC_TARGET_TRIPLE",
        _                             => "UNKNOWN_LOAD_COMMAND",
    }
}

pub fn print_load_commands(
    load_commands: &Vec<LoadCommand>,
) {
    println!();
    println!("{} {}", "Load Commands Found: ".green().bold(), load_commands.len());
    println!("----------------------------------------");
    for lc in load_commands {
        println!(" - {:<30} cmd=0x{:08x} size={}", load_comand_name(lc.cmd), lc.cmd, lc.cmdsize);
    }
    println!("----------------------------------------");
    println!();    


}


pub fn read_load_commands(
    data: &[u8],
    offset: u32,
    num_load_commands: u32,
    word_size: u32, // 32 or 64,
    big_endian: bool,
) -> Result<Vec<LoadCommand>, Box<dyn Error>> {
    let mut load_commands: Vec<LoadCommand> = Vec::new();
    let mut cursor = offset as usize;
    
    if word_size != 32 && word_size != 64 {
        return Err(format!("Incorrect or Unsupported word size supplied. Expected 32 or 64, received {}", word_size).into());
    }

    let alignment = if word_size == 64 { 8 } else { 4 };

    for i in 0..num_load_commands {
        if cursor + 8 >= data.len() {
            return Err(format!("Load command {} header exceeds file bounds", i).into());
        }
        
        let cmd: u32 = utils::bytes_to(big_endian, &data[cursor..])?; // Don't have to specify end index because bytes_to already knows the size
        let cmd_size: u32 = utils::bytes_to(big_endian, &data[cursor+4..])?;

        // Now verify variable length data as specified by cmd_size
        if cmd_size < 8 {
            return Err(format!("Load command {} has invalid cmdsize of {}", i, cmd_size).into());
        }

        if cmd_size % alignment != 0 {
            return Err(format!("Load command {} with cmdsize {} is not {}-byte aligned", i, cmd_size, alignment).into());
        }

        if cursor + cmd_size as usize > data.len() {
            return Err(format!("Load command {} exceeds file bounds", i).into());
        }

        // Now we can finally read it
        load_commands.push(LoadCommand { cmd, cmdsize: cmd_size, offset: cursor as u64 });

        cursor += cmd_size as usize;

    }

    Ok(load_commands)

}