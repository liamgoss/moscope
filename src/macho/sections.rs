// File Purpose: Enumerate Sections, Work with segments.rs
use crate::macho::constants::*;
use crate::macho::utils;
use crate::reporting::sections::SectionReport;
use std::error::Error;
use std::mem::size_of;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionKind {
    // Executable code
    Code,                       // __text
    // Indirect symbol consumers
    SymbolStubs,                // __TEXT,__stubs (S_SYMBOL_STUBS)
    LazySymbolPointers,         // __DATA,__la_symbol_ptr (S_LAZY_SYMBOL_POINTERS)
    NonLazySymbolPointers,      // __DATA,__nl_symbol_ptr (S_NON_LAZY_SYMBOL_POINTERS)
    GlobalOffsetTable,          // __DATA_CONST,__got
    // Data
    CString,                    // __cstring
    ConstData,                  // __const
    Data,                       // __data
    Bss,                        // __bss,
    // OBJC
    ObjCClass,
    ObjCMetaClass,
    ObjCSelectorRefs,
    ObjCMethodNames,
    ObjCMetadata,
    // Exceptions and Unwind
    Exception,                  // __exception
    Unwind,                     // __unwind_info
    // Init
    Init,                       // __mod_init_func
    // Debug & linkedit
    Debug,                      // __debug_*
    LinkEdit,                   // __LINKEDIT
    // Fallback
    Other,
    Unknown,
}

impl SectionKind {
    pub fn uses_indirect_symbols(&self) -> bool {
        matches!(
            self, 
            SectionKind::SymbolStubs |
            SectionKind::LazySymbolPointers | 
            SectionKind::NonLazySymbolPointers |
            SectionKind::GlobalOffsetTable
        )
    }
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
    pub offset: u32,
    pub addr: u64,          
    pub size: u64,         
    pub flags: u32,        
    pub kind: SectionKind, 
    // Adding reserved1 and 2 for indirect symbols and stubs
    pub reserved1: u32,
    pub reserved2: u32,
    pub reserved3: Option<u32>, // may or may not be present if not Section64 
}

impl ParsedSection {
    pub fn build_report(&self) -> SectionReport {
        SectionReport { 
            name: utils::byte_array_to_string(&self.sectname), 
            segment: utils::byte_array_to_string(&self.segname), 
            kind: format!("{:?}", self.kind), 
            addr: self.addr, 
            size: self.size 
        }
    }
}
pub fn classify_section(
    sect_name: [u8; 16],
    sect_type: u32,
    seg_name: [u8; 16],
) -> SectionKind {
    let stype = sect_type & SECTION_TYPE;

    // resolve by section type
    match stype {
        S_CSTRING_LITERALS                                      => return SectionKind::CString,
        S_ZEROFILL | S_GB_ZEROFILL                              => return SectionKind::Bss,
        S_SYMBOL_STUBS                                          => return SectionKind::SymbolStubs,
        S_LAZY_SYMBOL_POINTERS | S_LAZY_DYLUB_SYMBOL_POINTERS   => return SectionKind::LazySymbolPointers,
        S_NON_LAZY_SYMBOL_POINTERS                              => return SectionKind::NonLazySymbolPointers,
        S_MOD_INIT_FUNC_POINTERS | S_MOD_TERM_FUNC_POINTERS     => return SectionKind::Init,
        _ => {}
    }

    // resolve by segment + section name
    if stype == S_REGULAR {
        match (seg_name, sect_name) {
            // __TEXT
            (SEG_TEXT, SECT_TEXT) => SectionKind::Code,
            (SEG_TEXT, SECT_CONST) => SectionKind::ConstData,
            (SEG_TEXT, SECT_CSTRING) => SectionKind::CString,
            (SEG_TEXT, SECT_GCC_EXCEPT_TAB) => SectionKind::Exception,
            (SEG_TEXT, SECT_EH_FRAME) => SectionKind::Exception,
            (SEG_TEXT, SECT_UNWIND_INFO) => SectionKind::Unwind,
            (SEG_TEXT, SECT_INIT_OFFSETS) => SectionKind::Init,
            (SEG_TEXT, SECT_OBJC_METHNAME) => SectionKind::ObjCMethodNames,
            (SEG_TEXT, SECT_OBJC_STUBS) => SectionKind::SymbolStubs,

            // __DATA
            (SEG_DATA, SECT_DATA) => SectionKind::Data,
            (SEG_DATA, SECT_BSS) => SectionKind::Bss,
            (SEG_DATA, SECT_COMMON) => SectionKind::Bss,
            (SEG_DATA, SECT_OBJC_SELREFS) => SectionKind::ObjCSelectorRefs,
            (SEG_DATA, SECT_OBJC_CLASSREFS) => SectionKind::ObjCClass,

            // __DATA_CONST
            (SEG_DATA_CONST, SECT_CONST) => SectionKind::ConstData,
            (SEG_DATA_CONST, SECT_GOT) => SectionKind::GlobalOffsetTable,
            (SEG_DATA_CONST, SECT_CFSTRING) => SectionKind::ObjCMetadata,
            (SEG_DATA_CONST, SECT_OBJC_IMAGEINFO) => SectionKind::ObjCMetadata,
            (SEG_DATA_CONST, SECT_OBJC_CLASSLIST) => SectionKind::ObjCClass,
            (SEG_DATA_CONST, SECT_OBJC_PROTLIST) => SectionKind::ObjCMetadata,
            (SEG_DATA_CONST, SECT_OBJC_SELREFS) => SectionKind::ObjCSelectorRefs,

            // __AUTH / __AUTH_CONST            
            (SEG_AUTH_CONST, SECT_AUTH_GOT) => SectionKind::GlobalOffsetTable,
            (SEG_AUTH_CONST, SECT_AUTH_PTR) => SectionKind::NonLazySymbolPointers,
            (SEG_AUTH_CONST, SECT_CONST) => SectionKind::ConstData,
            (SEG_AUTH, SECT_DATA) => SectionKind::Data,
            (SEG_AUTH, SECT_OBJC_DATA) => SectionKind::ObjCClass,

            // __LINKEDIT
            (SEG_LINKEDIT, _) => SectionKind::LinkEdit,

            _ => SectionKind::Other,
        }
    } else {
        // fallback
        if seg_name == SEG_LINKEDIT {
            SectionKind::LinkEdit
        } else {
            SectionKind::Unknown
        }
    }
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
    let sect_fileoff: u32 = utils::bytes_to(is_be, &data[sect_offset + 48 .. sect_offset + 52])?;
    let sect_flags = utils::bytes_to(is_be, &data[sect_offset + 64..])?;
    let reserved1: u32 = utils::bytes_to(is_be, &data[sect_offset + 68 ..])?;
    let reserved2: u32 = utils::bytes_to(is_be, &data[sect_offset + 72 ..])?;
    let reserved3: u32 = utils::bytes_to(is_be, &data[sect_offset + 76 ..])?;

    
    // classify
    let sect_type = sect_flags & SECTION_TYPE;
    let sect_kind = classify_section(sect_name, sect_type, seg_name);

    Ok(ParsedSection {
        sectname: sect_name,
        segname: seg_name,
        offset: sect_fileoff,
        addr: sect_addr,
        size: sect_size,
        flags: sect_flags,
        kind: sect_kind,
        reserved1: reserved1,
        reserved2: reserved2,
        reserved3: Some(reserved3),
    })
}



pub fn read_section32_from_bytes(
    data: &[u8],
    is_be: bool,
    sect_offset: usize,
) -> Result<ParsedSection, Box<dyn Error>> {

    // bounds check
    if sect_offset + size_of::<Section>() > data.len() {
        println!("sect_offset {:?} + {:?} exceeds {:?}", sect_offset, size_of::<Section>(), data.len());
        return Err("Section out of bounds".into());
    }
    let sect_name: [u8; 16] = data[sect_offset .. sect_offset + 16].try_into()?;
    let seg_name: [u8; 16] = data[sect_offset + 16 .. sect_offset + 32].try_into()?;
    let sect_addr_32: u32 = utils::bytes_to(is_be, &data[sect_offset + 32 ..])?;
    let sect_size_32: u32 = utils::bytes_to(is_be, &data[sect_offset + 36 ..])?;
    let sect_flags: u32 = utils::bytes_to(is_be, &data[sect_offset + 56 ..])?;
    let reserved1: u32 = utils::bytes_to(is_be, &data[sect_offset + 60 ..])?;
    let reserved2: u32 = utils::bytes_to(is_be, &data[sect_offset + 64 ..])?;

    // widen to 64-bit for ParsedSection
    let sect_addr = sect_addr_32 as u64;
    let sect_size = sect_size_32 as u64;

    // classify
    let sect_type = sect_flags & SECTION_TYPE;
    let sect_kind = classify_section(sect_name, sect_type, seg_name);

    Ok(ParsedSection {
        sectname: sect_name,
        segname: seg_name,
        offset: sect_offset as u32,
        addr: sect_addr,
        size: sect_size,
        flags: sect_flags,
        kind: sect_kind,
        reserved1: reserved1,
        reserved2: reserved2,
        reserved3: None,
    })
}
