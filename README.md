# moscope — Mach-O static analysis and inspection toolkit

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
cargo run -- /usr/bin/my_binary
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

Usage: `moscope <BINARY>`


---

## Status

This project is under active development. APIs, output formats, and structure may change freely.

---

## Requirements

- Rust (stable)
- macOS (Mach-O binaries only)


