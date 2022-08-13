use std::ops::Range;

#[derive(Debug)]
pub enum Endian {
    Big,
    Little,
}

pub fn bytes_to_string(bytes: &[u8], range: Range<usize>) -> Option<String> {
    String::from_utf8(bytes[range].to_vec()).ok()
}

pub fn bytes_to_u32_be(bytes: &[u8], start: usize) -> Option<u32> {
    let u_bytes = bytes[start..(start + 4)].try_into().ok()?;

    Some(u32::from_be_bytes(u_bytes))
}

pub fn bytes_to_u32_le(bytes: &[u8], start: usize) -> Option<u32> {
    let u_bytes = bytes[start..(start + 4)].try_into().ok()?;

    Some(u32::from_le_bytes(u_bytes))
}

pub fn bytes_to_u16_be(bytes: &[u8], start: usize) -> Option<u16> {
    let u_bytes = bytes[start..(start + 2)].try_into().ok()?;

    Some(u16::from_be_bytes(u_bytes))
}

pub fn bytes_to_u16_le(bytes: &[u8], start: usize) -> Option<u16> {
    let u_bytes = bytes[start..(start + 2)].try_into().ok()?;

    Some(u16::from_le_bytes(u_bytes))
}

/// [From StackOverflow](https://stackoverflow.com/questions/35901547/how-can-i-find-a-subsequence-in-a-u8-slice)
pub fn get_subsequence_offset(bytes: &[u8], pattern: &[u8]) -> Option<usize> {
    bytes
        .windows(pattern.len())
        .position(|window| window == pattern)
}

pub fn bytes_to_u32(endian: &Endian, bytes: &[u8], start: usize) -> Option<u32> {
    match endian {
        Endian::Big => bytes_to_u32_be(bytes, start),
        Endian::Little => bytes_to_u32_le(bytes, start),
    }
}

pub fn bytes_to_u16(endian: &Endian, bytes: &[u8], start: usize) -> Option<u16> {
    match endian {
        Endian::Big => bytes_to_u16_be(bytes, start),
        Endian::Little => bytes_to_u16_le(bytes, start),
    }
}
