use std::ops::Range;

/// Find the start offset of a pattern in the provided bytes if it exists
/// [From StackOverflow](https://stackoverflow.com/questions/35901547/how-can-i-find-a-subsequence-in-a-u8-slice)
pub fn get_sequence_range(bytes: &[u8], pattern: &[u8]) -> Option<Range<usize>> {
    let start = bytes
        .windows(pattern.len())
        .position(|window| window == pattern)?;

    let end = pattern.len();

    Some(start..end)
}
