use std::ops::Range;

use crate::helpers::{bytes_to_string, bytes_to_u32_be};

/// EXIF Tag IDs from https://exiftool.org/TagNames/EXIF.html
pub enum ExifTagID {
    /// u32
    ImageWidth = 0x100,
    /// u32
    ImageHeight = 0x101,
    /// String
    Model = 0x0110,
}

#[derive(Debug)]
pub enum ExifValue {
    U32(u32),
    String(String),
}

pub struct ExifTag {
    id: ExifTagID,
    name: String,
    /// to simplify usage we will always return the value as a string
    value: String,
}

pub fn bytes_to_u32_value(bytes: &[u8], tag_id: ExifTagID) -> Option<ExifValue> {
    let result = bytes_to_u32_be(bytes, tag_id as usize)?;

    Some(ExifValue::U32(result))
}

// pub fn bytes_to_string_value(bytes: &[u8], tag_id: ExifTagID) -> Option<ExifValue> {
//     let result = bytes_to_string(bytes, [0..0].tr)?;

//     Some(ExifValue::String(result))
// }

pub fn parse_tag_value(tag_id: ExifTagID, jpeg: &[u8]) -> Option<ExifValue> {
    match tag_id {
        ExifTagID::ImageHeight => bytes_to_u32_value(jpeg, ExifTagID::ImageHeight),
        ExifTagID::ImageWidth => bytes_to_u32_value(jpeg, ExifTagID::ImageWidth),
        ExifTagID::Model => bytes_to_u32_value(jpeg, ExifTagID::Model),
    }
}
