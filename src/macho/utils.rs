// File Purpose: Various functions I seem to be using a lot between files

use std::error::Error;


/*  
    Instead of a ton of:
    
    let cputype_bytes: [u8; 4] = data[offset + 0 .. offset + 4].try_into()?;
    let cputype = if header.kind.is_be() {
        i32::from_be_bytes(cputype_bytes)
    } else {
        i32::from_le_bytes(cputype_bytes)
    };

    For each var and type, we can instead use the trait and implementations to save us the copy and paste hell
*/


pub trait FromEndianBytes: Sized {
    const SIZE: usize;

    fn from_be(bytes: &[u8]) -> Result<Self, Box<dyn Error>>;
    fn from_le(bytes: &[u8]) -> Result<Self, Box<dyn Error>>;
}

impl FromEndianBytes for u16 {
    const SIZE: usize = 2;

    fn from_be(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(u16::from_be_bytes(bytes.try_into()?))
    }
    fn from_le(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(u16::from_le_bytes(bytes.try_into()?))
    }
}

impl FromEndianBytes for u32 {
    const SIZE: usize = 4;

    fn from_be(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(u32::from_be_bytes(bytes.try_into()?))
    }
    fn from_le(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(u32::from_le_bytes(bytes.try_into()?))
    }
}

impl FromEndianBytes for i32 {
    const SIZE: usize = 4;

    fn from_be(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(i32::from_be_bytes(bytes.try_into()?))
    }
    fn from_le(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(i32::from_le_bytes(bytes.try_into()?))
    }
}

impl FromEndianBytes for u64 {
    const SIZE: usize = 8;

    fn from_be(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(u64::from_be_bytes(bytes.try_into()?))
    }
    fn from_le(bytes: &[u8]) -> Result<Self, Box<dyn Error>> {
        Ok(u64::from_le_bytes(bytes.try_into()?))
    }
}

pub fn bytes_to<T: FromEndianBytes>(is_be: bool, data: &[u8]) -> Result<T, Box<dyn Error>> {
    if data.len() <T::SIZE {
        return Err("Buffer too small".into());
    }
    if is_be {
        T::from_be(&data[..T::SIZE])
    } else {
        T::from_le(&data[..T::SIZE])
    }
}


pub fn byte_array_to_string(bytes: &[u8; 16]) -> String {
    // Intended use of this function at the time of creation is for segment and section names
    // Which I had defined in constants.rs as byte arrays in lieu of strings

    let end = bytes.iter().position(|&b| b == 0).unwrap_or(16); // scan array until first null byte, otherwise all 16 bytes are used for the name
    String::from_utf8_lossy(&bytes[..end]).into_owned()
    // from_utf8_lossy should convert any invalid UTF-8 sequences w/ the Unicode replacement char U+FFFD
    // the Mach-O spec (well, loader.h) does not appear to specify or guarantee anywhere that the segment/section names have to be UTF8
    // But according to my highly in-depth research (the first result I clicked on the internet) this func should accept any byte sequence and only return UTF8
    // so even if it's NOT UTF8 inside, it won't panic or give a result, it just swaps that with U+FFFD (the question mark in the diamond)

    // But from_utf8_lossy returns....a cow (moo?) which is a clone on write smart pointer 
    // https://doc.rust-lang.org/std/borrow/enum.Cow.html

    // tl;dr take byte array --> replace invalid utf --> clone the cow
}


pub fn read_uleb(data: &[u8], cursor: &mut usize) -> Result<u64, Box<dyn Error>> {
    // uleb128 = unsigned little endian base 128
    // Using the druntime implementation of reading it
    // Since that it what a half-decade-old stack overflow post pointed me to
    // https://github.com/dlang/druntime/blob/0dfc0ce5aef1fde00713b56e9be99dcdfb04d171/src/rt/backtrace/dwarf.d#L490-L534
    let mut result: u64 = 0;
    let mut shift: u32 = 0;
    loop {
        if *cursor >= data.len() {
            return Err("read_uleb: cursor out of bounds".into());
        }
        let byte = data[*cursor];
        *cursor += 1; // dereffing to advance the func caller's cursor 
        result |= ((byte & 0x7F) as u64) << shift; // bitwise or w/ 127 then shift
        shift += 7;
        if (byte & 0x80) == 0 { // MSB not set, no more to decode
            break;
        }

        if shift >= 64 {
            return Err("read_uleb: shift overflow (>64 bits)".into());
        }
        
    }
    Ok(result)
}



pub fn read_sleb(data: &[u8], cursor: &mut usize) -> Result<i64, Box<dyn Error>> {
    // sleb128 = signed little endian base 128
    let mut result: i64 = 0;
    let mut shift: u32 = 0;
    let size = 8 << 3; // 64 bits
    let mut byte: u8;

    loop {
        if *cursor >= data.len() {
            return Err("read_sleb: cursor out of bounds".into());
        }

        byte = data[*cursor];
        *cursor += 1;

        result |= ((byte & 0x7F) as i64) << shift;
        shift += 7;

        if (byte & 0x80) == 0 {
            break;
        }

        if shift >= 64 {
            return Err("read_sleb: shift overflow (>64 bits)".into());
        }
    }

    // sign extend
    if (shift < size) && (byte & 0x40) != 0 {
        result |= -(1 << shift);
    }

    Ok(result)
}

/*
============================
======== UNIT TESTS ========
============================ 
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_to_u32_be() {
        let data = [0x12, 0x34, 0x56, 0x78];
        let value: u32 = bytes_to(true, &data).unwrap();
        assert_eq!(value, 0x12345678);
    }

    #[test]
    fn bytes_to_u32_le() {
        let data = [0x78, 0x56, 0x34, 0x12];
        let value: u32 = bytes_to(false, &data).unwrap();
        assert_eq!(value, 0x12345678);
    }

    #[test]
    fn bytes_to_i32_negative_be() {
        let data = [0xFF, 0xFF, 0xFF, 0xFE];
        let value: i32 = bytes_to(true, &data).unwrap();
        assert_eq!(value, -2);
    }

    #[test]
    fn bytes_to_i32_negative_le() {
        let data = [0xFE, 0xFF, 0xFF, 0xFF];
        let value: i32 = bytes_to(false, &data).unwrap();
        assert_eq!(value, -2);
    }

    #[test]
    fn bytes_to_i32_positive_be() {
        let data = [0x12, 0x34, 0x56, 0x78];
        let value: i32 = bytes_to(true, &data).unwrap();
        assert_eq!(value, 0x12345678);
    }

    #[test]
    fn bytes_to_i32_positive_le() {
        let data = [0x78, 0x56, 0x34, 0x12];
        let value: i32 = bytes_to(false, &data).unwrap();
        assert_eq!(value, 0x12345678);
    }

    #[test]
    fn bytes_to_u64_le() {
        let data = [1, 0, 0, 0, 0, 0, 0, 0];
        let value: u64 = bytes_to(false, &data).unwrap();
        assert_eq!(value, 1);
    }

    fn bytes_to_u64_be() {
        let data = [0, 0, 0, 0, 0, 0, 0, 1];
        let value: u64 = bytes_to(true, &data).unwrap();
        assert_eq!(value, 1);
    }

    #[test]
    fn bytes_to_rejects_small_buffer() {
        let data = [0x01, 0x02, 0x03];
        let result: Result<u32, _> = bytes_to(true, &data);
        assert!(result.is_err());
    }

    #[test]
    fn bytes_to_keep_first_slice_only() {
        // Should only take # bytes needed for requested size, ignoring excess data
        let data = [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xFF, 0xDE, 0xAD, 0xBE, 0xEF];
        let value: u32 = bytes_to(true, &data).unwrap();
        assert_eq!(value, 0x12345678); 
        let value: i32 = bytes_to(true, &data).unwrap();
        assert_eq!(value, 0x12345678); 
        let value: u64 = bytes_to(true, &data).unwrap();
        assert_eq!(value, 0x12345678_9ABCDEFF); 
    }    
}