use std::ops::Range;

pub fn bytes_to_string(bytes: &[u8], range: Range<usize>) -> Option<String> {
    String::from_utf8(bytes[range].to_vec()).ok()
}

pub fn bytes_to_usize_be(bytes: &[u8], range: Range<usize>) -> Option<usize> {
    let u_bytes = bytes[range].try_into().ok()?;

    Some(usize::from_be_bytes(u_bytes))
}
