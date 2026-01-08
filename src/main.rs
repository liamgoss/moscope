#![allow(warnings)]
mod macho;

use macho::fat;
use macho::header;
use macho::constants;
use std::error::Error;
use colorize::AnsiColor;
use clap::Parser;
use std::path::PathBuf;

use crate::macho::constants::FAT_CIGAM_64;


#[derive(Parser, Debug)]
#[command(
    name = "moscope",
    version,
    about = "Mach-O static analysis and inspection toolkit"
)]
struct Cli {
    /// Path to the Mach-O binary to inspect
    #[arg(value_name = "BINARY")]
    binary: PathBuf,
}

fn decode_arm64_subtype(cpusubtype: i32) -> &'static str {
    let base = cpusubtype & !constants::CPU_SUBTYPE_MASK;
    let has_ptrauth = (cpusubtype & constants::CPU_SUBTYPE_PTRAUTH_ABI) != 0;

    if has_ptrauth {
        "arm64e"
    } else {
        match base {
            constants::CPU_SUBTYPE_ARM64_ALL |
            constants::CPU_SUBTYPE_ARM64_V8 => "arm64",
            _ =>  "arm64 (unknown subtype)",
        }
    }
}

fn display_arch(cputype: i32, cpusubtype: i32) -> (&'static str, &'static str) {
    let cpu = constants::cpu_type_name(cputype);

    let subtype = match cputype {
        constants::CPU_TYPE_ARM64 => decode_arm64_subtype(cpusubtype),
        _ => constants::cpu_subtype_name(cputype, cpusubtype),
    };

    (cpu, subtype)
}


fn fat_binary_user_decision(archs: &[fat::FatArch]) -> Result<(), Box<dyn Error>> {
    // Prompt user if they want to analyze the Intel or Apple Silicon binary
    println!("{}", "Available architectures:".green());
    for (i, arch) in archs.iter().enumerate() {
        match arch {
            fat::FatArch::Arch32(a) => {
                let (cpu, sub) = display_arch(a.cputype, a.cpusubtype);
                println!("{i}: {cpu} ({sub})");
            }
            fat::FatArch::Arch64(a) => {
                let (cpu, sub) = display_arch(a.cputype, a.cpusubtype);
                println!("{i}: {cpu} ({sub})");
            }
        }
    }

    use std::io::{self, Write};
    print!("Select architecture index: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice: usize = input.trim().parse()?;

    println!("Selected architecture {}", choice);


    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // Parse CLI arguments
    let cli = Cli::parse();

    println!("Reading header...");

    // Read the entire file into memory
    // Then all fat header parse & mach-o header parser can work from the same byte slice
    // but...hopefully it's not a large binary (need to change later)
    let data = std::fs::read(&cli.binary)
        .map_err(|e| format!("failed to read '{}': {}", cli.binary.display(), e))?;
    

    // Attempt to parse a FAT (universal) header
    // If successful, the file contains multiple Mach-O images
    // If it fails, the file is a thin/singular Mach-O binary


    match fat::read_fat_header(&data) {
        Ok(fat_header) => {
            println!("{}", "Fat binary detected:".green());
            println!("{:?}", fat_header);

            // Parse all architecture entries described by the fat header

            let archs = fat::read_fat_archs(
                &data,
                &fat_header,
            )?;

            // Present architectures to user for selection
            fat_binary_user_decision(&archs)?;
        }
        Err(_) => {
            println!("{}", "Not a fat binary!".red());
            // handle_binary(&data)?;
        }
    };

    Ok(())
}
