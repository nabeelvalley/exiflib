/// Find the start offset of a pattern in the provided bytes if it exists
/// [From StackOverflow](https://stackoverflow.com/questions/35901547/how-can-i-find-a-subsequence-in-a-u8-slice)
pub fn get_sequence_offset(bytes: &[u8], pattern: &[u8]) -> Option<usize> {
    bytes
        .windows(pattern.len())
        .position(|window| window == pattern)
}
