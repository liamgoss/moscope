# moscope — Mach-O static analysis and inspection toolkit

`moscope` is a Mach-O static analysis and inspection toolkit written in Rust. It focuses on accurate parsing and interpretation of Mach-O binaries, with an emphasis on universal binaries and architecture specific behavior. This project is intended for reverse engineering, security research, and low level inspection workflows.

## Current Features

- Detects Mach-O and universal (fat) binaries 
- Parses and displays the universal header and architecture table
- Enumerates contained architectures slices with resolved CPU types and subtypes
- Handles ARM, ARM64, and ARM64e subtype distinctions (including ABI related flags)
- Allows interactive selection of an architecture slice for further inspection
- Exposes functionality through a structured CLI
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
1: ARM (arm64e)
Select architecture index: 1

Mach-O Header Summary
----------------------------------------
  Magic        : 0xfeedfacf
  Architecture : ARM (arm64e)
  Word size    : 64-bit
  File type    : Demand Paged Executable File [[MH_EXECUTE]]
  Load cmds    : 20
  Cmds size    : 1800 bytes
  Flags        : 0x00200085
----------------------------------------


Load Commands Found:  20
----------------------------------------
 - LC_SEGMENT_64                  cmd=0x00000019 size=72
 - LC_SEGMENT_64                  cmd=0x00000019 size=472
 - LC_SEGMENT_64                  cmd=0x00000019 size=392
 - LC_SEGMENT_64                  cmd=0x00000019 size=232
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
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=104
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=88
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=56
 - LC_FUNCTION_STARTS             cmd=0x00000026 size=16
 - LC_DATA_IN_CODE                cmd=0x00000029 size=16
 - LC_CODE_SIGNATURE              cmd=0x0000001d size=16
----------------------------------------

```

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

- Load Command Enumeration
  - Parse and list all load commands present in the binary (COMPLETE)
  - Display command type, size, and relevant offsets (COMPLETE)
  - Preserve and report unknown or unsupported commands (COMPLETE)
- Segment and Section Inspection
  - Enumerate segments with virtual and file size information
  - Display memory protections and layout metadata
  - List contained sections with sizes and flag values
- String Extraction
  - Extract printable strings from the binary
  - Associate strings with sections if/where possible
  - Support basic filtering, sorting by length, case sensitivity, and pattern matching
- Dependency and Symbol Imports
  - List dependent dynamic libraries
  - Enumerate imported symbols
  - Distinguish weak imports from required imports

This project is intentionally deferring deeper runtime and platform specific features, including chained fixups, Objective-C runtime parsing, Swift metadata, pointer authentication analysis, entitlements, and code signature inspection. These areas will be revisited once core structural coverage is complete.
