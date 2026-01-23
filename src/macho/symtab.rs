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
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct nlist {
    n_strx_offset: u32, // index into the string table
    n_type: u8, // type flag
    n_sect: u8, // section number or NO_SECT
    n_desc: u16, // 
    n_value: u32, // value of this symbol or stab offset
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]struct nlist_64 {
    n_strx_offset: u32, // index into the string table
    n_type: u8, // type flag
    n_sect: u8, // section number or NO_SECT
    n_desc: u16, // 
    n_value: u64, // value of this symbol or stab offset
}
