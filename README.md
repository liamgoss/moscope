> Note: This tool is a W.I.P. project I am developing to learn Rust, binary parsing, and Mach-O file specs. As such, this tool *currently* lacks useful parsing capabilities. while I continue to learn and expand upon this project. 

# machochecko

`machochecko` is a work-in-progress command-line tool written in Rust for inspecting **Mach-O** binaries, with support for **fat (universal) binaries**. It is designed as a learning project that aims to be *correct*, *explicit*, and *educational*, rather than a thin wrapper around existing tools. 

The project currently focuses on:
- Correct parsing of fat Mach-O headers
- Endianness correctness (big-endian on disk vs little-endian host)
- Architecture and subtype decoding (including `arm64e`)
- Clean CLI ergonomics using `clap`



## Features (current)

- Detects fat (universal) Mach-O binaries
- Parses and prints the fat header
- Enumerates contained architectures (`x86_64`, `arm64`, `arm64e`, etc.)
- Correctly handles:
  - Big-endian fat headers
  - Byte-swapped (`FAT_CIGAM`) headers
  - ARM vs ARM64 subtype namespaces
  - Pointer authentication ABI flags (`arm64e`)
- Interactive architecture selection for fat binaries
- Uses a proper CLI interface (`clap`) with auto-generated help

---

## Example Usage

```bash
cargo run -- /usr/bin/caffeinate
```

Example output (abridged):
```
Reading header...
Fat binary detected:
FatHeader { magic: 3405691582, nfat_arch: 2 }

Available architectures:
0: x86_64
1: arm64e

Select architecture index: 1
...
```

---

## CLI

Usage: `machochecko <BINARY>`



---

## Design Notes

- Fat headers are always parsed as **big-endian on disk**
- Host endianness is detected and handled explicitly
- ARM and ARM64 subtypes are treated as **separate namespaces**
- `arm64e` detection properly accounts for pointer-auth ABI bits
- CLI parsing is kept separate from binary parsing logic


---

## Status

This project is under active development and is currently **private**.
APIs, output formats, and structure may change freely.

---

## Requirements

- Rust (stable)
- macOS (Mach-O binaries only)

---
