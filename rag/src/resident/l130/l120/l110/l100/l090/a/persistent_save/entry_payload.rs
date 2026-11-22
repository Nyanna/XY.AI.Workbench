//! Per-entry source reference and the 64-bit entry checksum (Fast64, version 1).
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::SharedBytes;
use crate::resident::l130::l120::l110::l100::l090::l080::a::index_format::{
    fnv1a64_extend, FNV_OFFSET,
};
use crate::resident::l130::l120::l110::l100::l090::l080::a::meta_codec::{
    self, MetaError, MetaFields,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct SourceRef {
    pub target_file: String,
    pub char_start: u64,
    pub char_end: u64,
}
impl SourceRef {
    pub fn new(
        target_file: impl Into<String>,
        char_start: u64,
        char_end: u64,
    ) -> SourceRef {
        SourceRef {
            target_file: target_file.into(),
            char_start,
            char_end,
        }
    }
    pub fn to_meta(&self) -> Vec<u8> {
        meta_codec::encode_meta(&MetaFields::from(self))
    }
    pub fn from_meta(bytes: &[u8]) -> Result<SourceRef, MetaError> {
        meta_codec::decode_meta(bytes).map(SourceRef::from)
    }
}
impl From<MetaFields> for SourceRef {
    fn from(fields: MetaFields) -> Self {
        SourceRef {
            target_file: fields.target_file,
            char_start: fields.char_start,
            char_end: fields.char_end,
        }
    }
}
impl From<&SourceRef> for MetaFields {
    fn from(source: &SourceRef) -> Self {
        MetaFields {
            target_file: source.target_file.clone(),
            char_start: source.char_start,
            char_end: source.char_end,
        }
    }
}
struct Fnv1a64(u64);
impl Fnv1a64 {
    fn new() -> Self {
        Fnv1a64(FNV_OFFSET)
    }
    fn write(&mut self, bytes: &[u8]) {
        self.0 = fnv1a64_extend(self.0, bytes);
    }
    fn write_u64(&mut self, value: u64) {
        self.write(&value.to_le_bytes());
    }
}
pub fn compute_checksum(vector: &[u8], source: &SourceRef) -> u64 {
    compute_checksum_parts(
        vector,
        &source.target_file,
        source.char_start,
        source.char_end,
    )
}
pub fn compute_checksum_parts(
    vector: &[u8],
    target_file: &str,
    char_start: u64,
    char_end: u64,
) -> u64 {
    let mut hash = Fnv1a64::new();
    hash.write_u64(vector.len() as u64);
    hash.write(vector);
    hash.write_u64(target_file.len() as u64);
    hash.write(target_file.as_bytes());
    hash.write_u64(char_start);
    hash.write_u64(char_end);
    hash.0
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumMismatch {
    pub stored: u64,
    pub computed: u64,
}
impl std::fmt::Display for ChecksumMismatch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, "Checksum stimmt nicht überein: gespeichert {:016x}, berechnet {:016x}",
            self.stored, self.computed
        )
    }
}
impl std::error::Error for ChecksumMismatch {}
pub fn verify_checksum(
    vector: &[u8],
    source: &SourceRef,
    stored: u64,
) -> Result<(), ChecksumMismatch> {
    let computed = compute_checksum(vector, source);
    if computed == stored {
        Ok(())
    } else {
        Err(ChecksumMismatch {
            stored,
            computed,
        })
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntryData {
    pub vector: SharedBytes,
    pub source: SourceRef,
    checksum: u64,
}
impl Default for EntryData {
    fn default() -> Self {
        EntryData::new(SharedBytes::default(), SourceRef::default())
    }
}
impl EntryData {
    pub fn new(vector: SharedBytes, source: SourceRef) -> EntryData {
        let checksum = compute_checksum(&vector, &source);
        EntryData {
            vector,
            source,
            checksum,
        }
    }
    pub fn from_stored(
        vector: SharedBytes,
        source: SourceRef,
        stored_checksum: u64,
    ) -> EntryData {
        EntryData {
            vector,
            source,
            checksum: stored_checksum,
        }
    }
    pub fn checksum(&self) -> u64 {
        self.checksum
    }
    pub fn verify(&self) -> Result<(), ChecksumMismatch> {
        verify_checksum(&self.vector, &self.source, self.checksum)
    }
    pub fn recompute(&mut self) -> u64 {
        self.checksum = compute_checksum(&self.vector, &self.source);
        self.checksum
    }
}
#[cfg(test)]
#[path = "test/entry_payload.rs"]
mod tests;
