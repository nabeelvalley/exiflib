#[derive(Debug)]
pub enum Endian {
    Big,
    Little,
}

/// Idea for trait based on the one from this playground
/// [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=63664a7dc68fb06c168975ada940153f)
pub trait EndianRead {
    fn from_endian_bytes(endian: &Endian, bytes: &[u8]) -> Option<Self>
    where
        Self: std::marker::Sized;

    fn from_offset_endian_bytes(endian: &Endian, bytes: &[u8], offset: usize) -> Option<Self>
    where
        Self: std::marker::Sized;
}
