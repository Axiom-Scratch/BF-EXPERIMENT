pub fn filter_ops(bytes: &[u8]) -> Vec<u8> {
    bytes
        .iter()
        .copied()
        .filter(|b| matches!(b, b'<' | b'>' | b'+' | b'-' | b'.' | b',' | b'[' | b']'))
        .collect()
}
