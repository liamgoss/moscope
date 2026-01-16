// File Purpose: Enumerate Segments and Sections
// https://web.archive.org/web/20260107202245/https://developer.apple.com/library/archive/documentation/Performance/Conceptual/CodeFootprint/Articles/MachOOverview.html
// https://web.archive.org/web/20250912084041/https://medium.com/@travmath/understanding-the-mach-o-file-format-66cf0354e3f4
// https://github.com/aidansteele/osx-abi-macho-file-format-reference/blob/master/README.md#table-1-the-sections-of-a__textsegment

// NOTE: I have read through the above 3 resources and compiled what I believe to be the most important ones to know
/*
=======================================
==== Notable Segments and Sections ====
=======================================

__TEXT (Read + Execute)
    Executable code and read-only data. Typically shared across processes.

    __text
        Compiled machine instructions.

    __const
        Read-only constant data that does not require relocation.

    __cstring
        Null-terminated C string literals.
        Duplicate strings are typically coalesced by the linker.

    __stubs
        Small trampoline functions used for calling dynamically
        linked functions. Each stub typically jumps through a
        corresponding symbol pointer.

    __stub_helper
        Helper code used by dyld to resolve lazy symbols at runtime.

    __picsymbol_stub (legacy / transitional)
        Position-independent symbol stubs used by older toolchains.
        Largely superseded by __stubs / __stub_helper in modern binaries.


__DATA (Read + Write)
    Mutable data sections mapped into writable memory.

    __data
        Initialized global and static variables
        (e.g., `int a = 1;`, `static int b = 2;`).

    __const
        Constant data that requires relocation
        (e.g., `char * const p = "foo";`).

    __bss
        Zero-initialized globals and statics.
        Occupies virtual memory but has no backing bytes in the file.

    __common (legacy)
        Uninitialized external globals.
        Largely folded into __bss by modern toolchains.

    __la_symbol_ptr
        Lazy symbol pointers used for functions.
        Initially unresolved and fixed up by dyld on first call.

    __nl_symbol_ptr
        Non-lazy symbol pointers.
        Resolved by dyld at load time.

    __dyld
        Reserved section used internally by the dynamic linker.


__PAGEZERO
    - Unmapped region starting at virtual address 0
    - No read/write/execute permissions
    - Size is typically one page or more
    - Occupies no space in the file
    - Used to trap NULL pointer dereferences
*/

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

/*
pub fn classify_section(segment_name: &str, section_name: &str, flags: u32) -> SectionKind {
    use SectionKind::*;

    match flags & 0xFF {
        
    }
}

*/