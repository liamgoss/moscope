// File Purpose: Where is the dynamic loader looking for libraries at runtime?

use std::error::Error;
use crate::macho::load_commands::{LoadCommand, load_command_name};
use crate::macho::utils;
use colored::Colorize;
use crate::reporting::rpaths::RPathsReport;

#[derive(Debug, Clone)]
pub struct ParsedRPath {
    pub source_lc: LoadCommand,
    pub path: String,
}

impl ParsedRPath {
    pub fn build_report(&self, is_json: bool) -> RPathsReport {
        RPathsReport { source_lc: load_command_name(self.source_lc.cmd).to_string(), path: self.path.clone() }
    }
}


pub fn parse_rpath(data: &[u8], lc: &LoadCommand, is_be: bool) -> Result<ParsedRPath, Box<dyn Error>> {
    // Check bounds
    let base = lc.offset as usize;
    let end = base + lc.cmdsize as usize;

    if end > data.len() {
        return Err("LC_RPATH exceeds file bounds".into());
    }



    // We can reuse pretty much all of the dylib reading code here
    let path_offset: u32 = utils::bytes_to(is_be, &data[base + 8..])?; // start at plus 8 to skip cmd & cmdsize
    let string_start = base + path_offset as usize;
    let string_end = base + lc.cmdsize as usize; 

    
    if string_start >= string_end || string_end > data.len() {
        return Err("RPATH path offset exceeds file bounds".into());
    }

    let string_bytes = &data[string_start..string_end];

    let first_null_byte = match string_bytes.iter().position(|&byte| byte == 0) {
        Some(pos) => pos,
        None => return Err("Unterminated RPATH path string".into()),
    };

    let rpath = String::from_utf8_lossy(&string_bytes[..first_null_byte]).to_string();

    Ok(ParsedRPath { source_lc: *lc, path: rpath })

}


pub fn print_rpaths_summary(rpaths: &Vec<ParsedRPath>) {
    if rpaths.is_empty() {
        return;
    }

    println!("{}", "\nRPATHs".green().bold());
    println!("----------------------------------------");

    for rpath in rpaths {
        println!("[{}] {}", "RPATH".yellow().bold(), rpath.path);
    }
}