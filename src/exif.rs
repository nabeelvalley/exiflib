use std::ops::Range;

use crate::helpers::{
    bytes_to_string, full_bytes_to_string, get_sequence_offset, Endian, EndianRead,
};

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
pub enum TagFormat {
    UnsignedByte,
    AsciiString,
    UnsignedShort,
    UnsignedLong,
    UnsignedRational,
    SignedByte,
    Undefined,
    SignedShort,
    SignedLong,
    SignedRational,
    SingleFloat,
    DoubleFloat,
}

#[derive(Debug)]
pub enum ExifValue<'a> {
    UnsignedByte(u8),
    AsciiString(String),
    UnsignedShort(u32),
    UnsignedLong(u64),
    UnsignedRational(u32, u32),
    SignedByte(i8),
    Undefined(&'a [u8]),
    SignedShort(i32),
    SignedLong(i64),
    SignedRational(i32, i32),
    SingleFloat(f32),
    DoubleFloat(f64),
}

pub struct Exif<'a> {
    pub bytes: &'a [u8],
    /// There are potentially multiple of these but I'm just handling the single case
    pub ifd: &'a [u8],
    endian: Endian,
}

#[derive(Debug)]
pub struct ExifTag<'a> {
    pub tag: u16,
    pub format: TagFormat,
    pub value: ExifValue<'a>,
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
    get_sequence_offset(file, EXIF_MARKER)
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
    let offset = u32::from_endian_bytes(endian, exif.get(IFD_OFFSET_RANGE.start..)?)?;

    // the IFD start is defined as the location from the Endian marker start
    let start = ENDIAN_RANGE.start + (offset as usize);

    Some(&exif[start..])
}

fn parse_entry<'a>(endian: &'a Endian, ifd: &'a [u8], entry: &'a [u8]) -> Option<ExifTag<'a>> {
    let tag = u16::from_endian_bytes(endian, entry)?;
    let components = u32::from_endian_bytes(endian, entry.get(4..)?)?;

    let format_value = u16::from_endian_bytes(endian, entry.get(2..)?)?;
    let format = get_tag_format(&format_value)?;

    let bytes_per_component = get_bytes_per_component(&format);

    let length = components * bytes_per_component;

    let value = if length > 4 {
        let data = entry.get(8..12)?;

        parse_tag_value(&format, endian, data)?
    } else {
        // the value needs to be checked at the offset and used from there
        let offset = u32::from_endian_bytes(endian, entry.get(8..)?)?;
        let start = offset as usize;
        let end = offset as usize;

        let range = start..end;

        let value_bytes = ifd.get(range)?;

        parse_tag_value(&format, endian, value_bytes)?
    };

    let tag = ExifTag { tag, format, value };

    Some(tag)
}

fn get_tag_format(value: &u16) -> Option<TagFormat> {
    match value {
        1 => Some(TagFormat::UnsignedByte),
        2 => Some(TagFormat::AsciiString),
        3 => Some(TagFormat::UnsignedShort),
        4 => Some(TagFormat::UnsignedLong),
        5 => Some(TagFormat::UnsignedRational),
        6 => Some(TagFormat::SignedByte),
        7 => Some(TagFormat::Undefined),
        8 => Some(TagFormat::SignedShort),
        9 => Some(TagFormat::SignedLong),
        10 => Some(TagFormat::SignedRational),
        11 => Some(TagFormat::SingleFloat),
        12 => Some(TagFormat::DoubleFloat),
        _ => None,
    }
}

fn get_bytes_per_component(format: &TagFormat) -> u32 {
    match format {
        TagFormat::UnsignedByte => 1,
        TagFormat::AsciiString => 1,
        TagFormat::UnsignedShort => 2,
        TagFormat::UnsignedLong => 4,
        TagFormat::UnsignedRational => 8,
        TagFormat::SignedByte => 1,
        TagFormat::Undefined => 1,
        TagFormat::SignedShort => 2,
        TagFormat::SignedLong => 4,
        TagFormat::SignedRational => 8,
        TagFormat::SingleFloat => 4,
        TagFormat::DoubleFloat => 8,
    }
}

pub fn parse_entries<'a>(endian: &'a Endian, ifd: &'a [u8]) -> Option<Vec<ExifTag<'a>>> {
    // the first two bytes are the record count
    let count_range_end = 2;
    let entry_size = 12;
    let count_range = 0..count_range_end;
    let count_bytes = &ifd[count_range];

    let count = u16::from_endian_bytes(endian, count_bytes)?;

    let entries: Vec<ExifTag<'a>> = (0..count)
        .filter_map(|c| {
            let start = count_range_end + ((c as usize) * entry_size);
            let end = start + entry_size;

            // TODO: move the entry locating into the parse_entry function
            parse_entry(endian, ifd, &ifd[start..end])
        })
        .collect();

    Some(entries)
}

fn parse_tag_value<'a>(
    format: &TagFormat,
    endian: &'a Endian,
    bytes: &'a [u8],
) -> Option<ExifValue<'a>> {
    match format {
        TagFormat::UnsignedByte => parse_unsigned_byte(endian, bytes),
        TagFormat::AsciiString => parse_ascii_string(bytes),
        TagFormat::UnsignedShort => parse_unsigned_short(endian, bytes),
        TagFormat::UnsignedLong => parse_unsigned_long(endian, bytes),
        TagFormat::UnsignedRational => parse_unsigned_rational(endian, bytes),
        TagFormat::SignedByte => parse_signed_byte(endian, bytes),
        TagFormat::Undefined => parse_undefined(bytes),
        TagFormat::SignedShort => parse_signed_short(endian, bytes),
        TagFormat::SignedLong => parse_signed_long(endian, bytes),
        TagFormat::SignedRational => parse_signed_rational(endian, bytes),
        TagFormat::SingleFloat => parse_single_float(endian, bytes),
        TagFormat::DoubleFloat => parse_double_float(endian, bytes),
    }
}

fn parse_unsigned_byte<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = u8::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::UnsignedByte(value))
}

fn parse_ascii_string(bytes: &[u8]) -> Option<ExifValue> {
    let value = full_bytes_to_string(bytes)?;

    Some(ExifValue::AsciiString(value))
}

fn parse_unsigned_short<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = u32::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::UnsignedShort(value))
}

fn parse_unsigned_long<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = u64::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::UnsignedLong(value))
}

fn parse_unsigned_rational<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let num_bytes = bytes.get(0..4)?;
    let den_bytes = bytes.get(4..8)?;

    let numerator = u32::from_endian_bytes(endian, num_bytes)?;
    let denominator = u32::from_endian_bytes(endian, den_bytes)?;

    Some(ExifValue::UnsignedRational(numerator, denominator))
}

fn parse_signed_byte<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = i8::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::SignedByte(value))
}

fn parse_undefined(bytes: &[u8]) -> Option<ExifValue> {
    Some(ExifValue::Undefined(bytes))
}

fn parse_signed_short<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = i32::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::SignedShort(value))
}

fn parse_signed_long<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = i64::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::SignedLong(value))
}

fn parse_signed_rational<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let num_bytes = bytes.get(0..4)?;
    let den_bytes = bytes.get(4..8)?;

    let numerator = i32::from_endian_bytes(endian, num_bytes)?;
    let denominator = i32::from_endian_bytes(endian, den_bytes)?;

    Some(ExifValue::SignedRational(numerator, denominator))
}

fn parse_single_float<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = f32::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::SingleFloat(value))
}

fn parse_double_float<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = f64::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::DoubleFloat(value))
}

// TODO: remove this or better organize the rest of the code into the impl
impl<'a> Exif<'a> {
    pub fn get_entries(&self) -> Option<Vec<ExifTag>> {
        let endian = &self.endian;
        let ifd = self.ifd;

        parse_entries(endian, ifd)
    }
}
