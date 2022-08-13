use std::ops::Range;

use crate::helpers::{bytes_to_string, bytes_to_u32_be, get_subsequence_offset};

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

pub enum Endian {
    Big,
    Little,
}

/// 6 bytes identify the EXIF data start = `Exif\x00\x00`
const EXIF_MARKER: [u8; 6] = [45, 78, 69, 66, 00, 00];

fn get_start_offset(file: &[u8]) -> Option<usize> {
    get_subsequence_offset(file, &EXIF_MARKER)
}

fn get_endian(exif: &[u8]) -> Option<Endian> {
    let range = 6..8;
    let endian = bytes_to_string(exif, range)?;

    match endian.as_str() {
        "MM" => Some(Endian::Big),
        "ll" => Some(Endian::Little),
        _ => None,
    }
}

fn get_u32_value(endian: Endian, exif: &[u8], tag_id: ExifTagID) -> Option<ExifValue> {
    let result = bytes_to_u32_be(exif, tag_id as usize)?;

    Some(ExifValue::U32(result))
}

// pub fn bytes_to_string_value(bytes: &[u8], tag_id: ExifTagID) -> Option<ExifValue> {
//     let result = bytes_to_string(bytes, [0..0])?;

//     Some(ExifValue::String(result))
// }

pub fn parse_tag_value(tag_id: ExifTagID, exif: &[u8], endian: Endian) -> Option<ExifValue> {
    match tag_id {
        ExifTagID::ImageHeight => get_u32_value(endian, exif, ExifTagID::ImageHeight),
        ExifTagID::ImageWidth => get_u32_value(endian, exif, ExifTagID::ImageWidth),
        ExifTagID::Model => get_u32_value(endian, exif, ExifTagID::Model),
    }
}
