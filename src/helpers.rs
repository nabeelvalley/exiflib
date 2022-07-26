use std::ops::Range;
use std::string::FromUtf8Error;

pub fn bytes_to_string(bytes: &[u8], range: Range<usize>) -> Result<String, FromUtf8Error> {
    String::from_utf8(bytes[range].to_vec())
}
