# moscope - Mach-O static analysis and inspection toolkit

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
- Parses and summarizes symbols
  - Extracts all symbol table entries from LC_SYMTAB
  - Classifies symbols (external, debug, etc.)
  - Displays symbol names, types, and linkage
- **String Extraction**
  - Extracts null-terminated strings from binary sections
  - Uses VM-based memory mapping for accurate extraction from dyld-cached binaries
  - Associates strings with their source segment and section
  - **Regex pattern filtering** for targeted string analysis
  - **Section filtering** to include/exclude specific sections
  - Configurable minimum string length and maximum string count
- Provides structured, human-readable output suitable for reverse engineering and binary inspection
- Exposes functionality through a structured command-line interface (with ANSI coloring, *optionally disabled with the `--no-color` flag*)

---

## Installation

```bash
# Clone the repository
git clone https://github.com/liamgoss/moscope.git
cd moscope

# Build with Cargo
cargo build --release

# The binary will be at target/release/moscope
```

---

## Example Usage

### Basic Inspection

```bash
# Inspect a thin or fat Mach-O binary with interactive colored text output (default)
moscope /path/to/target_binary

# Inspect a binary without color output
moscope /path/to/target_binary --no-color

# Inspect and output JSON instead of text (good for automation or parsing)
moscope /path/to/target_binary --format json
```

### Symbol Control

```bash
# Limit the number of symbol entries displayed (e.g., top 100 symbols)
moscope /path/to/target_binary --symbol-limit 100

# Skip symbol table output entirely
moscope /path/to/target_binary --no-symbols
```

### String Extraction

```bash
# Adjust string extraction behavior
#   --min-string-length: ignore strings shorter than the specified length (default: 4)
#   --max-num-strings: limit the total number of extracted strings
moscope /path/to/target_binary --min-string-length 6 --max-num-strings 100

# Filter strings using regex patterns
moscope /path/to/target_binary --string-pattern '^https?://'  # URLs
moscope /path/to/target_binary --string-pattern '\.dylib$'     # dylib names
moscope /path/to/target_binary --string-pattern '(?i)(error|warning|fail)'  # Error messages

# Only extract strings from specific sections
moscope /path/to/target_binary --string-sections __cstring,__const

# Skip specific sections (e.g., Objective-C type encodings)
moscope /path/to/target_binary --skip-sections __objc_methtype

# Combine filters
moscope /path/to/target_binary --string-pattern '^/' --skip-sections __objc_methtype
```

### Useful String Patterns

```bash
# Find URLs
moscope binary --string-pattern '^https?://'

# Find file paths
moscope binary --string-pattern '^/[A-Za-z/]+'

# Find error/warning messages
moscope binary --string-pattern '(?i)(error|warning|fail|exception)'

# Find potential secrets
moscope binary --string-pattern '(?i)(password|secret|key|token|api|auth)'

# Find email addresses
moscope binary --string-pattern '[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}'

# Find IP addresses
moscope binary --string-pattern '\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b'

# Find Apple internal debug strings (emojis!)
moscope binary --string-pattern '^[☀️🎨📸⚡️🐛]'

# Find format strings
moscope binary --string-pattern '%[sdifx@]'
```

### Output Control

```bash
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

## Command-Line Reference

| Flag | Description | Example |
|------|-------------|---------|
| `--no-color` | Disable colored output | `moscope binary --no-color` |
| `--format <json\|text>` | Output format (default: text) | `moscope binary --format json` |
| `--min-string-length <N>` | Minimum string length to extract (default: 4) | `moscope binary --min-string-length 8` |
| `--max-strings <N>` | Maximum number of strings to display | `moscope binary --max-strings 100` |
| `--max-symbols <N>` | Maximum number of symbols to display | `moscope binary --max-symbols 50` |
| `--string-pattern <REGEX>` | Filter strings by regex pattern | `moscope binary --string-pattern '^http'` |
| `--string-sections <LIST>` | Only extract from these sections (comma-separated) | `moscope binary --string-sections __cstring` |
| `--skip-sections <LIST>` | Skip these sections (comma-separated) | `moscope binary --skip-sections __objc_methtype` |
| `--no-symbols` | Skip symbol table output | `moscope binary --no-symbols` |
| `--no-segments` | Skip segment information | `moscope binary --no-segments` |
| `--no-loadcmds` | Skip load command information | `moscope binary --no-loadcmds` |
| `--no-header` | Skip Mach-O header output | `moscope binary --no-header` |

---

## Project Status

`moscope` is under active development. Internal APIs, function signatures, output formats, program structure, and program invocation are expected to change as additional Mach-O features and analysis stages are implemented.

## Requirements

- Rust (stable)

### Platform notes

`moscope` is a pure Rust Mach-O parsing tool and can be built and used on any platform (Linux, macOS, or Windows). 

`moscope` does not currently execute the target binary.

---

## Future Work

Development is focused on incrementally expanding coverage of the Mach-O format while keeping parsing behavior fault tolerant. Near term goals include:

- Symbol demangling (C++ and Swift)
- Enhanced Objective-C runtime parsing (classes, protocols, methods)
- Swift metadata extraction
- Code signature and entitlement inspection
- Pointer authentication analysis (arm64e)
- Chained fixups parsing

This project is intentionally deferring deeper runtime and platform specific features until core structural coverage is complete.

---

## Example Output

<details>
<summary><b>Thin Binary (Interactive Text Output)</b></summary>

```
Mach-O Report:

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


Dynamic Libraries
----------------------------------------
[LOAD    ] /usr/lib/libc++.1.dylib
[LOAD    ] /usr/lib/libSystem.B.dylib

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


Symbols
----------------------------------------
[SECT]                    __ZNSt3__124__put_character_sequenceB8ne200100IcNS_11char_traitsIcEEEERNS_13basic_ostreamIT_T0_EES7_PKS4_m
[SECT]                    __ZNSt3__116__pad_and_outputB8ne200100IcNS_11char_traitsIcEEEENS_19ostreambuf_iteratorIT_T0_EES6_PKS4_S8_S8_RNS_8ios_baseES4_
[SECT]                    ___clang_call_terminate
[SECT]                    __ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEC2B8ne200100Emc
[SECT]                    __ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEE20__throw_length_errorB8ne200100Ev
[SECT]                    __ZNSt3__120__throw_length_errorB8ne200100EPKc
[SECT]                    __ZNSt12length_errorC1B8ne200100EPKc
[SECT]                    GCC_except_table0
[SECT]                    GCC_except_table1
[SECT]                    GCC_except_table2
[SECT]                    GCC_except_table6
[SECT] EXT                __mh_execute_header
[SECT] EXT                _main
[UNDEF] EXT                __Unwind_Resume
[UNDEF] EXT                __ZNKSt3__16locale9use_facetERNS0_2idE
[UNDEF] EXT                __ZNKSt3__18ios_base6getlocEv
[UNDEF] EXT                __ZNSt11logic_errorC2EPKc
[UNDEF] EXT                __ZNSt12length_errorD1Ev
[UNDEF] EXT                __ZNSt3__113basic_ostreamIcNS_11char_traitsIcEEE3putEc
[UNDEF] EXT                __ZNSt3__113basic_ostreamIcNS_11char_traitsIcEEE5flushEv
[UNDEF] EXT                __ZNSt3__113basic_ostreamIcNS_11char_traitsIcEEE6sentryC1ERS3_
[UNDEF] EXT                __ZNSt3__113basic_ostreamIcNS_11char_traitsIcEEE6sentryD1Ev
[UNDEF] EXT                __ZNSt3__14coutE
[UNDEF] EXT                __ZNSt3__15ctypeIcE2idE
[UNDEF] EXT                __ZNSt3__16localeD1Ev
[UNDEF] EXT                __ZNSt3__18ios_base33__set_badbit_and_consider_rethrowEv
[UNDEF] EXT                __ZNSt3__18ios_base5clearEj
[UNDEF] EXT                __ZSt9terminatev
[UNDEF] EXT                __ZTISt12length_error
[UNDEF] EXT                __ZTVSt12length_error
[UNDEF] EXT                __ZdlPv
[UNDEF] EXT                __Znwm
[UNDEF] EXT                ___cxa_allocate_exception
[UNDEF] EXT                ___cxa_begin_catch
[UNDEF] EXT                ___cxa_end_catch
[UNDEF] EXT                ___cxa_free_exception
[UNDEF] EXT                ___cxa_throw
[UNDEF] EXT                ___gxx_personality_v0
[UNDEF] EXT                _memset

Strings
----------------------------------------
[__TEXT:__cstring] Hello world!
[__TEXT:__cstring] basic_string
liam@liam:projects/moscope - (main) > 
```

</details>

<details>
<summary><b>JSON Output (Universal Binary)</b></summary>

```json
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
        {
          "name": "__ZNSt3__14coutE",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNSt3__15ctypeIcE2idE",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNSt3__16localeD1Ev",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNSt3__18ios_base33__set_badbit_and_consider_rethrowEv",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNSt3__18ios_base5clearEj",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZSt9terminatev",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZTISt12length_error",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZTVSt12length_error",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZdlPv",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__Znwm",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "___cxa_allocate_exception",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "___cxa_begin_catch",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "___cxa_end_catch",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "___cxa_free_exception",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "___cxa_throw",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "___gxx_personality_v0",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "_memset",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        }
      ],
      "strings": [
        {
          "value": "Hello world!",
          "segname": "__TEXT",
          "sectname": "__cstring"
        },
        {
          "value": "basic_string",
          "segname": "__TEXT",
          "sectname": "__cstring"
        }
      ]
    },
    {
      "cpu_type": "ARM",
      "cpu_subtype": "arm64 (ARM64_ALL)",
      "header": {
        "magic": 4277009103,
        "file_type": "Demand Paged Executable File [[MH_EXECUTE]]",
        "cpu_type": "ARM",
        "cpu_subtype": "arm64 (ARM64_ALL)",
        "ncmds": 18,
        "sizeofcmds": 1184,
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
          "vmsize": 16384,
          "fileoff": 0,
          "filesize": 16384,
          "maxprot": "R-X",
          "initprot": "R-X",
          "sections": [
            {
              "name": "__text",
              "segment": "__TEXT",
              "kind": "Code",
              "addr": 4294968552,
              "size": 1152
            },
            {
              "name": "__stubs",
              "segment": "__TEXT",
              "kind": "Stub",
              "addr": 4294969704,
              "size": 240
            },
            {
              "name": "__gcc_except_tab",
              "segment": "__TEXT",
              "kind": "Exception",
              "addr": 4294969944,
              "size": 128
            },
            {
              "name": "__cstring",
              "segment": "__TEXT",
              "kind": "CString",
              "addr": 4294970072,
              "size": 26
            },
            {
              "name": "__unwind_info",
              "segment": "__TEXT",
              "kind": "Unwind",
              "addr": 4294970100,
              "size": 160
            }
          ]
        },
        {
          "name": "__DATA_CONST",
          "vmaddr": 4294983680,
          "vmsize": 16384,
          "fileoff": 16384,
          "filesize": 16384,
          "maxprot": "RW-",
          "initprot": "RW-",
          "sections": [
            {
              "name": "__got",
              "segment": "__DATA_CONST",
              "kind": "SymbolPointer",
              "addr": 4294983680,
              "size": 208
            }
          ]
        },
        {
          "name": "__LINKEDIT",
          "vmaddr": 4295000064,
          "vmsize": 16384,
          "fileoff": 32768,
          "filesize": 3544,
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
          "value": 4294969080,
          "kind": "SECT",
          "section": 1,
          "external": false,
          "debug": false
        },
        {
          "name": "___clang_call_terminate",
          "value": 4294969396,
          "kind": "SECT",
          "section": 1,
          "external": false,
          "debug": false
        },
        {
          "name": "__ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEEC2B8ne200100Emc",
          "value": 4294969412,
          "kind": "SECT",
          "section": 1,
          "external": false,
          "debug": false
        },
        {
          "name": "__ZNSt3__112basic_stringIcNS_11char_traitsIcEENS_9allocatorIcEEE20__throw_length_errorB8ne200100Ev",
          "value": 4294969568,
          "kind": "SECT",
          "section": 1,
          "external": false,
          "debug": false
        },
        {
          "name": "__ZNSt3__120__throw_length_errorB8ne200100EPKc",
          "value": 4294969588,
          "kind": "SECT",
          "section": 1,
          "external": false,
          "debug": false
        },
        {
          "name": "__ZNSt12length_errorC1B8ne200100EPKc",
          "value": 4294969668,
          "kind": "SECT",
          "section": 1,
          "external": false,
          "debug": false
        },
        {
          "name": "GCC_except_table0",
          "value": 4294969944,
          "kind": "SECT",
          "section": 3,
          "external": false,
          "debug": false
        },
        {
          "name": "GCC_except_table1",
          "value": 4294969964,
          "kind": "SECT",
          "section": 3,
          "external": false,
          "debug": false
        },
        {
          "name": "GCC_except_table2",
          "value": 4294970032,
          "kind": "SECT",
          "section": 3,
          "external": false,
          "debug": false
        },
        {
          "name": "GCC_except_table6",
          "value": 4294970056,
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
          "value": 4294968552,
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
        {
          "name": "__ZNSt3__14coutE",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNSt3__15ctypeIcE2idE",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNSt3__16localeD1Ev",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNSt3__18ios_base33__set_badbit_and_consider_rethrowEv",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZNSt3__18ios_base5clearEj",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZSt9terminatev",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZTISt12length_error",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZTVSt12length_error",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__ZdlPv",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "__Znwm",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "___cxa_allocate_exception",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "___cxa_begin_catch",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "___cxa_end_catch",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "___cxa_free_exception",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "___cxa_throw",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "___gxx_personality_v0",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        },
        {
          "name": "_memset",
          "value": 0,
          "kind": "UNDEF",
          "section": null,
          "external": true,
          "debug": false
        }
      ],
      "strings": [
        {
          "value": "Hello world!",
          "segname": "__TEXT",
          "sectname": "__cstring"
        },
        {
          "value": "basic_string",
          "segname": "__TEXT",
          "sectname": "__cstring"
        }
      ]
    }
  ]
}
```

</details>

<details>
<summary><b>String Extraction with Regex Filter</b></summary>

```bash
# Extract only URLs from ImageIO framework (iOS 26)
$ moscope ImageIO --string-pattern '^https?://'

Strings
----------------------------------------
[__TEXT:__cstring] http://ns.apple.com/HDRGainMap/1.0/
[__TEXT:__cstring] http://ns.apple.com/HDRToneMap/1.0/
[__TEXT:__cstring] http://ns.adobe.com/xmp/extension/
[__TEXT:__cstring] http://ns.adobe.com/xap/1.0/
[__TEXT:__cstring] http://ns.adobe.com/exif/1.0/
[__TEXT:__cstring] http://ns.adobe.com/exif/1.0/aux/
[__TEXT:__cstring] http://cipa.jp/exif/1.0/
[__TEXT:__cstring] http://ns.adobe.com/DICOM/
[__TEXT:__cstring] http://purl.org/dc/elements/1.1/
[__TEXT:__cstring] http://iptc.org/std/Iptc4xmpCore/1.0/xmlns/
[__TEXT:__cstring] http://iptc.org/std/Iptc4xmpExt/2008-02-29/
[__TEXT:__cstring] http://ns.adobe.com/photoshop/1.0/
[__TEXT:__cstring] http://ns.adobe.com/tiff/1.0/
[__TEXT:__cstring] http://ns.adobe.com/xap/1.0/rights/
[__TEXT:__cstring] http://ns.adobe.com/xap/1.0/sType/Dimensions#
[__TEXT:__cstring] http://www.w3.org/XML/1998/namespace
[__TEXT:__cstring] http://ns.adobe.com/camera-raw-settings/1.0/
[__TEXT:__cstring] http://ns.adobe.com/xap/1.0/mm/
[__TEXT:__cstring] http://www.metadataworkinggroup.com/schemas/regions/
[__TEXT:__cstring] http://ns.apple.com/faceinfo/1.0/
[__TEXT:__cstring] http://ns.apple.com/ImageIO/1.0/
[__TEXT:__cstring] http://ns.adobe.com/xmp/sType/Area#
[__TEXT:__cstring] http://ns.adobe.com/hdr-gain-map/1.0/
[__TEXT:__cstring] http://www.w3.org/1999/02/22-rdf-syntax-ns#
[__TEXT:__cstring] http://ns.adobe.com/xmp/note/
[__TEXT:__cstring] http://ns.adobe.com/xmp/1.0/DynamicMedia/
[__TEXT:__cstring] http://qualifiers
[__TEXT:__cstring] http://ns.adobe.com/pdf/1.3/
[__TEXT:__cstring] http://ns.adobe.com/xmp/1.0/Script/
[__TEXT:__cstring] http://ns.adobe.com/bwf/bext/1.0/
[__TEXT:__cstring] http://ns.adobe.com/StockPhoto/1.0/
[__TEXT:__cstring] http://ns.adobe.com/xap/1.0/t/
[__TEXT:__cstring] http://ns.adobe.com/xap/1.0/t/pg/
[__TEXT:__cstring] http://ns.adobe.com/xap/1.0/g/
[__TEXT:__cstring] http://ns.adobe.com/xap/1.0/g/img/
[__TEXT:__cstring] http://ns.adobe.com/xap/1.0/sType/Font#
[__TEXT:__cstring] http://ns.adobe.com/album/1.0/
[__TEXT:__cstring] http://ns.adobe.com/png/1.0/
[__TEXT:__cstring] http://ns.adobe.com/jpeg/1.0/
[__TEXT:__cstring] http://ns.adobe.com/jp2k/1.0/
[__TEXT:__cstring] http://ns.adobe.com/asf/1.0/
[__TEXT:__cstring] http://ns.adobe.com/xmp/wav/1.0/
[__TEXT:__cstring] http://ns.adobe.com/creatorAtom/1.0/
[__TEXT:__cstring] http://ns.adobe.com/xap/1.0/bj/
[__TEXT:__cstring] http://ns.adobe.com/aes/cart/
[__TEXT:__cstring] http://ns.adobe.com/riff/info/
[__TEXT:__cstring] http://ns.adobe.com/xap/1.0/sType/ResourceEvent#
[__TEXT:__cstring] http://ns.adobe.com/xap/1.0/sType/ResourceRef#
[__TEXT:__cstring] http://ns.adobe.com/xap/1.0/sType/Version#
[__TEXT:__cstring] http://ns.adobe.com/xap/1.0/sType/Job#
[__TEXT:__cstring] http://ns.adobe.com/xap/1.0/sType/ManifestItem#
[__TEXT:__cstring] http://ns.adobe.com/xmp/Identifier/qual/1.0/
[__TEXT:__cstring] http://ns.useplus.org/ldf/xmp/1.0/
[__TEXT:__cstring] http://www.aiim.org/pdfa/ns/schema#
[__TEXT:__cstring] http://www.aiim.org/pdfa/ns/property#
[__TEXT:__cstring] http://www.aiim.org/pdfa/ns/type#
[__TEXT:__cstring] http://www.aiim.org/pdfa/ns/field#
[__TEXT:__cstring] http://www.aiim.org/pdfa/ns/id/
[__TEXT:__cstring] http://www.aiim.org/pdfa/ns/extension/
[__TEXT:__cstring] http://ns.adobe.com/pdfx/1.3/
[__TEXT:__cstring] http://www.npes.org/pdfx/ns/id/
[__TEXT:__cstring] http://ns.adobe.com/iX/1.0/
[__TEXT:__cstring] http://ns.adobe.com/ixml/1.0/
[__TEXT:__cstring] http://purl.org/dc/1.1/
[__TEXT:__cstring] http://metadata
```

```bash
# Find Apple internal debug strings with emojis
$ moscope ImageIO --string-pattern '[☀️🎨📸⚡️🐛]'

Strings
----------------------------------------
[__TEXT:__cstring] ☀️  gain map headroom: %0.1f requested: %0.1f capacity: %0.1f -> %0.1f\n
[__TEXT:__cstring] ☀️  generating FlexGTC info as requested...\n
[__TEXT:__cstring] ☀️  destination already has FlexGTC info: %s\n
[__TEXT:__cstring] ☀️  generated FlexGTC info: %s\n
[__TEXT:__cstring] ☀️  generated FlexGTC space: %s\n
[__TEXT:__cstring] ☀️  computing HDRStats as requested...\n
[__TEXT:__cstring] ☀️ Stats data: %s\n
[__TEXT:__cstring] ☀️  gain map headroom: %0.1f requested: %0.1f -> %0.1f\n
[__TEXT:__cstring] ☀️ base headroom: %0.1f alt headroom: %0.1f requested: %0.1f -> %0.1f\n
[__TEXT:__cstring] ☀️ FlexGTC gain map headroom: %0.1f requested: %0.1f -> %0.1f\n
[__TEXT:__cstring] ☀️ FlexGTC curve data: %s\n
[__TEXT:__cstring] ☀️ Wrote CLLI data: (%u,%u)\n
[__TEXT:__cstring] ☀️ Missing CLLI data\n
[__TEXT:__cstring] ☀️ Invalid CLLI data: (%u,%u)\n
[__TEXT:__cstring] ☀️ Read CLLI data: (%u,%u)\n
[__TEXT:__cstring] ☀️ Using provided alternate space: %s\n
[__TEXT:__cstring] ☀️ Requested headroom (%0.1f) is less than FlexGTC headroom (%0.1f), dropping FlexGTC info\n
[__TEXT:__cstring] ☀️ HDR output colorspace: %s\n
[__TEXT:__cstring] ☀️ HDR input headroom: %0.1f, destination: %0.1f, output headroom: %0.1f\n
[__TEXT:__cstring] ☀️  Missing makernote data, using fallback hdrGain=%f\n
[__TEXT:__cstring] ☀️  Missing makernote data: %s, no fallback provided
[__TEXT:__cstring] ☀️  Invalid Meteor+ makernote data (hdrGain=%s, gainMapValue=%s)
[__TEXT:__cstring] ☀️  Meteor+ headroom: %f (hdrGain=%f, gainMapValue=%f)
[__TEXT:__cstring] ☀️ Metal disabled, will use SIMD for image conversion
[__TEXT:__cstring] ☀️ Failed to initialize Metal converter, falling back to SIMD for image conversion (slow)
[__TEXT:__cstring] ☀️ Using converter: %s\n
[__TEXT:__cstring] ☀️ HDRImageConverter::convertImageToImage SRC x ALT => DST\nSRC[EDR=%g] => %s\nALT[EDR=%g] => %s\nDST[EDR=%g] => %s
[__TEXT:__cstring] ☀️ HDRImageConverter::convertImageToImage SRC => DST\nSRC[EDR=%g] => %s\nDST[EDR=%g] => %s
[__TEXT:__cstring] ☀️ HDRImageConverter::generateToneMappingCurve SRC => GTC\nSRC[EDR=%g] => %s
[__TEXT:__cstring] ☀️ HDRImageConverter::computeHDRStatistics SRC => STATS\nSRC[EDR=%g] => %s
[__TEXT:__cstring] ☀️  PixelFormat::choosePixelFormat: '%c%c%c%c' plane: %u -> %s\n
[__TEXT:__cstring] ☀️ Using subsample factor: %u (%u px)
[__TEXT:__cstring] ☀️ Metal converter not available: no device!
[__TEXT:__cstring] ☀️ Metal converter not available: VM does not support argument buffers [100784848]
[__TEXT:__cstring] ☀️ Metal converter not available: Intel GPU does not support argument buffers [128179728]
[__TEXT:__cstring] ☀️  metalPixelFormatForPixelFormat: '%c%c%c%c' plane: %u -> %lu\n
[__TEXT:__cstring] ☀️ CommandBuffer %p failed '%s'
[__TEXT:__cstring] ☀️ [STATS] Using subsample factor: %u (%u px)
[__TEXT:__cstring] ☀️  %s - updating thumbnail headroom: %g\n
[__TEXT:__cstring] ☀️  %s - updating <IOSurface: %p>  headroom: %g\n
[__TEXT:__cstring] ☀️  %s - updating <IOSurface: %p>  maxContentLightLevel: %d\n
[__TEXT:__cstring] ☀️  CGImageCreateByConvertingExtendedSRGBToColorspace (mode: %d)\n
[__TEXT:__cstring] ☀️  _colorSpaceIsFlexGTCProxy - now creating a FlexGTC colorspace... (didCalculateFlexGTC: %d)\n
[__TEXT:__cstring] ☀️  updateColorSpace: - FlexGTC colorspace - headroom: %g\n
[__TEXT:__cstring] ☀️  updateColorSpace: - old caLL: %g   new: %g\n
[__TEXT:__cstring] ☀️  ERROR: failed to create FlexGTCInfo - setting colorSpace to '%s' [%p]\n
[__TEXT:__cstring] ☀️  %s - updating image headroom: %g [colorspace: '%s']\n
[__TEXT:__cstring] ☀️  %s - failed to update image headroom: %g  [colorspace: '%s']\n
[__TEXT:__cstring] ☀️  %s - not setting image headroom: %g for SDR [colorspace: '%s']\n
[__TEXT:__cstring] ☀️  %s - input is extended range\n
[__TEXT:__cstring] ☀️  ERROR: IIOCallConvertHDRData failed (%d)\n
[__TEXT:__cstring] ☀️  ERROR:  surfaceAlpha: %s        imgAlpha: %s\n
[__TEXT:__cstring] ☀️  wrappingIOSurface - using original images/IOSurface (no EncodeRequest)...\n
[__TEXT:__cstring] ☀️  %s - using 'IIOCallConvertHDRData'\n
[__TEXT:__cstring] ☀️  CGImageCreateByConvertingExtendedSRGBToColorspace\n
[__TEXT:__cstring] ☀️  %s - adding image with headroom: %g\n
[__TEXT:__cstring] ☀️  %s - using image with headroom: %g\n
[__TEXT:__cstring] ☀️  %s\n
[__TEXT:__cstring] ☀️  %s - srcImage has headroom '0.0' --> using 8.0 \n
[__TEXT:__cstring] ☀️  %s ☀️\n
[__TEXT:__cstring] ☀️  HEIFReadPlugin::HEIFReadPlugin   [%p]\n
[__TEXT:__cstring] ☀️  HEIFReadPlugin::~HEIFReadPlugin  ••• [%p]\n
[__TEXT:__cstring] ☀️  decodeRequest: 'kCGImageSourceDecodeToHDR'\n
[__TEXT:__cstring] ☀️  %s - _gainMapHeadroom: %g\n
[__TEXT:__cstring] ☀️  kCGComputeHDRStats was not specified - temporary setting _computeHDRStats to true\n
[__TEXT:__cstring] ☀️  decodeRequest: 'kCGImageSourceDecodeToSDR'\n
[__TEXT:__cstring] ☀️  'IIOCallCreatePixelBufferAttributesForHDRType'\n
[__TEXT:__cstring] ☀️  image has alpha plane --> pixelformat 'l64r'\n
[__TEXT:__cstring] ☀️  requestedPixelFormat: '%c%c%c%c'   %s:%d\n
[__TEXT:__cstring] ☀️  requestedColorSpace: '%s'\n
[__TEXT:__cstring] ☀️  requestedYCCMatrix: '%s'\n
[__TEXT:__cstring] ☀️  %s - headroom from makerNote: _meteorHeadroom: %g   _meteorPlusHeadroom:%g\n
[__TEXT:__cstring] ☀️  CHECKME: we called 'didCalculateFlexGTC' and are setting colorSpaceIsFlexGTCProxy - this will call CalculateFlexGTC again\n
[__TEXT:__cstring] ☀️  temp setting: _computeHDRStats to false   [%p]\n
[__TEXT:__cstring] ☀️  'IIOCallCreateFlexGTCInfo (didCalculateFlexGTC: %d)'\n
[__TEXT:__cstring] ☀️  'IIOCallComputeHDRStats'   (didComputeHDRStats: %d)\n
[__TEXT:__cstring] ☀️  re-setting: _computeHDRStats to %s   [%p]\n
[__TEXT:__cstring] ☀️  _requestedApplyGainMap --> createSurfaceWithGainMapApplied\n
[__TEXT:__cstring] ☀️  _requestedApplyToneMap --> createSurfaceWithToneMapApplied\n
[__TEXT:__cstring] ☀️  ***ERROR: cannot applyGainMap / applyToneMap into a caller-provided IOSurface\n
[__TEXT:__cstring] ☀️  no apply / no tone mapping / no compute stats [%p]\n
[__TEXT:__cstring] ☀️  _computeHDRStats --> IIOCallComputeHDRStats\n
[__TEXT:__cstring] ☀️  HDR Stats: headroom=%g brightness=%g  (%g)
[__TEXT:__cstring] ☀️  HDR Stats(cached): headroom=%g brightness=%g  (%g)
[__TEXT:__cstring] ☀️  'IIOCallApplyHDRGainmap'\n
[__TEXT:__cstring] ☀️  <<< %s [%g ms]\n
[__TEXT:__cstring] ☀️  %s [%g ms]\n
[__TEXT:__cstring] ☀️  'IIOCallConvertHDRData'\n
[__TEXT:__cstring] ☀️  %s - updating image headroom: %g\n
```

```bash
# Find error messages while skipping Objective-C type encodings
$ moscope binary --string-pattern '(?i)(error|fail)' --skip-sections __objc_methtype

Strings
----------------------------------------
[__TEXT:__cstring] ❌  failed to load 'CMTimebaseGetTime' [%s]\n
[__TEXT:__cstring] ❌  failed to load 'CMTimebaseSetRate' [%s]\n
[__TEXT:__cstring] ❌  failed to load 'CMTimebaseSetTimerDispatchSourceNextFireTime' [%s]\n
[__TEXT:__cstring] *** Failed to load 'CoreMedia' symbols ***\n
[__TEXT:__cstring] ❌  failed to load 'VTAreVideoDecodersRunningInProcess' [%s]\n
[__TEXT:__cstring] ❌  failed to load 'VTPixelTransferSessionCreate' [%s]\n
[__TEXT:__cstring] ❌  failed to load 'VTPixelTransferSessionTransferImage' [%s]\n
[__TEXT:__cstring] *** ERROR: CGAnimateImageAtURLWithBlock: url is nil\n
[__TEXT:__cstring] *** ERROR: CGAnimateImageAtURLWithBlock: url is not a CFURLRef\n
[__TEXT:__cstring] *** ERROR: CGAnimateImageAtURLWithBlock: options parameter is not a CFDictionaryRef - ignoring\n
[__TEXT:__cstring] Error occured '%s' at columns: %d:%d while parsing metadata path.\n%s\n%*s%s\n
[__TEXT:__cstring] Error occured while parsing metadata path: '%s'
[__TEXT:__cstring] zlib error
[__TEXT:__cstring] zlib memory error
[__TEXT:__cstring] zlib version error
[__TEXT:__cstring] Unknown zlib error
[__TEXT:__cstring] Decompression error
[__TEXT:__cstring] *** ERROR: imageProperties or imageMetadata have to be non-nil\n
[__TEXT:__cstring] *** ERROR: XMP not recognized (does not start with 'http://ns.adobe.com/xap/1.0/'\n
[__TEXT:__cstring] *** ERROR: XMP exention blocks not handled yet!\n
[__TEXT:__cstring] *** ERROR: IPTC not recognized (does not start with 'Photoshop 3.0'\n
[__TEXT:__cstring] *** ERROR: xmpData: %p %s\n
[__TEXT:__cstring] *** ERROR: extendedXMPData: %p is not a CFArrayRef\n
[__TEXT:__cstring] *** ERROR: xmpDataOut & imageMetadataOut are both NULL\n

```

</details>

---

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.
