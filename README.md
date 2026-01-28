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
- Parses and summarized symbols
  - Extracts all symbol table entries from LC_SYMTAB
  - Classifies symbols (external, debug, etc.)
  - Displays symbol names, types, and linkage
- Provides structured, human-readable output suitable for reverse engineering and binary inspection
- Exposes functionality through a structured command-line interface (with ANSI coloring, *optionally disabled with the `--no-color` flag*)
---

## Example Usage (Expected to change upon release)

```bash
# Inspect a thin or fat Mach-O binary with interactive colored text output (default)
moscope /path/to/target_binary

# Inspect a binary without color output
moscope /path/to/target_binary --no-color

# Inspect and output JSON instead of text (good for automation or parsing)
moscope /path/to/target_binary --format json

# Limit the number of symbol entries displayed (e.g., top 10 symbols)
moscope /path/to/target_binary --symbol-limit 10

# Adjust string extraction behavior
#   --min-string-length: ignore strings shorter than the specified length (default: 4)
#   --max-num-strings: limit the total number of extracted strings
moscope /path/to/target_binary --min-string-length 6 --max-num-strings 100

# Disable specific sections of the report
#   --no-symbols: skip symbol table output
#   --no-segments: skip segment information
#   --no-loadcmds: skip load command information
#   --no-header: skip Mach-O header output
moscope /path/to/target_binary --no-symbols --no-segments

# Combine flags for customized output
moscope /path/to/target_binary --format json --symbol-limit 50 --no-loadcmds
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

- Demangling
- Output limiting (only output first X number of symbols, strings, etc.)
- String Extraction
  - Extract printable strings from the binary
  - Associate strings with sections if/where possible
  - Support basic filtering, sorting by length, case sensitivity, and pattern matching


This project is intentionally deferring deeper runtime and platform specific features, including chained fixups, Objective-C runtime parsing, Swift metadata, pointer authentication analysis, entitlements, and code signature inspection. These areas will be revisited once core structural coverage is complete.


## Example Output (Thin Binary, Interactive Version)
```
Mach-O Report:

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


Symbols
----------------------------------------
[SECT] EXT                __mh_execute_header
[UNDEF] EXT                _CFBundleCopyExecutableURL
[UNDEF] EXT                _CFBundleCreate
[UNDEF] EXT                _CFRelease
[UNDEF] EXT                _CFStringCreateWithCString
[UNDEF] EXT                _CFStringCreateWithFormat
[UNDEF] EXT                _CFStringGetCString
[UNDEF] EXT                _CFURLCopyAbsoluteURL
[UNDEF] EXT                _CFURLCopyFileSystemPath
[UNDEF] EXT                _CFURLCreateWithFileSystemPath
[UNDEF] EXT                _NSApp
[UNDEF] EXT                _NSRunCriticalAlertPanel
[UNDEF] EXT                _OBJC_CLASS_$_NSBundle
[UNDEF] EXT                _OBJC_CLASS_$_NSMenuItem
[UNDEF] EXT                _OBJC_CLASS_$_NSRunningApplication
[UNDEF] EXT                __DefaultRuneLocale
[UNDEF] EXT                __Unwind_Resume
[UNDEF] EXT                __ZN2QT10QArrayData10deallocateEPS0_mm
[UNDEF] EXT                __ZN2QT10QArrayData11shared_nullE
[UNDEF] EXT                __ZN2QT10QArrayData8allocateEmmmNS_6QFlagsINS0_16AllocationOptionEEE
[UNDEF] EXT                __ZN2QT10QBoxLayout10setSpacingEi
[UNDEF] EXT                __ZN2QT10QBoxLayout10setStretchEii
[UNDEF] EXT                __ZN2QT10QBoxLayout12insertWidgetEiPNS_7QWidgetEiNS_6QFlagsINS_2Qt13AlignmentFlagEEE
[UNDEF] EXT                __ZN2QT10QBoxLayout13addSpacerItemEPNS_11QSpacerItemE
[UNDEF] EXT                __ZN2QT10QBoxLayout13insertStretchEii
.... [ABRIDGED OUTPUT]

```


## Example Output (Thin Binary, JSON Output)
```JSON
{
  "is_fat": true,
  "architectures": [
    {
      "cpu_type": "x86",
      "cpu_subtype": "x86_64",
      "header": {
        "magic": 4277009103,
        "file_type": "Demand Paged Executable File [[MH_EXECUTE]]",
        "cpu_type": "x86",
        "cpu_subtype": "x86_64",
        "ncmds": 17,
        "sizeofcmds": 1168,
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
          "size": 472
        },
        {
          "command": "LC_SEGMENT_64",
          "cmd": 25,
          "size": 152
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
          "size": 48
        },
        {
          "command": "LC_LOAD_DYLIB",
          "cmd": 12,
          "size": 56
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
          "vmsize": 4096,
          "fileoff": 0,
          "filesize": 4096,
          "maxprot": "R-X",
          "initprot": "R-X",
          "sections": [
            {
              "name": "__text",
              "segment": "__TEXT",
              "kind": "Code",
              "addr": 4294968568,
              "size": 1029
            },
            {
              "name": "__stubs",
              "segment": "__TEXT",
              "kind": "Stub",
              "addr": 4294969598,
              "size": 120
            },
            {
              "name": "__gcc_except_tab",
              "segment": "__TEXT",
              "kind": "Exception",
              "addr": 4294969720,
              "size": 124
            },
            {
              "name": "__cstring",
              "segment": "__TEXT",
              "kind": "CString",
              "addr": 4294969844,
              "size": 26
            },
            {
              "name": "__unwind_info",
              "segment": "__TEXT",
              "kind": "Unwind",
              "addr": 4294969872,
              "size": 168
            }
          ]
        },
        {
          "name": "__DATA_CONST",
          "vmaddr": 4294971392,
          "vmsize": 4096,
          "fileoff": 4096,
          "filesize": 4096,
          "maxprot": "RW-",
          "initprot": "RW-",
          "sections": [
            {
              "name": "__got",
              "segment": "__DATA_CONST",
              "kind": "SymbolPointer",
              "addr": 4294971392,
              "size": 208
            }
          ]
        },
        {
          "name": "__LINKEDIT",
          "vmaddr": 4294975488,
          "vmsize": 4096,
          "fileoff": 8192,
          "filesize": 3128,
          "maxprot": "R--",
          "initprot": "R--",
          "sections": []
        }
      ],
      "dylibs": [
        {
          "path": "/usr/lib/libc++.1.dylib",
          "timestamp": 2,
          "current_version": 131088128,
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
          "current_version": 88866816,
          "compatibility_version": 65536,
          "kind": "LOAD",
          "load_command": {
            "command": "LC_LOAD_DYLIB",
            "cmd": 12,
            "size": 56
          }
        }
      ],
      "rpaths": [],
      "symbols": [
        {
          "name": "__ZNSt3__124__put_character_sequenceB8ne200100IcNS_11char_traitsIcEEEERNS_13basic_ostreamIT_T0_EES7_PKS4_m",
          "value": 4294968720,
          "kind": "SECT",
          "section": 1,
          "external": false,
          "debug": false
        },
        {
          "name": "__ZNSt3__116__pad_and_outputB8ne200100IcNS_11char_traitsIcEEEENS_19ostreambuf_iteratorIT_T0_EES6_PKS4_S8_S8_RNS_8ios_baseES4_",
          "value": 4294969056,
          "kind": "SECT",
          "section": 1,
          "external": false,
          "debug": false
        },
        {
          "name": "___clang_call_terminate",
          "value": 4294969323,
          "kind": "SECT",
          "section": 1,
          "external": false,
          "debug": false
        },
        {
          "name": "__ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEC2B8ne200100Emc",
          "value": 4294969338,
          "kind": "SECT",
          "section": 1,
          "external": false,
          "debug": false
        },
        {
          "name": "__ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEE20__throw_length_errorB8ne200100Ev",
          "value": 4294969470,
          "kind": "SECT",
          "section": 1,
          "external": false,
          "debug": false
        },
        {
          "name": "__ZNSt3__120__throw_length_errorB8ne200100EPKc",
          "value": 4294969486,
          "kind": "SECT",
          "section": 1,
          "external": false,
          "debug": false
        },
        {
          "name": "__ZNSt12length_errorC1B8ne200100EPKc",
          "value": 4294969562,
          "kind": "SECT",
          "section": 1,
          "external": false,
          "debug": false
        },
        {
          "name": "GCC_except_table0",
          "value": 4294969720,
          "kind": "SECT",
          "section": 3,
          "external": false,
          "debug": false
        },
        {
          "name": "GCC_except_table1",
          "value": 4294969740,
          "kind": "SECT",
          "section": 3,
          "external": false,
          "debug": false
        },
        {
          "name": "GCC_except_table2",
          "value": 4294969808,
          "kind": "SECT",
          "section": 3,
          "external": false,
          "debug": false
        },
        {
          "name": "GCC_except_table6",
          "value": 4294969828,
          "kind": "SECT",
          "section": 3,
          "external": false,
          "debug": false
        },
        {
          "name": "__mh_execute_header",
          "value": 4294967296,
          "kind": "SECT",
          "section": 1,
          "external": true,
          "debug": false
        },
        {
          "name": "_main",
          "value": 4294968568,
          "kind": "SECT",
          "section": 1,
          "external": true,
          "debug": false
        },
        {
          "name": "__Unwind_Resume",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNKSt3__16locale9use_facetERNS0_2idE",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNKSt3__18ios_base6getlocEv",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNSt11logic_errorC2EPKc",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNSt12length_errorD1Ev",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNSt3__113basic_ostreamIcNS_11char_traitsIcEEE3putEc",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNSt3__113basic_ostreamIcNS_11char_traitsIcEEE5flushEv",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNSt3__113basic_ostreamIcNS_11char_traitsIcEEE6sentryC1ERS3_",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNSt3__113basic_ostreamIcNS_11char_traitsIcEEE6sentryD1Ev",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
...[ABRIDGED OUTPUT]

```
