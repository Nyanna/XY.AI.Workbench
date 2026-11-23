use std::any::Any;
use std::fmt;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::b::doorbell::Gap;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::memory_access::{
    AccessError, AccessRegion,
};
/// Kernel run state, supplied by the caller.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KernelMode {
    /// The kernel does not read these structures while a gap is open.
    Batch,
    /// The kernel may read during the gap; torn reads are possible.
    Persistent,
}
/// Hierarchy level, used for error attribution and diagnostics only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    Root,
    SegmentTable,
    Segment,
    OffsetTable,
    LookupTable,
}
/// A naturally aligned 8-byte pointer slot in shared memory.
pub struct PtrCell {
    ptr: NonNull<AtomicU64>,
    _keepalive: Arc<dyn Any + Send + Sync>,
    level: Level,
}
unsafe impl Send for PtrCell {}
unsafe impl Sync for PtrCell {}
impl PtrCell {
    /// Host-only heap cell with the given initial value.
    pub fn new_local(level: Level, initial: u64) -> PtrCell {
        let cell = Arc::new(AtomicU64::new(initial));
        let ptr = NonNull::from(&*cell);
        PtrCell {
            ptr,
            _keepalive: cell,
            level,
        }
    }
    /// # Safety
    /// `ptr` must be 8-aligned, valid for reads and writes of 8 bytes, accessed only
    /// atomically, and must stay valid as long as `keepalive` lives.
    pub unsafe fn from_raw(
        level: Level,
        ptr: NonNull<AtomicU64>,
        keepalive: Arc<dyn Any + Send + Sync>,
    ) -> PtrCell {
        PtrCell {
            ptr,
            _keepalive: keepalive,
            level,
        }
    }
    pub fn level(&self) -> Level {
        self.level
    }
    /// Acquire load; allowed at any time.
    pub fn load(&self) -> u64 {
        self.atomic().load(Ordering::Acquire)
    }
    fn atomic(&self) -> &AtomicU64 {
        unsafe { self.ptr.as_ref() }
    }
    #[cfg(test)]
    pub(crate) fn share(&self) -> PtrCell {
        PtrCell {
            ptr: self.ptr,
            _keepalive: Arc::clone(&self._keepalive),
            level: self.level,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MutationError {
    /// Reserved; never produced.
    NoGap,
    /// The new pointer is 0.
    NullPointer,
    /// The new value is not 8-byte aligned.
    Misaligned { value: u64 },
    /// Reserved; never produced.
    ModeViolation { mode: KernelMode },
    Access(AccessError),
}
impl fmt::Display for MutationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MutationError::NoGap => write!(f, "no open gap"),
            MutationError::NullPointer => write!(f, "null pointer is not a valid target"),
            MutationError::Misaligned { value } => {
                write!(f, "pointer value {value:#x} is not 8-byte aligned")
            }
            MutationError::ModeViolation { mode } => {
                write!(f, "operation not allowed in kernel mode {mode:?}")
            }
            MutationError::Access(e) => write!(f, "region access failed: {e}"),
        }
    }
}
impl std::error::Error for MutationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MutationError::Access(e) => Some(e),
            _ => None,
        }
    }
}
impl From<AccessError> for MutationError {
    fn from(e: AccessError) -> Self {
        MutationError::Access(e)
    }
}
/// Proof that a gap is open. Borrows the gap read-only and never closes it.
pub struct GapGuard<'g> {
    gap: &'g Gap<'g>,
    mode: KernelMode,
}
impl<'g> GapGuard<'g> {
    pub fn enter(gap: &'g Gap<'_>, mode: KernelMode) -> GapGuard<'g> {
        GapGuard { gap, mode }
    }
    pub fn mode(&self) -> KernelMode {
        self.mode
    }
    /// `(first, last)` of the gap.
    pub fn gap_range(&self) -> (u64, u64) {
        (self.gap.first(), self.gap.last())
    }
    /// Copy-on-write publish: atomically swaps the pointer to `new` (Release) and
    /// returns the previous value. The caller frees the old structure only after
    /// the subengine no longer reads it.
    pub fn swap_ptr(&self, cell: &PtrCell, new: u64) -> Result<u64, MutationError> {
        if new == 0 {
            return Err(MutationError::NullPointer);
        }
        if new % 8 != 0 {
            return Err(MutationError::Misaligned {
                value: new,
            });
        }
        Ok(cell.atomic().swap(new, Ordering::Release))
    }
    /// In-place update of small data. In persistent mode the kernel may observe a
    /// partially updated range (torn read of valid values); this is accepted.
    pub fn write_in_place(
        &self,
        region: &AccessRegion,
        offset: usize,
        src: &[u8],
    ) -> Result<(), MutationError> {
        if src.is_empty() {
            return Ok(());
        }
        region.write(offset, src).map_err(MutationError::Access)
    }
}
const _: () = {
    fn _a<T: Send + Sync + 'static>() {}
    fn _b() {
        _a::<PtrCell>();
        _a::<MutationError>();
    }
};
#[cfg(test)]
#[path = "test/mutation.rs"]
mod tests;
