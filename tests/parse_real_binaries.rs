use std::fs;
use std::path::Path;

use moscope::macho::fat::{read_fat_archs, read_fat_header, FatKind};



/*
===============================
======== Thin Binaries ========
=============================== 
*/

#[test]
fn parses_thin_arm64_binary() {
    let path = Path::new("tests/samples/hello_arm64");
    let data = fs::read(path).expect("failed to read hello_arm64");

    // Thin binaries should NOT parse as fat
    let header = read_fat_header(&data);
    assert!(header.is_err(), "thin binary misclassified as fat");
}

#[test]
fn parses_thin_x86_64_binary() {
    let path = Path::new("tests/samples/hello_x86_64");
    let data = fs::read(path).expect("failed to read hello_x86_64");

    let header = read_fat_header(&data);
    assert!(header.is_err(), "thin binary misclassified as fat");
}

/*
========================================
======== Fat/Universal Binaries ========
======================================== 
*/

#[test]
fn parses_fat_binary_header() {
    let path = Path::new("tests/samples/hello_fat");
    let data = fs::read(path).expect("failed to read hello_fat");

    let header = read_fat_header(&data)
        .expect("failed to parse fat header");

    assert_eq!(header.nfat_arch, 2);
    assert!(
        matches!(header.kind, FatKind::Fat32BE | FatKind::Fat64BE),
        "unexpected fat kind: {:?}",
        header.kind
    );
}

#[test]
fn parses_fat_binary_archs() {
    let path = Path::new("tests/samples/hello_fat");
    let data = fs::read(path).expect("failed to read hello_fat");

    let header = read_fat_header(&data).unwrap();
    let archs = read_fat_archs(&data, &header)
        .expect("failed to parse fat archs");

    assert_eq!(archs.len(), 2);
}
