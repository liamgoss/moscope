use std::error::Error;
use colored::Colorize;
use regex::Regex;
use crate::macho::utils;
use crate::macho::constants::*;
use crate::reporting::symtab::*;

// As per *OS Internals Vol. 1 (UserSpace) - Chapter 6
// LC_SYMTAB specifies the offset and number of entries in the symbol and string tables of the object 
// From mach-o/nlist.h
/*
* Format of a symbol table entry of a Mach-O file for 32-bit architectures.
* Modified from the BSD format.  The modifications from the original format
* were changing n_other (an unused field) to n_sect and the addition of the
* N_SECT type.  These modifications are required to support symbols in a larger
* number of sections not just the three sections (text, data and bss) in a BSD
* file.

struct nlist {
    union {
#ifndef __LP64__
        char *n_name;	/* for use when in-core */
#endif
        uint32_t n_strx;	/* index into the string table */
    } n_un;
    uint8_t n_type;		/* type flag, see below */
    uint8_t n_sect;		/* section number or NO_SECT */
    int16_t n_desc;		/* see <mach-o/stab.h> */
    uint32_t n_value;	/* value of this symbol (or stab offset) */
};

* This is the symbol table entry structure for 64-bit architectures.

struct nlist_64 {
    union {
        uint32_t  n_strx; /* index into the string table */
    } n_un;
    uint8_t n_type;        /* type flag, see below */
    uint8_t n_sect;        /* section number or NO_SECT */
    uint16_t n_desc;       /* see <mach-o/stab.h> */
    uint64_t n_value;      /* value of this symbol (or stab offset) */
};
*/
// https://developer.apple.com/documentation/kernel/nlist_64/1583957-n_desc
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NList32 {
    n_strx: u32, // index into the string table
    n_type: u8, // type flag
    n_sect: u8, // section number or NO_SECT
    n_desc: u16, // A 16-bit value providing additional information about the nature of this symbol
    n_value: u32, // value of this symbol or stab offset
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NList64 {
    n_strx: u32, // index into the string table
    n_type: u8, // type flag
    n_sect: u8, // section number or NO_SECT
    n_desc: u16, // A 16-bit value providing additional information about the nature of this symbol
    n_value: u64, // value of this symbol or stab offset
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// SymbolKind is determined by n_type which isn't necessarily a "type" but it's a bitfield
/*
7 6 5 |   4  | 3 2 1 |  0
------+------+-------+----
 STAB | PEXT | TYPE  | EXT

 if N_STAB != 0 --> the symbol is a debugging entry, can have a meaningless n value maybe?
 N_EXT --> external
 N_PEXT --> private external
*/
pub enum SymbolKind {
    Undefined,          // N_UNDF
    Absolute,           // N_ABS
    Section,            // N_SECT
    PreboundUndefined,  // N_PBUD
    Indirect,           // N_INDR
    Lazy,               // __la_symbol_ptr
    Stub,               // __stubs
    Got,            // __got
    Unknown,
}

impl SymbolKind {
    pub fn from_n_type(n_type: u8) -> Self {
        match n_type & N_TYPE {
            N_UNDF => SymbolKind::Undefined,
            N_ABS => SymbolKind::Absolute,
            N_SECT => SymbolKind::Section,
            N_PBUD => SymbolKind::PreboundUndefined,
            N_INDR => SymbolKind::Indirect,
            _ => SymbolKind::Unknown,
        }
    }
}



pub struct ParsedString {
    pub value: String,
    pub segname: [u8; 16],
    pub sectname: [u8; 16],
}

impl ParsedString {
    pub fn build_report(&self, _is_json: bool) -> StringReport {
        StringReport { 
            value: self.value.clone(), 
            segname: String::from_utf8_lossy(&self.segname).trim_end_matches('\0').to_string(), 
            sectname: String::from_utf8_lossy(&self.sectname).trim_end_matches('\0').to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct ParsedSymbol {
    pub name: String,
    pub addr: u64,
    pub value: u64,
    pub kind: SymbolKind,
    pub section: Option<SectionIndex>,
    pub is_external: bool,
    pub is_debug: bool,
    pub sectname: Option<String>,
    pub segname: Option<String>,
    pub n_desc: u16,
    pub n_type: u8,
    pub n_sect: u8,
    pub indirect_addr: Option<u64>,
    pub indirect_sect: Option<String>,
}

impl ParsedSymbol {
    pub fn from_nlist32(nlist: NList32, data: &[u8], str_offset: usize, str_size: usize) -> Self {
        let kind = SymbolKind::from_n_type(nlist.n_type);
        let is_debug = (nlist.n_type & N_STAB) != 0;
        let is_external = (nlist.n_type & N_EXT) != 0;
        let section = if nlist.n_sect == 0 { None } else { Some(SectionIndex(nlist.n_sect)) };
        let name = read_symbol_name(data, str_offset, str_size, nlist.n_strx).unwrap_or_else(|| "N/A".to_string());

        ParsedSymbol {
            name,
            addr: nlist.n_value as u64,
            n_type: nlist.n_type,
            n_sect: nlist.n_sect,
            n_desc: nlist.n_desc,
            value: nlist.n_value as u64,
            kind,
            section,
            is_external,
            is_debug,
            sectname: None, 
            segname: None,
            indirect_addr: None,
            indirect_sect: None,
        }
    }

    pub fn from_nlist64(nlist: NList64, data: &[u8], str_offset: usize, str_size: usize) -> Self {
        let kind = SymbolKind::from_n_type(nlist.n_type);
        let is_debug = (nlist.n_type & N_STAB) != 0;
        let is_external = (nlist.n_type & N_EXT) != 0;
        let section = if nlist.n_sect == 0 { None } else { Some(SectionIndex(nlist.n_sect)) };
        let name = read_symbol_name(data, str_offset, str_size, nlist.n_strx).unwrap_or_else(|| "N/A".to_string());
        

        ParsedSymbol {
            name,
            addr: nlist.n_value as u64,
            n_type: nlist.n_type,
            n_sect: nlist.n_sect,
            n_desc: nlist.n_desc,
            value: nlist.n_value,
            kind,
            section,
            is_external,
            is_debug,
            sectname: None, 
            segname: None,
            indirect_addr: None,
            indirect_sect: None,
        }
    }

    pub fn build_report(&self, json: bool) -> SymbolReport {
        SymbolReport {
            name: self.name.clone(),
            value: self.value,
            addr: self.addr,
            addr_hex: format!("0x{:x}", self.addr),
            kind: if json {
                self.kind_plain()
            } else {
                self.kind_colored()
            },
            section: self.section.map(|s| s.0),
            external: self.is_external,
            debug: self.is_debug,
            sectname: self.sectname.clone(),
            segname: self.segname.clone(),
        }
    }

    fn kind_plain(&self) -> String {
        match self.kind {
            SymbolKind::Undefined           => "UNDEF",
            SymbolKind::Absolute            => "ABS",
            SymbolKind::Section             => "SECT",
            SymbolKind::PreboundUndefined   => "PBUD",
            SymbolKind::Indirect            => "INDR",
            SymbolKind::Lazy                => "LAZY",
            SymbolKind::Stub                => "STUB",
            SymbolKind::Got                 => "GOT",
            SymbolKind::Unknown             => "UNKNOWN"
        }.to_string()
    }

    fn kind_colored(&self) -> String {
        match self.kind {
            SymbolKind::Undefined           => "UNDEF".yellow().bold(),
            SymbolKind::Absolute            => "ABS".yellow().bold(),
            SymbolKind::Section             => "SECT".green().bold(),
            SymbolKind::PreboundUndefined   => "PBUD".yellow().bold(),
            SymbolKind::Indirect            => "INDR".yellow().bold(),
            SymbolKind::Lazy                => "LAZY".yellow().bold(),
            SymbolKind::Stub                => "STUB".yellow().bold(),
            SymbolKind::Got                 => "GOT".yellow().bold(),
            SymbolKind::Unknown             => "UNKNOWN".red().bold(),
        }.to_string()
    }

    pub fn bind_str(&self) -> &'static str {
        if self.is_external { "EXT" } else { "LOC" }
    }

    pub fn sect_str(&self) -> String {
        self.sectname.clone().unwrap_or_else(|| "-".into())
    }

    pub fn seg_str(&self) -> String {
        self.segname.clone().unwrap_or_else(|| "-".into())
    }
    
}

#[derive(Debug, Clone, Copy)]
pub struct SymtabCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub symoff: u32,
    pub nsyms: u32,
    pub stroff: u32,
    pub strsize: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct DYSymtabCommand {
    pub cmd: u32,
    pub cmdsize: u32,
    pub ilocalsym: u32, // index to local symbols
    pub nlocalsym: u32, // number of local symbols
    pub iextdefsym: u32, // index to externally defined symbols
    pub nextdefsym: u32, // number of externally defined symbols
    pub iundefsym: u32, // index to undefined symbols
    pub nundefsym: u32, // number of undefined symbols
    pub tocoff: u32, // file offset to table of contents
    pub ntoc: u32, // number of entires in table of contents
    pub modtaboff: u32, // file offset to module table
    pub nmodtab: u32, // number of module table entires
    pub extrefsymoff: u32, // offset to referenced symbol table
    pub nextrefsyms: u32, // number of referenced symbol table entries
    pub indirectsymoff: u32, // file offset to the indirect symbol table
    pub nindirectsyms: u32, // number of indirect symbol table entries
    pub extreloff: u32, // offset to external relocation entries
    pub nextrel: u32, // number of external relocation entries
    pub locreloff: u32, // offset to local relocation entires
    pub nlocrel: u32, // number of local relocation entries
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SectionIndex(pub u8);

impl NList32 {
    pub const SIZE: usize = 12;

    pub fn parse(data: &[u8], offset: usize, is_be: bool) -> Result<Self, Box<dyn Error>> {
        let n_strx: u32 = utils::bytes_to(is_be, &data[offset .. offset + 4])?;
        let n_type: u8 = data[offset + 4];
        let n_sect: u8 = data[offset + 5];
        let n_desc: u16 = utils::bytes_to(is_be, &data[offset + 6 .. offset + 8])?;
        let n_value: u32 = utils::bytes_to(is_be, &data[offset + 8 .. offset + 12])?;
        
        Ok(Self { n_strx, n_type, n_sect, n_desc, n_value })
    }
}

impl NList64 {
    pub const SIZE: usize = 16;

    pub fn parse(data: &[u8], offset: usize, is_be: bool) -> Result<Self, Box<dyn Error>> {
        let n_strx: u32 = utils::bytes_to(is_be, &data[offset .. offset + 4])?;
        let n_type: u8 = data[offset + 4];
        let n_sect: u8 = data[offset + 5];
        let n_desc: u16 = utils::bytes_to(is_be, &data[offset + 6 .. offset + 8])?;
        let n_value: u64 = utils::bytes_to(is_be, &data[offset + 8 .. offset + 16])?;

        Ok(Self { n_strx, n_type, n_sect, n_desc, n_value })
    }
}

pub fn read_symbol_name(data: &[u8], str_offset: usize, str_size: usize, strx: u32) -> Option<String> {
    if strx == 0 {
        return None;
    }

    let start = str_offset + strx as usize;
    let end = str_offset + str_size;

    if start >= end {
        return None;
    }

    let mut cursor = start;
    while cursor < end && data[cursor] != 0 {
        cursor += 1;
    }

    std::str::from_utf8(&data[start..cursor]).ok().map(|s| s.to_string())
}


pub fn extract_strings(section_data: &[u8], min_len: usize) -> Vec<String> {
    let mut strings = Vec::new();
    let mut start = 0;

    while start < section_data.len() {
        // just like in rpaths we check for the first null byte
        if let Some(end) = section_data[start..].iter().position(|&byte| byte == 0) {
            let slice = &section_data[start..start + end];
            if slice.len() >= min_len {
                if let Ok(s) = std::str::from_utf8(slice) {
                    strings.push(escape_string(s).to_string());
                }
            }

            start += end + 1; // skip the null byte
        } else {
            break;
        }
    }

    strings
}

pub fn extract_filtered_strings(section_data: &[u8], pattern: &str) -> Result<Vec<String>, regex::Error> {
    let re = Regex::new(pattern)?;
    // If using regex, we want all strings (min_len = 1)
    Ok(extract_strings(section_data, 1)
        .into_iter()
        .filter(|s| re.is_match(s))
        .collect())
}

fn escape_string(s: &str) -> String {
    s.chars()
        .flat_map(|c| match c {
            '\n' => "\\n".chars().collect::<Vec<_>>(),
            '\r' => "\\r".chars().collect::<Vec<_>>(),
            '\t' => "\\t".chars().collect::<Vec<_>>(),
            '\0' => "\\0".chars().collect::<Vec<_>>(),
            c if c.is_control() => format!("\\x{:02x}", c as u8).chars().collect(),
            c => vec![c],
        })
        .collect()
}


fn sort_addr(sym: &ParsedSymbol) -> Option<u64> {
    sym.indirect_addr.or_else(|| {
        if sym.addr != 0 { Some(sym.addr) } else { None }
    })
}

pub fn print_symbols_summary(symbols: &[ParsedSymbol]) {
    if symbols.is_empty() {
        return;
    }

    let mut symbols = symbols.to_vec();

    // Sort by address that will be printed with undefined symbols last
    symbols.sort_by(|a, b| {
    match (sort_addr(a), sort_addr(b)) {
        (Some(a_addr), Some(b_addr)) => a_addr.cmp(&b_addr),
        (Some(_), None) => std::cmp::Ordering::Less,  // addressed first
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    }
});

    println!();
    println!("{}", "Symbols".green().bold());
    println!("--------------------------------------------------------------------------------");
    println!(
        "{:<18} {:<6} {:<5} {:<20} {}",
        "Address", "Type", "Bind", "Section", "Symbol"
    );
    println!("--------------------------------------------------------------------------------");

    for sym in symbols {
        // Format address: show '-' if 0
        let addr_str = if let Some(indirect) = sym.indirect_addr {
            format!("0x{:016x}", indirect)
        } else if sym.addr != 0 {
            format!("0x{:016x}", sym.addr)
        } else {
            "-".to_string()
        };

        println!(
            "{:<18} {:<6} {:<5} {:<20} {}",
            addr_str,
            sym.kind_plain(),
            sym.bind_str(),
            sym.sect_str(),
            sym.name
        );
    }

    println!("--------------------------------------------------------------------------------");
}

pub fn print_strings_summary(strings: &Vec<ParsedString>, min_len: usize, max_count: Option<usize>) {
    if strings.is_empty() {
        return;
    }

    println!("{}", "\nStrings".green().bold());
    println!("----------------------------------------");

    // Filter by min length
    let mut filtered: Vec<&ParsedString> = strings.iter().filter(|s| s.value.len() >= min_len).collect();

    // Sort or limit if max_count is provided
    if let Some(max) = max_count {
        filtered.truncate(max);
    }

    for s in filtered {
        let segname_raw = String::from_utf8_lossy(&s.segname);
        let segname = segname_raw.trim_end_matches('\0');
        let sectname_raw = String::from_utf8_lossy(&s.sectname);
        let sectname = sectname_raw.trim_end_matches('\0');

        println!("[{}:{}] {}", segname, sectname, s.value);
    }
}
