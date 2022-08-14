use std::ops::Range;

use crate::helpers::get_sequence_offset;
use crate::parsing::{self, ExifValue};
use crate::traits::{Endian, EndianRead};

/// 6 bytes identify the EXIF data start = `Exif\x00\x00`
const EXIF_MARKER: &[u8] = "Exif\0\0".as_bytes();
const ENDIAN_RANGE: Range<usize> = 6..8;
const IFD_OFFSET_RANGE: Range<usize> = 10..14;
/// Each Exif Entry is structured as TTTT FFFF
const EXIF_ENTRY_SIZE: usize = 12;
/// Used to locate the SubIFD which contains additional metadata for the image
const SUB_IFD_TAG_ID: u32 = 0x8769;

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

pub struct Exif<'a> {
    /// IFD starting at either MM or II
    /// > Offsets within the IFD are calculated relative to this starting point
    ifd: &'a [u8],
    ifd0_offset: usize,
    /// The start of the IFD entries (skips past the initial two sizing bytes)
    ifd0_entry_offset: usize,
    ifd0_count: u16,

    endian: Endian,
}

#[derive(Debug)]
pub struct ExifTag<'a> {
    pub tag: u16,
    pub format: TagFormat,
    pub value: ExifValue<'a>,
    pub components: u32,
    pub bytes_per_component: u32,
    pub length: u32,
}

pub fn parse(file: &[u8]) -> Option<Exif> {
    let start = get_exif_start(file)?;

    // TODO: There's probably a way we can find the end of the exif header
    let bytes = file.get(start..)?;

    let endian = get_endian(bytes)?;

    let ifd0_offset = get_ifd_first_entry_offset(&endian, bytes)? as usize;
    let ifd0_entry_offset = ifd0_offset + 2;

    let ifd0_count = u16::from_offset_endian_bytes(&endian, bytes, ifd0_offset)?;

    let ifd = get_ifd_bytes(bytes)?;

    let exif = Exif {
        endian,
        ifd,
        ifd0_offset,
        ifd0_count,
        ifd0_entry_offset,
    };

    Some(exif)
}

impl<'a> Exif<'a> {
    pub fn get_entries(&self) -> Option<Vec<ExifTag>> {
        let endian = &self.endian;
        let ifd = self.ifd;
        let ifd0_entry_offset = self.ifd0_entry_offset;
        let ifd0_count = self.ifd0_count;

        parse_entries(endian, ifd, ifd0_entry_offset, ifd0_count)
    }
}

fn get_endian(exif: &[u8]) -> Option<Endian> {
    let endian = parsing::bytes_to_string(exif, ENDIAN_RANGE)?;

    match endian.as_str() {
        "MM" => Some(Endian::Big),
        "II" => Some(Endian::Little),
        _ => None,
    }
}

// TODO: find a way to find the end of this byte range
fn get_exif_start(file: &[u8]) -> Option<usize> {
    get_sequence_offset(file, EXIF_MARKER)
}

fn get_ifd_first_entry_offset(endian: &Endian, exif: &[u8]) -> Option<u32> {
    let bytes = exif.get(IFD_OFFSET_RANGE.start..)?;
    u32::from_endian_bytes(endian, bytes)
}

fn get_ifd_bytes(exif: &[u8]) -> Option<&[u8]> {
    exif.get(ENDIAN_RANGE.start..)
}

fn parse_entry<'a>(endian: &'a Endian, ifd: &'a [u8], entry: &'a [u8]) -> Option<ExifTag<'a>> {
    let tag = u16::from_endian_bytes(endian, entry)?;

    let components = u32::from_offset_endian_bytes(endian, entry, 4)?;

    let format_value = u16::from_offset_endian_bytes(endian, entry, 2)?;
    let format = get_tag_format(&format_value)?;

    let bytes_per_component = get_bytes_per_component(&format);

    let length = components * bytes_per_component;

    let data = entry.get(8..12)?;

    let value = if length <= 4 {
        parse_tag_value(&format, endian, data)
    } else {
        // the value needs to be checked at the offset and used from there
        let offset = u32::from_endian_bytes(endian, data)?;

        let start = (offset) as usize;
        let end = start + (length) as usize;

        let range = start..end;

        let value_bytes = &ifd[range];
        // let value_bytes = ifd.get(range)?;

        parse_tag_value(&format, endian, value_bytes)
    };

    let tag = ExifTag {
        tag,
        format,
        value: value?,
        components,
        bytes_per_component,
        length,
    };

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

pub fn parse_entries<'a>(
    endian: &'a Endian,
    ifd: &'a [u8],
    ifd0_entry_offset: usize,
    ifd0_count: u16,
) -> Option<Vec<ExifTag<'a>>> {
    let entries: Vec<ExifTag<'a>> = (0..ifd0_count)
        .filter_map(|c| {
            let start = ifd0_entry_offset + ((c as usize) * EXIF_ENTRY_SIZE);
            let end = start + EXIF_ENTRY_SIZE;

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
        TagFormat::UnsignedByte => parsing::bytes_to_unsigned_byte(endian, bytes),
        TagFormat::AsciiString => parsing::bytes_to_ascii_string(bytes),
        TagFormat::UnsignedShort => parsing::bytes_to_unsigned_short(endian, bytes),
        TagFormat::UnsignedLong => parsing::bytes_to_unsigned_long(endian, bytes),
        TagFormat::UnsignedRational => parsing::bytes_to_unsigned_rational(endian, bytes),
        TagFormat::SignedByte => parsing::bytes_to_signed_byte(endian, bytes),
        TagFormat::Undefined => parsing::bytes_to_undefined(bytes),
        TagFormat::SignedShort => parsing::bytes_to_signed_short(endian, bytes),
        TagFormat::SignedLong => parsing::bytes_to_signed_long(endian, bytes),
        TagFormat::SignedRational => parsing::bytes_to_signed_rational(endian, bytes),
        TagFormat::SingleFloat => parsing::bytes_to_single_float(endian, bytes),
        TagFormat::DoubleFloat => parsing::bytes_to_double_float(endian, bytes),
    }
}
