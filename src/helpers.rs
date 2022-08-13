use std::ops::Range;

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

/// [From StackOverflow](https://stackoverflow.com/questions/35901547/how-can-i-find-a-subsequence-in-a-u8-slice)
pub fn get_subsequence_offset(bytes: &[u8], pattern: &[u8]) -> Option<usize> {
    bytes
        .windows(pattern.len())
        .position(|window| window == pattern)
}
