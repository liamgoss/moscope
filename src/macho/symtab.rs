use std::error::Error;
use crate::macho::utils::{self, bytes_to};

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
struct NList32 {
    n_strx: u32, // index into the string table
    n_type: u8, // type flag
    n_sect: u8, // section number or NO_SECT
    n_desc: u16, // A 16-bit value providing additional information about the nature of this symbol
    n_value: u32, // value of this symbol or stab offset
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct NList64 {
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
}




pub struct ParsedSymbol {
    name: Option<String>,
    value: u64,
    kind: SymbolKind,
    section: Option<SectionIndex>,
    is_external: bool,
    is_debug: bool,
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

impl NList32 {
    pub const SIZE: usize = 12;

    pub fn parse(data: &[u8], offset: usize, is_be: bool) -> Result<Self, Box<dyn Error>> {
        let n_strx: u32 = utils::bytes_to(is_be, &data[offset .. offset + 4])?;
        let n_type: u8 = data[offset + 4];
        let n_sect: u8 = data[offset + 5];
        let n_desc: u16 = utils::bytes_to(is_be, &data[offset + 6 .. offset + 8])?;
        let n_value: u32 = utils::bytes_to(is_be, &data[offset + 8 .. offset + 16])?;

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

