//! Uniform concurrent byte access to memory-domain regions and the tensor reference built on it.
use super::super::l010::layout::{DType, Layout, LayoutError};
use std::fmt;
use std::sync::Arc;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessError {
    /// `offset + len > region_len`, overflow included.
    OutOfBounds { offset: usize, len: usize, region_len: usize },
    /// Domain failure; the text comes from the domain.
    Backend(String),
}
impl fmt::Display for AccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccessError::OutOfBounds { offset, len, region_len } => {
                write!(
                    f,
                    "Zugriff außerhalb der Region: offset={offset} len={len} region_len={region_len}"
                )
            }
            AccessError::Backend(text) => {
                write!(f, "Speicherzugriff fehlgeschlagen: {text}")
            }
        }
    }
}
impl std::error::Error for AccessError {}
/// Implemented by memory domains. All methods take `&self` and may be called concurrently.
/// `offset + len` is already checked against `len()` when the backend is called.
pub trait RegionBackend: Send + Sync + fmt::Debug {
    fn len(&self) -> usize;
    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError>;
    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError>;
}
/// Cheaply clonable handle to a region. Clones refer to the same bytes.
#[derive(Debug, Clone)]
pub struct Region {
    backend: Arc<dyn RegionBackend>,
}
impl Region {
    pub fn new(backend: Arc<dyn RegionBackend>) -> Region {
        Region { backend }
    }
    pub fn len(&self) -> usize {
        self.backend.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// Reads `dst.len()` bytes starting at `offset` into `dst`.
    pub fn read(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        if self.check_range(offset, dst.len())? {
            self.backend.read_raw(offset, dst)
        } else {
            Ok(())
        }
    }
    /// Writes `src.len()` bytes into the region starting at `offset`.
    pub fn write(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        if self.check_range(offset, src.len())? {
            self.backend.write_raw(offset, src)
        } else {
            Ok(())
        }
    }
    /// True if both handles reference the same backend.
    pub fn same_region(&self, other: &Region) -> bool {
        Arc::ptr_eq(&self.backend, &other.backend)
    }
    /// `Ok(true)` if the backend must be called, `Ok(false)` for an empty in-bounds access.
    fn check_range(&self, offset: usize, len: usize) -> Result<bool, AccessError> {
        let region_len = self.backend.len();
        match offset.checked_add(len) {
            Some(end) if end <= region_len => Ok(len != 0),
            _ => {
                Err(AccessError::OutOfBounds {
                    offset,
                    len,
                    region_len,
                })
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TensorRefError {
    Layout(LayoutError),
    /// The byte range occupied by the layout (offset included) is not inside the region.
    ExceedsRegion { required_end: usize, region_len: usize },
}
impl fmt::Display for TensorRefError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TensorRefError::Layout(e) => write!(f, "invalid tensor layout: {e}"),
            TensorRefError::ExceedsRegion { required_end, region_len } => {
                write!(
                    f,
                    "tensor exceeds region: required_end={required_end} region_len={region_len}"
                )
            }
        }
    }
}
impl std::error::Error for TensorRefError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TensorRefError::Layout(e) => Some(e),
            TensorRefError::ExceedsRegion { .. } => None,
        }
    }
}
impl From<LayoutError> for TensorRefError {
    fn from(e: LayoutError) -> Self {
        TensorRefError::Layout(e)
    }
}
/// Tensor reference: region, layout and the byte offset of the origin inside the region.
#[derive(Debug, Clone)]
pub struct TensorRef {
    region: Region,
    layout: Layout,
    offset_bytes: usize,
    end_bytes: usize,
}
/// Bytes spanned from the origin to the end of the last element; `None` on overflow.
fn span_bytes(layout: &Layout) -> Option<usize> {
    if layout.shape().iter().any(|&d| d == 0) {
        return Some(0);
    }
    let mut span = layout.dtype().size_in_bytes();
    for (&d, &s) in layout.shape().iter().zip(layout.strides()) {
        span = span.checked_add((d - 1).checked_mul(s)?)?;
    }
    Some(span)
}
impl TensorRef {
    pub fn new(region: Region, layout: Layout) -> Result<TensorRef, TensorRefError> {
        TensorRef::with_offset(region, layout, 0)
    }
    pub fn with_offset(
        region: Region,
        layout: Layout,
        offset_bytes: usize,
    ) -> Result<TensorRef, TensorRefError> {
        let region_len = region.len();
        let end_bytes = span_bytes(&layout)
            .and_then(|span| offset_bytes.checked_add(span))
            .ok_or(TensorRefError::ExceedsRegion {
                required_end: usize::MAX,
                region_len,
            })?;
        if end_bytes > region_len {
            return Err(TensorRefError::ExceedsRegion {
                required_end: end_bytes,
                region_len,
            });
        }
        Ok(TensorRef {
            region,
            layout,
            offset_bytes,
            end_bytes,
        })
    }
    pub fn region(&self) -> &Region {
        &self.region
    }
    pub fn layout(&self) -> &Layout {
        &self.layout
    }
    pub fn offset_bytes(&self) -> usize {
        self.offset_bytes
    }
    pub fn dtype(&self) -> DType {
        self.layout.dtype()
    }
    /// Exclusive end of the bytes the tensor can reach: offset plus span.
    pub fn end_bytes(&self) -> usize {
        self.end_bytes
    }
    pub fn transpose(
        &self,
        dim1: usize,
        dim2: usize,
    ) -> Result<TensorRef, TensorRefError> {
        let layout = self.layout.transpose(dim1, dim2)?;
        TensorRef::with_offset(self.region.clone(), layout, self.offset_bytes)
    }
    pub fn narrow(
        &self,
        dim: usize,
        start: usize,
        len: usize,
    ) -> Result<TensorRef, TensorRefError> {
        let layout = self.layout.narrow(dim, start, len)?;
        let overflow = TensorRefError::ExceedsRegion {
            required_end: usize::MAX,
            region_len: self.region.len(),
        };
        if self.layout.strides()[dim].checked_mul(start).is_none() {
            return Err(overflow);
        }
        let delta = self.layout.narrow_offset_bytes(dim, start)?;
        let offset = self.offset_bytes.checked_add(delta).ok_or(overflow)?;
        TensorRef::with_offset(self.region.clone(), layout, offset)
    }
}
#[cfg(test)]
#[path = "test/memory_access.rs"]
mod tests;
