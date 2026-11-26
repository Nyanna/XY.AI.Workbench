#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct MetaFields {
    pub target_file: String,
    pub char_start: u64,
    pub char_end: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum MetaError {
    #[error("meta truncated: need {needed} bytes, have {have}")]
    Truncated { needed: u64, have: usize },
    #[error("meta has {extra} trailing bytes")]
    Trailing { extra: usize },
    #[error("meta target_file is not valid UTF-8")]
    Utf8,
}

const LEN_BYTES: usize = 8;
const TAIL_BYTES: usize = 16;
const MIN_BYTES: u64 = (LEN_BYTES + TAIL_BYTES) as u64;

pub fn encode_meta(fields: &MetaFields) -> Vec<u8> {
    encode_meta_parts(&fields.target_file, fields.char_start, fields.char_end)
}

pub fn encode_meta_parts(target_file: &str, char_start: u64, char_end: u64) -> Vec<u8> {
    if target_file.is_empty() && char_start == 0 && char_end == 0 {
        return Vec::new();
    }
    let mut out = Vec::with_capacity(MIN_BYTES as usize + target_file.len());
    out.extend_from_slice(&(target_file.len() as u64).to_le_bytes());
    out.extend_from_slice(target_file.as_bytes());
    out.extend_from_slice(&char_start.to_le_bytes());
    out.extend_from_slice(&char_end.to_le_bytes());
    out
}

pub fn decode_meta(bytes: &[u8]) -> Result<MetaFields, MetaError> {
    if bytes.is_empty() {
        return Ok(MetaFields::default());
    }
    let have = bytes.len();
    let Some(head) = bytes.get(..LEN_BYTES) else {
        return Err(MetaError::Truncated {
            needed: MIN_BYTES,
            have,
        });
    };
    let name_len = read_u643(head);
    let needed = MIN_BYTES.saturating_add(name_len);
    let rest = &bytes[LEN_BYTES..];
    let Some(name_len) = usize::try_from(name_len)
        .ok()
        .filter(|&n| n <= rest.len())
    else {
        return Err(MetaError::Truncated { needed, have });
    };
    let (name, tail) = rest.split_at(name_len);
    if tail.len() < TAIL_BYTES {
        return Err(MetaError::Truncated { needed, have });
    }
    if tail.len() > TAIL_BYTES {
        return Err(MetaError::Trailing {
            extra: tail.len() - TAIL_BYTES,
        });
    }
    let target_file = std::str::from_utf8(name)
        .map_err(|_| MetaError::Utf8)?
        .to_owned();
    Ok(MetaFields {
        target_file,
        char_start: read_u643(&tail[..8]),
        char_end: read_u643(&tail[8..]),
    })
}

fn read_u643(bytes: &[u8]) -> u64 {
    let mut buf = [0u8; 8];
    buf.copy_from_slice(&bytes[..8]);
    u64::from_le_bytes(buf)
}

#[cfg(test)]
#[path = "test/meta_codec.rs"]
mod tests;
