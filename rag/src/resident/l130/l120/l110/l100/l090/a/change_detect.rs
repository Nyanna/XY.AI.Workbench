//! File change detection: stamp, size and times, racy recordings, and the fast content hash.
use std::io::{ErrorKind, Read};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use crate::resident::l130::l120::l110::l100::l090::l080::a::index_format::{
    fnv1a64_extend, IndexHeader, FNV_OFFSET, HASH_ALGORITHM, HASH_VERSION,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::change_stamp::Stamp;
pub const RACY_WINDOW: Duration = Duration::from_secs(2);
const READ_BLOCK_SIZE: usize = 64 * 1024;
const NO_VALUE: &str = "keine";
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileMeta {
    pub size: u64,
    pub mtime: SystemTime,
    pub ctime: Option<SystemTime>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileRecord {
    pub meta: FileMeta,
    /// Reference time of the iteration in which the file was recorded.
    pub reference: SystemTime,
    pub stamp: Option<Stamp>,
    pub content_hash: Option<u64>,
}
impl FileRecord {
    /// A recording is racy when its latest timestamp lies within `RACY_WINDOW` before the
    /// reference time, or at or after it. A time underflow counts as not racy.
    pub fn is_racy(&self) -> bool {
        let latest = match self.meta.ctime {
            Some(ctime) => self.meta.mtime.max(ctime),
            None => self.meta.mtime,
        };
        match self.reference.checked_sub(RACY_WINDOW) {
            Some(threshold) => latest >= threshold,
            None => false,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckReason {
    StampDiffers,
    SizeDiffers,
    MtimeDiffers,
    CtimeDiffers,
    Racy,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeCheck {
    Unchanged,
    /// A content check is required. `diagnostic` is a German log text, never empty.
    ContentCheckRequired { reason: CheckReason, diagnostic: String },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentCheck {
    Unchanged,
    Changed,
    NoComparableHash,
}
fn format_time(t: SystemTime) -> String {
    match t.duration_since(UNIX_EPOCH) {
        Ok(d) => format!("{}.{:09}s seit Epoche", d.as_secs(), d.subsec_nanos()),
        Err(e) => {
            let d = e.duration();
            format!("-{}.{:09}s seit Epoche", d.as_secs(), d.subsec_nanos())
        }
    }
}
fn format_optional_time(t: Option<SystemTime>) -> String {
    t.map_or_else(|| NO_VALUE.to_owned(), format_time)
}
fn required(reason: CheckReason, diagnostic: String) -> ChangeCheck {
    ChangeCheck::ContentCheckRequired {
        reason,
        diagnostic,
    }
}
/// Fallback chain without content hash. The first mismatch wins; an equal stamp alone
/// never yields `Unchanged`.
pub fn check_file(
    record: &FileRecord,
    current: &FileMeta,
    current_stamp: Option<Stamp>,
) -> ChangeCheck {
    if let (Some(recorded), Some(now)) = (record.stamp, current_stamp) {
        if recorded != now {
            return required(
                CheckReason::StampDiffers,
                format!(
                    "Change-Stamp abweichend: aufgezeichnet={}, aktuell={}", recorded.0,
                    now.0
                ),
            );
        }
    }
    let recorded = &record.meta;
    if recorded.size != current.size {
        return required(
            CheckReason::SizeDiffers,
            format!(
                "Größe abweichend: aufgezeichnet={} Byte, aktuell={} Byte", recorded
                .size, current.size
            ),
        );
    }
    if recorded.mtime != current.mtime {
        return required(
            CheckReason::MtimeDiffers,
            format!(
                "Änderungszeit (mtime) abweichend: aufgezeichnet={}, aktuell={}",
                format_time(recorded.mtime), format_time(current.mtime)
            ),
        );
    }
    if recorded.ctime != current.ctime {
        return required(
            CheckReason::CtimeDiffers,
            format!(
                "Statuszeit (ctime) abweichend: aufgezeichnet={}, aktuell={}",
                format_optional_time(recorded.ctime), format_optional_time(current.ctime)
            ),
        );
    }
    if record.is_racy() {
        return required(
            CheckReason::Racy,
            format!(
                "Racy-Aufzeichnung: mtime={}, ctime={}, Referenzzeitpunkt={}, Fenster={}s",
                format_time(recorded.mtime), format_optional_time(recorded.ctime),
                format_time(record.reference), RACY_WINDOW.as_secs()
            ),
        );
    }
    ChangeCheck::Unchanged
}
/// True when the header's hash algorithm and version match this module's hash.
pub fn hash_matches_header(header: &IndexHeader) -> bool {
    header.hash_algorithm == HASH_ALGORITHM && header.hash_version == HASH_VERSION
}
/// Compares a freshly computed hash with the recorded one. Without a comparable hash the
/// caller must treat the file as changed.
pub fn compare_content(
    record: &FileRecord,
    header: &IndexHeader,
    new_hash: u64,
) -> ContentCheck {
    match record.content_hash {
        Some(recorded) if hash_matches_header(header) => {
            if recorded == new_hash {
                ContentCheck::Unchanged
            } else {
                ContentCheck::Changed
            }
        }
        _ => ContentCheck::NoComparableHash,
    }
}
/// Fast non-cryptographic 64-bit hash (FNV-1a; `HashAlgorithm::Fast64`, `HASH_VERSION`).
/// Changing the algorithm requires bumping `HASH_VERSION`.
pub fn content_hash(bytes: &[u8]) -> u64 {
    fnv1a64_extend(FNV_OFFSET, bytes)
}
/// Streaming variant of `content_hash`; reads in blocks and retries on `Interrupted`.
pub fn content_hash_reader<R: Read>(mut r: R) -> std::io::Result<u64> {
    let mut buf = vec![0u8; READ_BLOCK_SIZE];
    let mut state = FNV_OFFSET;
    loop {
        match r.read(&mut buf) {
            Ok(0) => return Ok(state),
            Ok(n) => {
                let chunk = buf.get(..n).unwrap_or(&buf);
                state = fnv1a64_extend(state, chunk);
            }
            Err(e) if e.kind() == ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
}
#[cfg(test)]
#[path = "test/change_detect.rs"]
mod tests;
