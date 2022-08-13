use std::ops::Range;

use crate::helpers::{bytes_to_string, bytes_to_u16, bytes_to_u32, get_subsequence_offset, Endian};

/// EXIF Tag IDs from https://exiftool.org/TagNames/EXIF.html
/// Offsets are from the start of the Endian Marker (MM or II)
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

pub struct Exif<'a> {
    pub bytes: &'a [u8],
    /// There are potentially multiple of these but I'm just handling the single case
    pub ifd: &'a [u8],
    endian: Endian,
}

#[derive(Debug)]
pub struct RawTag<'a> {
    pub entry: &'a [u8],
    pub tag: &'a [u8],
    pub format: &'a [u8],
    pub components: &'a [u8],
    pub data: &'a [u8],
}

pub struct ExifTag {
    pub id: ExifTagID,
    pub name: String,
    /// to simplify usage we will always return the value as a string
    pub value: String,
}

/// 6 bytes identify the EXIF data start = `Exif\x00\x00`
const EXIF_MARKER: &[u8] = "Exif\0\0".as_bytes();
const ENDIAN_RANGE: Range<usize> = 6..8;
const IFD_OFFSET_RANGE: Range<usize> = 10..14;

pub fn parse(file: &[u8]) -> Option<Exif> {
    let start = get_start(file)?;

    // TODO: get these bytes in a better way
    let bytes = &file[start..];

    let endian = get_endian(bytes)?;
    let ifd = get_ifd_bytes(&endian, bytes)?;

    let exif = Exif { ifd, bytes, endian };

    Some(exif)
}

fn get_marker_start(file: &[u8]) -> Option<usize> {
    get_subsequence_offset(file, EXIF_MARKER)
}

fn get_endian(exif: &[u8]) -> Option<Endian> {
    let endian = bytes_to_string(exif, ENDIAN_RANGE)?;

    match endian.as_str() {
        "MM" => Some(Endian::Big),
        "II" => Some(Endian::Little),
        _ => None,
    }
}

// TODO: find a way to find the end of this byte range
fn get_start(file: &[u8]) -> Option<usize> {
    get_marker_start(file)
}

fn get_ifd_bytes<'a>(endian: &Endian, exif: &'a [u8]) -> Option<&'a [u8]> {
    let offset = bytes_to_u32(endian, exif, IFD_OFFSET_RANGE.start)?;

    // the IFD start is defined as the location from the Endian marker start
    let start = ENDIAN_RANGE.start + (offset as usize);

    Some(&exif[start..])
}

fn get_u32_value(endian: &Endian, ifd: &[u8], tag_id: ExifTagID) -> Option<ExifValue> {
    let result = bytes_to_u32(endian, ifd, tag_id as usize);
    let value = ExifValue::U32(result?);

    Some(value)
}

pub fn get_tag_value(exif: &Exif, tag_id: ExifTagID) -> Option<ExifValue> {
    let endian = &exif.endian;
    let ifd = exif.ifd;

    match tag_id {
        ExifTagID::ImageHeight => get_u32_value(endian, ifd, ExifTagID::ImageHeight),
        ExifTagID::ImageWidth => get_u32_value(endian, ifd, ExifTagID::ImageWidth),
        ExifTagID::Model => get_u32_value(endian, ifd, ExifTagID::Model),
    }
}

fn parse_entry(entry: &[u8]) -> Option<RawTag> {
    let tag = entry.get(0..2)?;
    let format = entry.get(2..4)?;
    let components = entry.get(4..8)?;
    let data = entry.get(8..12)?;

    let tag = RawTag {
        entry,
        tag,
        format,
        components,
        data,
    };

    Some(tag)
}

pub fn parse_entries<'a>(endian: &'a Endian, ifd: &'a [u8]) -> Option<Vec<RawTag<'a>>> {
    // the first two bytes are the record count
    let count_range_end = 2;
    let entry_size = 12;
    let count_range = 0..count_range_end;
    let count_bytes = &ifd[count_range];

    let count = bytes_to_u16(endian, count_bytes, 0)?;

    let entries: Vec<RawTag<'a>> = (0..count)
        .filter_map(|c| {
            let start = count_range_end + ((c as usize) * entry_size);
            let end = start + entry_size;

            parse_entry(&ifd[start..end])
        })
        .collect();

    Some(entries)
}

impl<'a> Exif<'a> {
    pub fn get_entries(&self) -> Option<Vec<RawTag>> {
        let endian = &self.endian;
        let ifd = self.ifd;

        parse_entries(endian, ifd)
    }
}
