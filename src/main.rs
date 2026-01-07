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
    name = "checkomacho",
    version,
    about = "Inspect Mach-O and FAT (universal) binaries"
)]
struct Cli {
    /// Path to the Mach-O binary to inspect
    #[arg(value_name = "BINARY")]
    binary: PathBuf,
}

fn fat_binary_user_decision(archs: &[fat::FatArch]) -> Result<(), Box<dyn Error>> {
    // Prompt user if they want to analyze the Intel or Apple Silicon binary
    println!("{}", "Available architectures:".green());
    for (i, arch) in archs.iter().enumerate() {
        match arch {
            fat::FatArch::Arch32(a) => {
                println!(
                    "{}: {} ({})",
                    i,
                    constants::cpu_type_name(a.cputype),
                    constants::cpu_subtype_name(a.cputype, a.cpusubtype),
                );
            }
            fat::FatArch::Arch64(a) => {
                println!(
                    "{}: {} ({})",
                    i,
                    constants::cpu_type_name(a.cputype),
                    constants::cpu_subtype_name(a.cputype, a.cpusubtype),
                );
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
    // Then all parsers can work from the same byte slice
    // but...hopefully it's not a large binary
    let data = std::fs::read(&cli.binary)
        .map_err(|e| format!("failed to read '{}': {}", cli.binary.display(), e))?;
    

    // Attempt to parse a FAT (universal) header
    // If successful, the file contains multiple Mach-O images
    // If it fails, the file is a thin/singular Mach-O binary
    match fat::read_fat_header(&data) {
        Ok(fat_header) => {
            println!("{}", "Fat binary detected:".green());
            println!("{:?}", fat_header);

            // Determine FAT properties from the magic bytes
            let raw_magic: [u8; 4] = data[0..4].try_into()?;

            let is_fat_header_64: bool = raw_magic == constants::FAT_MAGIC_64 || raw_magic == constants::FAT_CIGAM_64;

            
            //let needs_swap = raw_magic == constants::FAT_CIGAM || raw_magic == constants::FAT_CIGAM_64;
            //let needs_swap = cfg!(target_endian = "little");

            println!(
                "[main] raw_magic={:02x} {:02x} {:02x} {:02x}, is_fat_header_64={}, needs_swap={}",
                raw_magic[0], raw_magic[1], raw_magic[2], raw_magic[3],
                is_fat_header_64,
                needs_swap
            );

            let archs = fat::read_fat_archs(
                &data,
                &fat_header,
                is_fat_header_64,
                needs_swap,
            )?;

            fat_binary_user_decision(&archs)?;
        }
        Err(_) => {
            println!("{}", "Not a fat binary!".red());
            // handle_binary(&data)?;
        }
    };

    Ok(())
}
