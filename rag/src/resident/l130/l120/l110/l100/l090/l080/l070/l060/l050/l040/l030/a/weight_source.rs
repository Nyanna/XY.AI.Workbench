//! Memory-mapped safetensors weight files and verbatim copy of selected tensors into a weights arena region.

use super::super::l020::a::arenas::{Arena, ArenaKind};
use super::super::l020::a::memory_access::{AccessError, Region, TensorRef, TensorRefError};
use super::super::l020::l010::layout::{DType, Layout, LayoutError};
use memmap2::Mmap;
use serde::de::{self, IgnoredAny, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fs::File;
use std::path::{Path, PathBuf};

/// Alignment of every tensor start inside the arena (bytes).
pub const WEIGHT_ALIGN: u64 = 256;

const METADATA_KEY: &str = "__metadata__";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeightInfo {
    pub name: String,
    pub dtype: DType,
    pub shape: Vec<usize>,
    /// Byte range relative to the start of the data section: [begin, end)
    pub data_begin: u64,
    pub data_end: u64,
}

#[derive(Debug)]
pub enum WeightError {
    Io { path: PathBuf, reason: String },
    InvalidFormat { path: PathBuf, reason: String },
    Dtype(LayoutError),
    NotFound { name: String },
    ArenaKindMismatch { got: ArenaKind },
    CapacityExceeded { required_end: u64, arena_capacity: u64, region_len: u64 },
    Access(AccessError),
    TensorRef(TensorRefError),
}

impl fmt::Display for WeightError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WeightError::Io { path, reason } => {
                write!(f, "weight file I/O error at {}: {}", path.display(), reason)
            }
            WeightError::InvalidFormat { path, reason } => {
                write!(f, "invalid safetensors file {}: {}", path.display(), reason)
            }
            WeightError::Dtype(e) => write!(f, "unsupported weight dtype: {e:?}"),
            WeightError::NotFound { name } => write!(f, "tensor not found: {name}"),
            WeightError::ArenaKindMismatch { got } => {
                write!(f, "arena kind mismatch: expected Weights, got {got:?}")
            }
            WeightError::CapacityExceeded { required_end, arena_capacity, region_len } => write!(
                f,
                "weights do not fit: required end {required_end}, arena capacity {arena_capacity}, region length {region_len}"
            ),
            WeightError::Access(e) => write!(f, "region access failed: {e}"),
            WeightError::TensorRef(e) => write!(f, "tensor reference failed: {e}"),
        }
    }
}

impl std::error::Error for WeightError {}

impl From<AccessError> for WeightError {
    fn from(e: AccessError) -> Self {
        WeightError::Access(e)
    }
}

impl From<TensorRefError> for WeightError {
    fn from(e: TensorRefError) -> Self {
        WeightError::TensorRef(e)
    }
}

#[derive(Deserialize)]
struct RawEntry {
    dtype: String,
    shape: Vec<usize>,
    data_offsets: [u64; 2],
}

/// Header entries in file order; duplicate keys are rejected while parsing.
struct RawHeader(Vec<(String, RawEntry)>);

impl<'de> Deserialize<'de> for RawHeader {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = RawHeader;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a safetensors header object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<RawHeader, A::Error> {
                let mut seen: HashSet<String> = HashSet::new();
                let mut out = Vec::new();
                while let Some(key) = map.next_key::<String>()? {
                    if !seen.insert(key.clone()) {
                        return Err(de::Error::custom(format!("duplicate key {key}")));
                    }
                    if key == METADATA_KEY {
                        map.next_value::<IgnoredAny>()?;
                    } else {
                        let entry = map.next_value::<RawEntry>()?;
                        out.push((key, entry));
                    }
                }
                Ok(RawHeader(out))
            }
        }
        d.deserialize_map(V)
    }
}

/// Mapped safetensors file. Immutable after `open`.
#[derive(Debug)]
pub struct WeightFile {
    mmap: Mmap,
    path: PathBuf,
    data_start: usize,
    infos: Vec<WeightInfo>,
    index: HashMap<String, usize>,
}

fn invalid(path: &Path, reason: impl Into<String>) -> WeightError {
    WeightError::InvalidFormat { path: path.to_path_buf(), reason: reason.into() }
}

fn io_err(path: &Path, e: std::io::Error) -> WeightError {
    WeightError::Io { path: path.to_path_buf(), reason: e.to_string() }
}

fn align_up(v: u64) -> Option<u64> {
    v.checked_add(WEIGHT_ALIGN - 1).map(|x| x & !(WEIGHT_ALIGN - 1))
}

impl WeightFile {
    /// Opens read-only, maps the whole file and validates the header. No tensor data is copied.
    pub fn open(path: &Path) -> Result<WeightFile, WeightError> {
        let file = File::open(path).map_err(|e| io_err(path, e))?;
        let file_len = file.metadata().map_err(|e| io_err(path, e))?.len();
        if file_len < 8 {
            return Err(invalid(path, "file shorter than 8 bytes"));
        }
        // SAFETY: the mapping is read-only. The caller guarantees that the file is neither
        // modified nor truncated by anyone else while this `WeightFile` is alive.
        let mmap = unsafe { Mmap::map(&file) }.map_err(|e| io_err(path, e))?;
        let total = mmap.len() as u64;
        if total < 8 {
            return Err(invalid(path, "file shorter than 8 bytes"));
        }
        let mut len_bytes = [0u8; 8];
        len_bytes.copy_from_slice(&mmap[..8]);
        let header_len = u64::from_le_bytes(len_bytes);
        let data_start64 = header_len
            .checked_add(8)
            .filter(|e| *e <= total)
            .ok_or_else(|| invalid(path, "header length exceeds file size"))?;
        let data_start = usize::try_from(data_start64)
            .map_err(|_| invalid(path, "header length exceeds address space"))?;
        let data_len = total - data_start64;

        let raw: RawHeader = serde_json::from_slice(&mmap[8..data_start])
            .map_err(|e| invalid(path, format!("header JSON: {e}")))?;

        let mut infos = Vec::with_capacity(raw.0.len());
        for (name, e) in raw.0 {
            let source = format!("{}:{}", path.display(), name);
            let dtype = DType::from_safetensors(&e.dtype, &source).map_err(WeightError::Dtype)?;
            let [begin, end] = e.data_offsets;
            if begin > end {
                return Err(invalid(path, format!("tensor {name}: begin offset exceeds end offset")));
            }
            if end > data_len {
                return Err(invalid(path, format!("tensor {name}: end offset beyond data section")));
            }
            let elems = e
                .shape
                .iter()
                .try_fold(1u64, |acc, d| acc.checked_mul(*d as u64))
                .ok_or_else(|| invalid(path, format!("tensor {name}: shape overflow")))?;
            let expected = elems
                .checked_mul(dtype.size_in_bytes() as u64)
                .ok_or_else(|| invalid(path, format!("tensor {name}: size overflow")))?;
            if end - begin != expected {
                return Err(invalid(
                    path,
                    format!("tensor {name}: byte range does not match shape and dtype"),
                ));
            }
            infos.push(WeightInfo { name, dtype, shape: e.shape, data_begin: begin, data_end: end });
        }

        infos.sort_by(|a, b| {
            (a.data_begin, a.data_end, &a.name).cmp(&(b.data_begin, b.data_end, &b.name))
        });
        let mut cursor = 0u64;
        for i in &infos {
            if i.data_begin < cursor {
                return Err(invalid(path, format!("tensor {} overlaps another tensor", i.name)));
            }
            if i.data_begin > cursor {
                return Err(invalid(path, format!("gap before tensor {}", i.name)));
            }
            cursor = i.data_end;
        }
        if cursor != data_len {
            return Err(invalid(path, "data section is not fully covered by tensors"));
        }

        let index = infos.iter().enumerate().map(|(n, i)| (i.name.clone(), n)).collect();
        Ok(WeightFile { mmap, path: path.to_path_buf(), data_start, infos, index })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    /// All tensors in file order. `__metadata__` is not included.
    pub fn tensors(&self) -> &[WeightInfo] {
        &self.infos
    }

    pub fn info(&self, name: &str) -> Option<&WeightInfo> {
        self.index.get(name).map(|&i| &self.infos[i])
    }

    fn bytes_of(&self, info: &WeightInfo) -> &[u8] {
        // Offsets were validated against the mapping length in `open`.
        let b = self.data_start + info.data_begin as usize;
        let e = self.data_start + info.data_end as usize;
        &self.mmap[b..e]
    }

    /// Zero-copy slice into the mapping.
    pub fn raw_bytes(&self, name: &str) -> Result<&[u8], WeightError> {
        let info = self.info(name).ok_or_else(|| WeightError::NotFound { name: name.to_owned() })?;
        Ok(self.bytes_of(info))
    }

    /// Copies all tensors selected by `select` into `region` starting at arena offset `base_offset`.
    pub fn copy_to_arena(
        &self,
        arena: &Arena,
        region: &Region,
        base_offset: u64,
        select: &dyn Fn(&str) -> bool,
    ) -> Result<LoadedWeights, WeightError> {
        if arena.kind != ArenaKind::Weights {
            return Err(WeightError::ArenaKindMismatch { got: arena.kind });
        }
        let region_len = region.len() as u64;
        let exceeded = |required_end: u64| WeightError::CapacityExceeded {
            required_end,
            arena_capacity: arena.capacity,
            region_len,
        };

        let mut plan: Vec<(&WeightInfo, u64)> = Vec::new();
        let mut cursor = base_offset;
        for info in &self.infos {
            if !select(&info.name) {
                continue;
            }
            let start = align_up(cursor).ok_or_else(|| exceeded(u64::MAX))?;
            let end = start
                .checked_add(info.data_end - info.data_begin)
                .ok_or_else(|| exceeded(u64::MAX))?;
            plan.push((info, start));
            cursor = end;
        }
        if plan.is_empty() {
            return Ok(LoadedWeights { tensors: Vec::new(), end_offset: base_offset });
        }
        if cursor > arena.capacity || cursor > region_len {
            return Err(exceeded(cursor));
        }

        let mut tensors = Vec::with_capacity(plan.len());
        for (info, start) in plan {
            let offset = usize::try_from(start).map_err(|_| exceeded(cursor))?;
            region.write(offset, self.bytes_of(info))?;
            let layout = Layout::contiguous(&info.shape, info.dtype);
            let t = TensorRef::with_offset(region.clone(), layout, offset)?;
            tensors.push((info.name.clone(), t));
        }
        Ok(LoadedWeights { tensors, end_offset: cursor })
    }
}

#[derive(Debug, Clone)]
pub struct LoadedWeights {
    /// In file order; layout is contiguous; the offset is the placement position.
    pub tensors: Vec<(String, TensorRef)>,
    /// First free byte after the last tensor (no trailing alignment).
    pub end_offset: u64,
}

impl LoadedWeights {
    pub fn get(&self, name: &str) -> Option<&TensorRef> {
        self.tensors.iter().find(|(n, _)| n == name).map(|(_, t)| t)
    }
}

#[cfg(test)]
#[path = "test/weight_source.rs"]
mod tests;
