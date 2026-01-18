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
# Inspect a thin or fat Mach-O binary with interactive text output (with color)
moscope /path/to/target_binary

# Inspect a thin or fat Mach-O binary with interactive text output (without color)
moscope /path/to/target_binary --no-color

# Inspect and output JSON (automatically outputs all architectures if fat)
moscope /path/to/target_binary --format json

# The output is colored by default but can be disabled by using `--no-color`. If `--format json` is used, coloring is disabled automatically

```

> This project comes with 3 sample binaries (x86_64, ARM64, FAT) to test with.  

> The sample binaries can be found in `tests/samples/` or can be compiled yourself using `/tests/samples/src/hello.cpp`

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
- Dependency and Symbol Imports *(WIP)*
  - List dependent dynamic libraries
  - Enumerate imported symbols
  - Distinguish weak imports from required imports

This project is intentionally deferring deeper runtime and platform specific features, including chained fixups, Objective-C runtime parsing, Swift metadata, pointer authentication analysis, entitlements, and code signature inspection. These areas will be revisited once core structural coverage is complete.


## Example Output (Thin Binary, Interactive Version)
```
Mach-O Header Summary
----------------------------------------
  Magic        : 0xfeedfacf
  Architecture : ARM (arm64 (ARM64_ALL))
  Word size    : 64-bit
  File type    : Demand Paged Executable File [[MH_EXECUTE]]
  Load cmds    : 37
  Cmds size    : 3736 bytes
  Flags        : NOUNDEFS, DYLDLINK, TWOLEVEL, BINDS_TO_WEAK, PIE
----------------------------------------

Mach-O Report:

Segments Summary
----------------------------------------

Segment __PAGEZERO
  VM range   : 0x0000000000000000 - 0x0000000100000000 (0x100000000 bytes)
  File range : 0x00000000 - 0x00000000 (0x0 bytes)
  Protections: ---
  Sections   : 0

Segment __TEXT
  VM range   : 0x0000000100000000 - 0x0000000100404000 (0x404000 bytes)
  File range : 0x00000000 - 0x00404000 (0x404000 bytes)
  Protections: R-X
  Sections   : 11
    - __text           Code size=0x2c1548
    - __stubs          Stub size=0x7614
    - __objc_stubs     Stub size=0x2e0
    - __init_offsets   Unknown size=0x12c
    - __gcc_except_tab Exception size=0x3614c
    - __const          ConstData size=0xc7dfe
    - __cstring        CString size=0x25c9b
    - __objc_methname  CString size=0x181
    - __info_plist     Other          size=0x70c
    - __unwind_info    Unwind size=0xfce8
    - __eh_frame       Unknown size=0x38

Segment __DATA_CONST
  VM range   : 0x0000000100404000 - 0x0000000100448000 (0x44000 bytes)
  File range : 0x00404000 - 0x00448000 (0x44000 bytes)
  Protections: RW-
  Sections   : 4
    - __got            SymbolPointer size=0x5178
    - __const          ConstData size=0x3c648
    - __cfstring       ObjC size=0x1c0
    - __objc_imageinfo ObjCMetadata size=0x8

Segment __DATA
  VM range   : 0x0000000100448000 - 0x000000010045c000 (0x14000 bytes)
  File range : 0x00448000 - 0x00458000 (0x10000 bytes)
  Protections: RW-
  Sections   : 5
    - __objc_selrefs   Unknown size=0xd8
    - __objc_classrefs ObjC size=0x18
    - __data           Data size=0xcd68
    - __bss            Bss size=0x4df8
    - __common         Bss size=0x1c80

Segment __LINKEDIT
  VM range   : 0x000000010045c000 - 0x00000001004b4000 (0x58000 bytes)
  File range : 0x00458000 - 0x004ada50 (0x55a50 bytes)
  Protections: R--
  Sections   : 0
----------------------------------------


Dynamic Libraries
----------------------------------------
[LOAD    ] @rpath/QtPrintSupport.framework/Versions/5/QtPrintSupport
[LOAD    ] @rpath/QtSvg.framework/Versions/5/QtSvg
[LOAD    ] @rpath/QtWidgets.framework/Versions/5/QtWidgets
[LOAD    ] @rpath/QtMacExtras.framework/Versions/5/QtMacExtras
[LOAD    ] @rpath/QtGui.framework/Versions/5/QtGui
[LOAD    ] /System/Library/Frameworks/AppKit.framework/Versions/C/AppKit
[LOAD    ] /System/Library/Frameworks/Metal.framework/Versions/A/Metal
[LOAD    ] @rpath/QtCore.framework/Versions/5/QtCore
[LOAD    ] /System/Library/Frameworks/DiskArbitration.framework/Versions/A/DiskArbitration
[LOAD    ] /System/Library/Frameworks/IOKit.framework/Versions/A/IOKit
[LOAD    ] /System/Library/Frameworks/OpenGL.framework/Versions/A/OpenGL
[LOAD    ] /System/Library/Frameworks/AGL.framework/Versions/A/AGL
[LOAD    ] @rpath/libida.dylib
[LOAD    ] /usr/lib/libc++.1.dylib
[LOAD    ] /usr/lib/libSystem.B.dylib
[LOAD    ] /System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation
[LOAD    ] /System/Library/Frameworks/Foundation.framework/Versions/C/Foundation
[LOAD    ] /usr/lib/libobjc.A.dylib

RPATHs
----------------------------------------
[RPATH] @executable_path/
[RPATH] @executable_path/../Frameworks

Load Commands Found:  37
----------------------------------------
 - LC_SEGMENT_64                  cmd=0x00000019 size=72
 - LC_SEGMENT_64                  cmd=0x00000019 size=952
 - LC_SEGMENT_64                  cmd=0x00000019 size=392
 - LC_SEGMENT_64                  cmd=0x00000019 size=472
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
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=88
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=64
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=72
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=80
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=64
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=88
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=88
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=72
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=104
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=88
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=88
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=80
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=48
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=48
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=56
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=104
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=96
 - LC_LOAD_DYLIB                  cmd=0x0000000c size=56
 - LC_RPATH                       cmd=0x8000001c size=32
 - LC_RPATH                       cmd=0x8000001c size=48
 - LC_FUNCTION_STARTS             cmd=0x00000026 size=16
 - LC_DATA_IN_CODE                cmd=0x00000029 size=16
 - LC_CODE_SIGNATURE              cmd=0x0000001d size=16
----------------------------------------

```


## Example Output (Thin Binary, JSON Output)
```JSON
{
  "is_fat": false,
  "architectures": [
    {
      "cpu_type": "ARM",
      "cpu_subtype": "arm64 (ARM64_ALL)",
      "header": {
        "magic": 4277009103,
        "file_type": "Demand Paged Executable File [[MH_EXECUTE]]",
        "cpu_type": "ARM",
        "cpu_subtype": "arm64 (ARM64_ALL)",
        "ncmds": 37,
        "sizeofcmds": 3736,
        "flags": [
          "NOUNDEFS",
          "DYLDLINK",
          "TWOLEVEL",
          "BINDS_TO_WEAK",
          "PIE"
        ]
      },
      "load_commands": [
        {
          "command": "LC_SEGMENT_64",
          "cmd": 25,
          "size": 72
        },
        {
          "command": "LC_SEGMENT_64",
          "cmd": 25,
          "size": 952
        },
        {
          "command": "LC_SEGMENT_64",
          "cmd": 25,
          "size": 392
        },
        {
          "command": "LC_SEGMENT_64",
          "cmd": 25,
          "size": 472
        },
        {
          "command": "LC_SEGMENT_64",
          "cmd": 25,
          "size": 72
        },
        {
          "command": "LC_DYLD_CHAINED_FIXUPS",
          "cmd": 2147483700,
          "size": 16
        },
        {
          "command": "LC_DYLD_EXPORTS_TRIE",
          "cmd": 2147483699,
          "size": 16
        },
        {
          "command": "LC_SYMTAB",
          "cmd": 2,
          "size": 24
        },
        {
          "command": "LC_DSYMTAB",
          "cmd": 11,
          "size": 80
        },
        {
          "command": "LC_LOAD_DYLINKER",
          "cmd": 14,
          "size": 32
        },
        {
          "command": "LC_UUID",
          "cmd": 27,
          "size": 24
        },
        {
          "command": "LC_BUILD_VERSION",
          "cmd": 50,
          "size": 32
        },
        {
          "command": "LC_SOURCE_VERSION",
          "cmd": 42,
          "size": 16
        },
        {
          "command": "LC_MAIN",
          "cmd": 2147483688,
          "size": 24
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 88
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 64
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 72
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 80
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 64
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 88
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 88
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 72
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 104
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 88
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 88
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 80
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 48
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 48
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 56
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 104
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 96
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 56
        },
        {
          "command": "LC_RPATH",
          "cmd": 2147483676,
          "size": 32
        },
        {
          "command": "LC_RPATH",
          "cmd": 2147483676,
          "size": 48
        },
        {
          "command": "LC_FUNCTION_STARTS",
          "cmd": 38,
          "size": 16
        },
        {
          "command": "LC_DATA_IN_CODE",
          "cmd": 41,
          "size": 16
        },
        {
          "command": "LC_CODE_SIGNATURE",
          "cmd": 29,
          "size": 16
        }
      ],
      "segments": [
        {
          "name": "__PAGEZERO",
          "vmaddr": 0,
          "vmsize": 4294967296,
          "fileoff": 0,
          "filesize": 0,
          "maxprot": "---",
          "initprot": "---",
          "sections": []
        },
        {
          "name": "__TEXT",
          "vmaddr": 4294967296,
          "vmsize": 4210688,
          "fileoff": 0,
          "filesize": 4210688,
          "maxprot": "R-X",
          "initprot": "R-X",
          "sections": [
            {
              "name": "__text",
              "segment": "__TEXT",
              "kind": "Code",
              "addr": 4294995708,
              "size": 2889032
            },
            {
              "name": "__stubs",
              "segment": "__TEXT",
              "kind": "Stub",
              "addr": 4297884740,
              "size": 30228
            },
            {
              "name": "__objc_stubs",
              "segment": "__TEXT",
              "kind": "Stub",
              "addr": 4297914968,
              "size": 736
            },
            {
              "name": "__init_offsets",
              "segment": "__TEXT",
              "kind": "Unknown",
              "addr": 4297915704,
              "size": 300
            },
            {
              "name": "__gcc_except_tab",
              "segment": "__TEXT",
              "kind": "Exception",
              "addr": 4297916004,
              "size": 221516
            },
            {
              "name": "__const",
              "segment": "__TEXT",
              "kind": "ConstData",
              "addr": 4298137520,
              "size": 818686
            },
            {
              "name": "__cstring",
              "segment": "__TEXT",
              "kind": "CString",
              "addr": 4298956206,
              "size": 154779
            },
            {
              "name": "__objc_methname",
              "segment": "__TEXT",
              "kind": "CString",
              "addr": 4299110985,
              "size": 385
            },
            {
              "name": "__info_plist",
              "segment": "__TEXT",
              "kind": "Other",
              "addr": 4299111370,
              "size": 1804
            },
            {
              "name": "__unwind_info",
              "segment": "__TEXT",
              "kind": "Unwind",
              "addr": 4299113176,
              "size": 64744
            },
            {
              "name": "__eh_frame",
              "segment": "__TEXT",
              "kind": "Unknown",
              "addr": 4299177920,
              "size": 56
            }
          ]
        },
        {
          "name": "__DATA_CONST",
          "vmaddr": 4299177984,
          "vmsize": 278528,
          "fileoff": 4210688,
          "filesize": 278528,
          "maxprot": "RW-",
          "initprot": "RW-",
          "sections": [
            {
              "name": "__got",
              "segment": "__DATA_CONST",
              "kind": "SymbolPointer",
              "addr": 4299177984,
              "size": 20856
            },
            {
              "name": "__const",
              "segment": "__DATA_CONST",
              "kind": "ConstData",
              "addr": 4299198840,
              "size": 247368
            },
            {
              "name": "__cfstring",
              "segment": "__DATA_CONST",
              "kind": "ObjC",
              "addr": 4299446208,
              "size": 448
            },
            {
              "name": "__objc_imageinfo",
              "segment": "__DATA_CONST",
              "kind": "ObjCMetadata",
              "addr": 4299446656,
              "size": 8
            }
          ]
        },
        {
          "name": "__DATA",
          "vmaddr": 4299456512,
          "vmsize": 81920,
          "fileoff": 4489216,
          "filesize": 65536,
          "maxprot": "RW-",
          "initprot": "RW-",
          "sections": [
            {
              "name": "__objc_selrefs",
              "segment": "__DATA",
              "kind": "Unknown",
              "addr": 4299456512,
              "size": 216
            },
            {
              "name": "__objc_classrefs",
              "segment": "__DATA",
              "kind": "ObjC",
              "addr": 4299456728,
              "size": 24
            },
            {
              "name": "__data",
              "segment": "__DATA",
              "kind": "Data",
              "addr": 4299456752,
              "size": 52584
            },
            {
              "name": "__bss",
              "segment": "__DATA",
              "kind": "Bss",
              "addr": 4299509344,
              "size": 19960
            },
            {
              "name": "__common",
              "segment": "__DATA",
              "kind": "Bss",
              "addr": 4299529312,
              "size": 7296
            }
          ]
        },
        {
          "name": "__LINKEDIT",
          "vmaddr": 4299538432,
          "vmsize": 360448,
          "fileoff": 4554752,
          "filesize": 350800,
          "maxprot": "R--",
          "initprot": "R--",
          "sections": []
        }
      ],
      "dylibs": [
        {
          "path": "@rpath/QtPrintSupport.framework/Versions/5/QtPrintSupport",
          "timestamp": 2,
          "current_version": 331523,
          "compatibility_version": 331520,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 88
          }
        },
        {
          "path": "@rpath/QtSvg.framework/Versions/5/QtSvg",
          "timestamp": 2,
          "current_version": 331522,
          "compatibility_version": 331520,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 64
          }
        },
        {
          "path": "@rpath/QtWidgets.framework/Versions/5/QtWidgets",
          "timestamp": 2,
          "current_version": 331523,
          "compatibility_version": 331520,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 72
          }
        },
        {
          "path": "@rpath/QtMacExtras.framework/Versions/5/QtMacExtras",
          "timestamp": 2,
          "current_version": 331522,
          "compatibility_version": 331520,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 80
          }
        },
        {
          "path": "@rpath/QtGui.framework/Versions/5/QtGui",
          "timestamp": 2,
          "current_version": 331523,
          "compatibility_version": 331520,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 64
          }
        },
        {
          "path": "/System/Library/Frameworks/AppKit.framework/Versions/C/AppKit",
          "timestamp": 2,
          "current_version": 163003497,
          "compatibility_version": 2949120,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 88
          }
        },
        {
          "path": "/System/Library/Frameworks/Metal.framework/Versions/A/Metal",
          "timestamp": 2,
          "current_version": 22483712,
          "compatibility_version": 65536,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 88
          }
        },
        {
          "path": "@rpath/QtCore.framework/Versions/5/QtCore",
          "timestamp": 2,
          "current_version": 331523,
          "compatibility_version": 331520,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 72
          }
        },
        {
          "path": "/System/Library/Frameworks/DiskArbitration.framework/Versions/A/DiskArbitration",
          "timestamp": 2,
          "current_version": 65536,
          "compatibility_version": 65536,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 104
          }
        },
        {
          "path": "/System/Library/Frameworks/IOKit.framework/Versions/A/IOKit",
          "timestamp": 2,
          "current_version": 18022400,
          "compatibility_version": 65536,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 88
          }
        },
        {
          "path": "/System/Library/Frameworks/OpenGL.framework/Versions/A/OpenGL",
          "timestamp": 2,
          "current_version": 65536,
          "compatibility_version": 65536,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 88
          }
        },
        {
          "path": "/System/Library/Frameworks/AGL.framework/Versions/A/AGL",
          "timestamp": 2,
          "current_version": 65536,
          "compatibility_version": 65536,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 80
          }
        },
        {
          "path": "@rpath/libida.dylib",
          "timestamp": 2,
          "current_version": 0,
          "compatibility_version": 0,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 48
          }
        },
        {
          "path": "/usr/lib/libc++.1.dylib",
          "timestamp": 2,
          "current_version": 111476485,
          "compatibility_version": 65536,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 48
          }
        },
        {
          "path": "/usr/lib/libSystem.B.dylib",
          "timestamp": 2,
          "current_version": 88176642,
          "compatibility_version": 65536,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 56
          }
        },
        {
          "path": "/System/Library/Frameworks/CoreFoundation.framework/Versions/A/CoreFoundation",
          "timestamp": 2,
          "current_version": 164036864,
          "compatibility_version": 9830400,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 104
          }
        },
        {
          "path": "/System/Library/Frameworks/Foundation.framework/Versions/C/Foundation",
          "timestamp": 2,
          "current_version": 164036864,
          "compatibility_version": 19660800,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 96
          }
        },
        {
          "path": "/usr/lib/libobjc.A.dylib",
          "timestamp": 2,
          "current_version": 14942208,
          "compatibility_version": 65536,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 56
          }
        }
      ],
      "rpaths": [
        {
          "source_lc": "LC_RPATH",
          "path": "@executable_path/"
        },
        {
          "source_lc": "LC_RPATH",
          "path": "@executable_path/../Frameworks"
        }
      ]
    }
  ]
}

```
