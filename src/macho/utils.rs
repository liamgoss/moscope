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