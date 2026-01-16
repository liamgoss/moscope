// File Purpose: Enumerate Sections, Work with segments.rs

use crate::macho::segments::*;
use crate::macho::constants::*;
use crate::macho::utils;
use std::error::Error;
use std::mem::size_of;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionKind {
    Code,
    CString,
    ConstData,
    Stub,
    SymbolPointer,
    Bss,
    ObjC,
    LinkEdit,
    Other,
    Unknown
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Section {        // For 32-bit architectures
    pub sectname: [u8; 16], // name of this section
    pub segname: [u8; 16],  // segment this section goes in
    pub addr: u32,          // memory address of this section
    pub size: u32,          // size in bytes of this section
    pub offset: u32,        // file offset of this section
    pub align: u32,         // section alignment (power of 2)
    pub reloff: u32,        // file offset of relocation entries
    pub nreloc: u32,        // number of relocation entries
    pub flags: u32,         // flags (section type and attributes)
    pub reserved1: u32,     // reserved (for offset or index)
    pub reserved2: u32,     // reserved (for count or size)
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Section64 {      // For 64-bit architectures
    pub sectname: [u8; 16], // name of this section
    pub segname: [u8; 16],  // segment this section goes in
    pub addr: u64,          // memory address of this section
    pub size: u64,          // size in bytes of this section
    pub offset: u32,        // file offset of this section
    pub align: u32,         // section alignment (power of 2)
    pub reloff: u32,        // file offset of relocation entries
    pub nreloc: u32,        // number of relocation entries
    pub flags: u32,         // flags (section type and attributes)
    pub reserved1: u32,     // reserved (for offset or index)
    pub reserved2: u32,     // reserved (for count or size)
    pub reserved3: u32,     // reserved 
}

pub struct ParsedSection {
    pub sectname: [u8; 16], 
    pub segname: [u8; 16],  
    pub addr: u64,          
    pub size: u64,         
    pub flags: u32,        
    pub kind: SectionKind, 
}

pub fn read_section64_from_bytes(data: &[u8], is_be: bool, sect_offset: usize ) -> Result<ParsedSection, Box<dyn Error>> {
    // bounds check
    if sect_offset + size_of::<Section64>() > data.len() {
        println!("sect_offset {:?} + {:?} exceeds {:?}", sect_offset, size_of::<Section64>(), data.len());
        return Err("Section out of bounds".into());
    }
    
    let sect_name = data[sect_offset .. sect_offset + 16].try_into()?;
    let seg_name = data[sect_offset + 16 .. sect_offset + 32].try_into()?;
    let sect_addr = utils::bytes_to(is_be, &data[sect_offset + 32..])?; 
    let sect_size = utils::bytes_to(is_be, &data[sect_offset + 40..])?;
    let sect_flags = utils::bytes_to(is_be, &data[sect_offset + 64..])?;
    
    // classify
    let sect_type = sect_flags & SECTION_TYPE;
    let sect_kind = match sect_type {
        S_CSTRING_LITERALS => SectionKind::CString,
        S_ZEROFILL => SectionKind::Bss,
        S_SYMBOL_STUBS => SectionKind::Stub,
        S_LAZY_SYMBOL_POINTERS | S_NON_LAZY_SYMBOL_POINTERS => SectionKind::SymbolPointer,
        S_MOD_INIT_FUNC_POINTERS | S_MOD_TERM_FUNC_POINTERS => SectionKind::Other,
        _ => {
            if seg_name == SEG_TEXT && sect_name == SECT_TEXT {
                SectionKind::Code
            } else if seg_name == SEG_LINKEDIT {
                SectionKind::LinkEdit
            } else {
                SectionKind::Unknown
            }
        }
    };

    Ok(ParsedSection {
        sectname: sect_name,
        segname: seg_name,
        addr: sect_addr,
        size: sect_size,
        flags: sect_flags,
        kind: sect_kind,
    })
}



pub fn read_section32_from_bytes(
    data: &[u8],
    is_be: bool,
    sect_offset: usize,
) -> Result<ParsedSection, Box<dyn Error>> {

    // bounds check
    if sect_offset + size_of::<Section64>() > data.len() {
        println!("sect_offset {:?} + {:?} exceeds {:?}", sect_offset, size_of::<Section>(), data.len());
        return Err("Section out of bounds".into());
    }
    let sect_name: [u8; 16] = data[sect_offset .. sect_offset + 16].try_into()?;
    let seg_name: [u8; 16] = data[sect_offset + 16 .. sect_offset + 32].try_into()?;
    let sect_addr_32: u32 = utils::bytes_to(is_be, &data[sect_offset + 32 ..])?;
    let sect_size_32: u32 = utils::bytes_to(is_be, &data[sect_offset + 36 ..])?;
    let sect_flags: u32 = utils::bytes_to(is_be, &data[sect_offset + 56 ..])?;

    // widen to 64-bit for ParsedSection
    let sect_addr = sect_addr_32 as u64;
    let sect_size = sect_size_32 as u64;

    // classify
    let sect_type = sect_flags & SECTION_TYPE;
    let sect_kind = match sect_type {
        S_CSTRING_LITERALS => SectionKind::CString,
        S_ZEROFILL => SectionKind::Bss,
        S_SYMBOL_STUBS => SectionKind::Stub,
        S_LAZY_SYMBOL_POINTERS | S_NON_LAZY_SYMBOL_POINTERS => SectionKind::SymbolPointer,
        S_MOD_INIT_FUNC_POINTERS | S_MOD_TERM_FUNC_POINTERS => SectionKind::Other,
        _ => {
            if seg_name == SEG_TEXT && sect_name == SECT_TEXT {
                SectionKind::Code
            } else if seg_name == SEG_LINKEDIT {
                SectionKind::LinkEdit
            } else {
                SectionKind::Unknown
            }
        }
    };

    Ok(ParsedSection {
        sectname: sect_name,
        segname: seg_name,
        addr: sect_addr,
        size: sect_size,
        flags: sect_flags,
        kind: sect_kind,
    })
}
