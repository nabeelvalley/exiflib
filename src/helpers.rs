use std::ops::Range;

#[derive(Debug)]
pub enum Endian {
    Big,
    Little,
}

pub fn bytes_to_string(bytes: &[u8], range: Range<usize>) -> Option<String> {
    String::from_utf8(bytes.get(range)?.to_vec()).ok()
}

pub fn full_bytes_to_string(bytes: &[u8]) -> Option<String> {
    String::from_utf8(bytes.to_vec()).ok()
}

pub fn bytes_to_u32_be(bytes: &[u8], start: usize) -> Option<u32> {
    let u_bytes = bytes[start..(start + 4)].try_into().ok()?;

    Some(u32::from_be_bytes(u_bytes))
}

pub fn bytes_to_u32_le(bytes: &[u8], start: usize) -> Option<u32> {
    let u_bytes = bytes[start..(start + 4)].try_into().ok()?;

    Some(u32::from_le_bytes(u_bytes))
}

/// Find the start offset of a pattern in the provided bytes if it exists
/// [From StackOverflow](https://stackoverflow.com/questions/35901547/how-can-i-find-a-subsequence-in-a-u8-slice)
pub fn get_sequence_offset(bytes: &[u8], pattern: &[u8]) -> Option<usize> {
    bytes
        .windows(pattern.len())
        .position(|window| window == pattern)
}

/// Idea for trait based on the one from this playground
/// [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=63664a7dc68fb06c168975ada940153f)
pub trait EndianRead {
    fn from_endian_bytes(endian: &Endian, bytes: &[u8]) -> Option<Self>
    where
        Self: std::marker::Sized;
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
        }
    )*
});

impl_endian_read!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);
