use std::ops::Range;

pub fn bytes_to_string(bytes: &[u8], range: Range<usize>) -> Option<String> {
    String::from_utf8(bytes[range].to_vec()).ok()
}

pub fn bytes_to_u32_be(bytes: &[u8], start: usize) -> Option<u32> {
    let u_bytes = bytes[start..(start + 4)].try_into().ok()?;

    Some(u32::from_be_bytes(u_bytes))
}
