use std::ops::Range;
use std::string::FromUtf8Error;

pub fn bytes_to_string(bytes: &[u8], range: Range<usize>) -> Result<String, FromUtf8Error> {
    String::from_utf8(bytes[range].to_vec())
}

pub fn bytes_to_integer_be(bytes: &[u8], range: Range<usize>) -> Result<u32, ()> {
    match bytes[range].try_into() {
        Ok(result) => Ok(u32::from_be_bytes(result)),
        Err(_) => Err(()),
    }
}
