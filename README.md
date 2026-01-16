# moscope — Mach-O static analysis and inspection toolkit

`moscope` is a Mach-O static analysis and inspection toolkit written in Rust. It focuses on accurate parsing and interpretation of Mach-O binaries, with an emphasis on universal binaries and architecture specific behavior. This project is intended for reverse engineering, security research, and low level inspection workflows.

## Current Features

- Detects Mach-O and universal (fat) binaries
- Parses and displays the universal (fat) header and architecture table
- Enumerates contained architecture slices with resolved CPU types and subtypes
- Handles ARM, ARM64, and ARM64e subtype distinctions, including ABI-related flags
- Allows interactive selection of an architecture slice for further inspection
- Parses and summarizes the Mach-O header
  - Magic value, word size, file type
  - Load command count and total command size
  - Header flags with symbolic decoding
- Enumerates all load commands
  - Displays command type, raw command ID, and command size
  - Preserves and reports unknown or unsupported commands without failure
- Enumerates Mach-O segments
  - Displays virtual memory ranges and file-backed ranges
  - Shows initial memory protections
  - Identifies standard segments (__TEXT, __DATA, __DATA_CONST, __LINKEDIT, __PAGEZERO)
- Enumerates sections within each segment
  - Displays section names and sizes
  - Classifies sections into semantic categories (code, data, BSS, stubs, symbol pointers, ObjC metadata, unwind info, exceptions, etc.)
  - Correctly handles modern macOS conventions using section type, attributes, and name-based classification
- Provides structured, human-readable output suitable for reverse engineering and binary inspection
- Exposes functionality through a structured command-line interface (with ANSI coloring, *optionally disabled with the `--no-color` flag*)
---

## Example Usage (Expected to change upon release)

```bash
moscope /path/to/target_binary
```

> This project comes with 3 sample binaries (x86_64, ARM64, FAT) to test with.  

> The sample binaries can be found in `tests/samples/` or can be compiled yourself using `/tests/samples/src/hello.cpp`

### Example Output
```
Checking for universal binary...
Fat binary detected:
Available architectures:
0: x86 (x86_64)
1: ARM (arm64)
Select architecture index: 1

Mach-O Header Summary
----------------------------------------
  Magic        : 0xfeedfacf
  Architecture : ARM (arm64 (ARM64_ALL))
  Word size    : 64-bit
  File type    : Demand Paged Executable File [[MH_EXECUTE]]
  Load cmds    : 18
  Cmds size    : 1184 bytes
  Flags        : NOUNDEFS, DYLDLINK, TWOLEVEL, BINDS_TO_WEAK, PIE
----------------------------------------


Load Commands Found:  18
----------------------------------------
 - LC_SEGMENT_64                  cmd=0x00000019 size=72
 - LC_SEGMENT_64                  cmd=0x00000019 size=472
 - LC_SEGMENT_64                  cmd=0x00000019 size=152
 - LC_SEGMENT_64                  cmd=0x00000019 size=72
 - LC_DYLD_CHAINED_FIXUPS         cmd=0x80000034 size=16
 - LC_DYLD_EXPORTS_TRIE           cmd=0x80000033 size=16
 - LC_SYMTAB                      cmd=0x00000002 size=24
 - LC_DSYMTAB                     cmd=0x0000000b size=80
 - LC_LOAD_DYLINKER               cmd=0x0000000e size=32
 - LC_UUID                        cmd=0x0000001b size=24
 - LC_BUILD_VERSION               cmd=0x00000032 size=32
 - LC_SOURCE_VERSION              cmd=0x0000002a size=16
 - LC_MAIN                        cmd=0x80000028 size=24
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=48
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=56
 - LC_FUNCTION_STARTS             cmd=0x00000026 size=16
 - LC_DATA_IN_CODE                cmd=0x00000029 size=16
 - LC_CODE_SIGNATURE              cmd=0x0000001d size=16
----------------------------------------


Segments Summary
----------------------------------------

Segment __PAGEZERO
  VM range   : 0x0000000000000000 - 0x0000000100000000 (0x100000000 bytes)
  File range : 0x00000000 - 0x00000000 (0x0 bytes)
  Protections: ---
  Sections   : 0

Segment __TEXT
  VM range   : 0x0000000100000000 - 0x0000000100004000 (0x4000 bytes)
  File range : 0x00000000 - 0x00004000 (0x4000 bytes)
  Protections: R-X
  Sections   : 5
    - __text           Code size=0x480
    - __stubs          Stub size=0xf0
    - __gcc_except_tab Exception size=0x80
    - __cstring        CString size=0x1a
    - __unwind_info    Unwind size=0xa0

Segment __DATA_CONST
  VM range   : 0x0000000100004000 - 0x0000000100008000 (0x4000 bytes)
  File range : 0x00004000 - 0x00008000 (0x4000 bytes)
  Protections: RW-
  Sections   : 1
    - __got            SymbolPointer size=0xd0

Segment __LINKEDIT
  VM range   : 0x0000000100008000 - 0x000000010000c000 (0x4000 bytes)
  File range : 0x00008000 - 0x00008dd8 (0xdd8 bytes)
  Protections: R--
  Sections   : 0
----------------------------------------

```

> The output is colored by default but can be disabled by using `--no-color`

---

## Project Status

`moscope` is under active development. Internal APIs, function signatures, output formats, program structure, and program invocation are expected to change as additional Mach-O features and analysis stages are implemented.


## Requirements

- Rust (stable)

### Platform notes

`moscope` is a pure Rust Mach-O parsing tool and can be built and used on any platform
(Linux, macOS, or Windows). 

`moscope` does not currently execute the target binary.

## Future Work

Development is focused on incrementally expanding coverage of the Mach-O format while keeping parsing behavior fault tolerant. Near term goals include:

- String Extraction
  - Extract printable strings from the binary
  - Associate strings with sections if/where possible
  - Support basic filtering, sorting by length, case sensitivity, and pattern matching
- Dependency and Symbol Imports
  - List dependent dynamic libraries
  - Enumerate imported symbols
  - Distinguish weak imports from required imports

This project is intentionally deferring deeper runtime and platform specific features, including chained fixups, Objective-C runtime parsing, Swift metadata, pointer authentication analysis, entitlements, and code signature inspection. These areas will be revisited once core structural coverage is complete.
