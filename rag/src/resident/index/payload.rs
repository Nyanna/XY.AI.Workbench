//! Per-entry back-reference to the source text and its integrity checksum.

use xxhash_rust::xxh3::Xxh3;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Payload {
    pub target_file: String,
    pub char_start: u64,
    pub char_end: u64,
}

impl Payload {
    pub fn new(target_file: impl Into<String>, char_start: u64, char_end: u64) -> Self {
        Self { target_file: target_file.into(), char_start, char_end }
    }

    /// xxh3-64 over vector bytes and all payload fields; any bit flip in either is detectable.
    pub fn checksum(&self, vector: &[u8]) -> u64 {
        let mut h = Xxh3::new();
        h.update(&(vector.len() as u64).to_le_bytes());
        h.update(vector);
        h.update(&(self.target_file.len() as u64).to_le_bytes());
        h.update(self.target_file.as_bytes());
        h.update(&self.char_start.to_le_bytes());
        h.update(&self.char_end.to_le_bytes());
        h.digest()
    }
}
