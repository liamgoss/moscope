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