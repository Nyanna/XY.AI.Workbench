//! Executes the planner-set materialization flag for a finished device result.

use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};

use super::super::l050::l040::l030::a::region_handle::RegionHandle;
use super::super::l050::l040::l030::l020::a::memory_access::{
    AccessError, Region, RegionBackend, TensorRef, TensorRefError,
};

/// Flag set by the planner.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Materialization {
    /// Copy the result to host memory when it completes.
    ToHost,
    /// The result stays in VRAM; only the reference is passed on.
    KeepOnDevice,
}

/// Finished result in VRAM: reference plus byte access.
#[derive(Clone, Debug)]
pub struct DeviceResult {
    handle: RegionHandle,
    tensor: TensorRef,
}

impl DeviceResult {
    pub fn new(handle: RegionHandle, tensor: TensorRef) -> DeviceResult {
        DeviceResult { handle, tensor }
    }

    pub fn handle(&self) -> RegionHandle {
        self.handle
    }

    pub fn tensor(&self) -> &TensorRef {
        &self.tensor
    }
}

/// Outcome of materialization.
#[derive(Clone, Debug)]
pub enum Materialized {
    /// In host memory; independent copy.
    Host(TensorRef),
    /// Still in VRAM; the unchanged device result.
    Device(DeviceResult),
}

impl Materialized {
    pub fn is_host(&self) -> bool {
        matches!(self, Materialized::Host(_))
    }

    pub fn is_device(&self) -> bool {
        matches!(self, Materialized::Device(_))
    }

    pub fn tensor(&self) -> &TensorRef {
        match self {
            Materialized::Host(t) => t,
            Materialized::Device(d) => d.tensor(),
        }
    }

    /// `Some` only for `Device`.
    pub fn handle(&self) -> Option<RegionHandle> {
        match self {
            Materialized::Host(_) => None,
            Materialized::Device(d) => Some(d.handle()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaterializeError {
    Access(AccessError),
    Tensor(TensorRefError),
}

impl fmt::Display for MaterializeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaterializeError::Access(e) => write!(f, "materialization access failed: {e}"),
            MaterializeError::Tensor(e) => write!(f, "materialization tensor failed: {e}"),
        }
    }
}

impl std::error::Error for MaterializeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MaterializeError::Access(e) => Some(e),
            MaterializeError::Tensor(e) => Some(e),
        }
    }
}

impl From<AccessError> for MaterializeError {
    fn from(e: AccessError) -> Self {
        MaterializeError::Access(e)
    }
}

impl From<TensorRefError> for MaterializeError {
    fn from(e: TensorRefError) -> Self {
        MaterializeError::Tensor(e)
    }
}

#[derive(Debug)]
struct HostBuffer {
    bytes: Mutex<Vec<u8>>,
    len: usize,
}

impl HostBuffer {
    fn new(bytes: Vec<u8>) -> HostBuffer {
        let len = bytes.len();
        HostBuffer { bytes: Mutex::new(bytes), len }
    }

    fn lock(&self) -> MutexGuard<'_, Vec<u8>> {
        self.bytes.lock().unwrap_or_else(|p| p.into_inner())
    }

    fn range(&self, offset: usize, len: usize) -> Result<std::ops::Range<usize>, AccessError> {
        match offset.checked_add(len) {
            Some(end) if end <= self.len => Ok(offset..end),
            _ => Err(AccessError::OutOfBounds { offset, len, region_len: self.len }),
        }
    }
}

impl RegionBackend for HostBuffer {
    fn len(&self) -> usize {
        self.len
    }

    fn read_raw(&self, offset: usize, dst: &mut [u8]) -> Result<(), AccessError> {
        let r = self.range(offset, dst.len())?;
        dst.copy_from_slice(&self.lock()[r]);
        Ok(())
    }

    fn write_raw(&self, offset: usize, src: &[u8]) -> Result<(), AccessError> {
        let r = self.range(offset, src.len())?;
        self.lock()[r].copy_from_slice(src);
        Ok(())
    }
}

/// Executes the flag. Synchronous; blocks only for the duration of the region read.
pub fn materialize(
    flag: Materialization,
    result: DeviceResult,
) -> Result<Materialized, MaterializeError> {
    match flag {
        Materialization::KeepOnDevice => Ok(Materialized::Device(result)),
        Materialization::ToHost => {
            let src = result.tensor();
            let start = src.offset_bytes();
            let span = src.end_bytes() - start;
            let mut buf = vec![0u8; span];
            if span > 0 {
                src.region().read(start, &mut buf)?;
            }
            let region = Region::new(Arc::new(HostBuffer::new(buf)));
            let t = TensorRef::new(region, src.layout().clone())?;
            Ok(Materialized::Host(t))
        }
    }
}

#[cfg(test)]
#[path = "test/materialization.rs"]
mod tests;
