use crate::traits::{Endian, EndianRead};

#[derive(Debug, Clone)]
pub enum ExifValue<'a> {
    UnsignedByte(u8),
    AsciiString(String),
    UnsignedShort(u16),
    UnsignedLong(u32),
    UnsignedRational(u32, u32),
    SignedByte(i8),
    Undefined(&'a [u8]),
    SignedShort(i16),
    SignedLong(i32),
    SignedRational(i32, i32),
    SingleFloat(f32),
    DoubleFloat(f64),
}

pub fn full_bytes_string(bytes: &[u8]) -> Option<String> {
    String::from_utf8(bytes.to_vec()).ok()
}

pub fn bytes_to_unsigned_byte<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = u8::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::UnsignedByte(value))
}

pub fn bytes_to_ascii_string(bytes: &[u8]) -> Option<ExifValue> {
    let value = full_bytes_string(bytes)?.replace("\0", "");

    Some(ExifValue::AsciiString(value))
}

pub fn bytes_to_unsigned_short<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = u16::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::UnsignedShort(value))
}

pub fn bytes_to_unsigned_long<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = u32::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::UnsignedLong(value))
}

pub fn bytes_to_unsigned_rational<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let num_bytes = bytes.get(0..4)?;
    let den_bytes = bytes.get(4..8)?;

    let numerator = u32::from_endian_bytes(endian, num_bytes)?;
    let denominator = u32::from_endian_bytes(endian, den_bytes)?;

    Some(ExifValue::UnsignedRational(numerator, denominator))
}

pub fn bytes_to_signed_byte<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = i8::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::SignedByte(value))
}

pub fn bytes_to_undefined(bytes: &[u8]) -> Option<ExifValue> {
    Some(ExifValue::Undefined(bytes))
}

pub fn bytes_to_signed_short<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = i16::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::SignedShort(value))
}

pub fn bytes_to_signed_long<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = i32::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::SignedLong(value))
}

pub fn bytes_to_signed_rational<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let num_bytes = bytes.get(0..4)?;
    let den_bytes = bytes.get(4..8)?;

    let numerator = i32::from_endian_bytes(endian, num_bytes)?;
    let denominator = i32::from_endian_bytes(endian, den_bytes)?;

    Some(ExifValue::SignedRational(numerator, denominator))
}

pub fn bytes_to_single_float<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = f32::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::SingleFloat(value))
}

pub fn bytes_to_double_float<'a>(endian: &Endian, bytes: &'a [u8]) -> Option<ExifValue<'a>> {
    let value = f64::from_endian_bytes(endian, bytes)?;

    Some(ExifValue::DoubleFloat(value))
}

fn range(bytes: &[u8], start: usize, count: usize) -> Option<&[u8]> {
    bytes.get(start..(start + count))
}

macro_rules! impl_endian_read (( $($type:ident),* ) => {
    $(
        impl EndianRead for $type {
            fn from_endian_bytes(endian: &Endian, bytes: &[u8]) -> Option<Self>
            where
                Self: std::marker::Sized,
            {
                match endian {
                    Endian::Big => {
                        let slice = range(bytes, 0, std::mem::size_of::<Self>())?
                            .try_into()
                            .ok()?;

                        Some($type::from_be_bytes(slice))
                    }
                    Endian::Little => {
                        let slice = range(bytes, 0, std::mem::size_of::<Self>())?
                            .try_into()
                            .ok()?;

                        Some($type::from_le_bytes(slice))
                    }
                }
            }

            fn from_offset_endian_bytes(endian: &Endian, bytes: &[u8], offset: usize) -> Option<Self>
            where
                Self: std::marker::Sized
            {
                let relative = &bytes.get(offset..)?;
                Self::from_endian_bytes(endian, relative)
            }
        }
    )*
});

impl_endian_read!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
