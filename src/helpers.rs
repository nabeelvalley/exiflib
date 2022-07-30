use std::ops::Range;

pub fn bytes_to_string(bytes: &[u8], range: Range<usize>) -> Option<String> {
    match String::from_utf8(bytes[range].to_vec()) {
        Ok(val) => Some(val),
        _ => None,
    }
}

pub fn bytes_to_usize_be(bytes: &[u8], range: Range<usize>) -> Option<usize> {
    match bytes[range].try_into() {
        Ok(result) => Some(usize::from_be_bytes(result)),
        Err(_) => None,
    }
}
