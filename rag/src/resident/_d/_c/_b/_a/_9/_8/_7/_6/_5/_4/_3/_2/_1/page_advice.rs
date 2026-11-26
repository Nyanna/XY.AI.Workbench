//! Non-binding page-cache hints per byte range and blocking reads in the async runtime's blocking pool.
use std::{io, sync::{Arc, RwLock}};
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Advice {
    WillNeed,
    Sequential,
    Random,
    Cold,
}
/// Half-open byte range [start, end).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageRange {
    pub start: u64,
    pub end: u64,
}
#[derive(Debug, thiserror::Error)]
pub enum AdviceError {
    #[error("range start {start} is behind end {end}")]
    StartAfterEnd { start: u64, end: u64 },
    #[error("range end {end} exceeds mapping length {len}")]
    EndBeyondLen { end: u64, len: u64 },
    #[error("os error: {0}")]
    Os(#[from] io::Error),
}
#[derive(Debug, thiserror::Error)]
pub enum BlockingError {
    #[error("blocking task was cancelled")]
    Cancelled,
    #[error("blocking task panicked")]
    Panicked,
}
/// Replaceable hint target. Implementations must not block for I/O.
pub trait AdviceTarget: Send + Sync {
    /// Length of the target in bytes.
    fn len(&self) -> u64;
    /// Apply advice. The range is already validated and non-empty.
    fn advise(&self, advice: Advice, range: PageRange) -> Result<(), AdviceError>;
}
/// Validation only: start<=end, end<=len.
pub fn check_range(range: PageRange, len: u64) -> Result<(), AdviceError> {
    if range.start > range.end {
        return Err(AdviceError::StartAfterEnd {
            start: range.start,
            end: range.end,
        });
    }
    if range.end > len {
        return Err(AdviceError::EndBeyondLen {
            end: range.end,
            len,
        });
    }
    Ok(())
}
/// Validates the range, then forwards it to the target.
pub fn advise(
    target: &dyn AdviceTarget,
    advice: Advice,
    range: PageRange,
) -> Result<(), AdviceError> {
    check_range(range, target.len())?;
    if range.start == range.end {
        return Ok(());
    }
    target.advise(advice, range)
}
/// Equivalent to `advise(target, Advice::WillNeed, range)`.
pub fn prefetch(target: &dyn AdviceTarget, range: PageRange) -> Result<(), AdviceError> {
    advise(target, Advice::WillNeed, range)
}
/// Concrete target over a memory mapping.
pub struct MmapAdvisor {
    map: Arc<memmap2::Mmap>,
}
impl MmapAdvisor {
    pub fn new(map: Arc<memmap2::Mmap>) -> Self {
        Self { map }
    }
    #[cfg(target_os = "linux")]
    fn advise_cold(&self, range: PageRange) -> Result<(), AdviceError> {
        const MADV_COLD: libc::c_int = 20;
        let page = unsafe { libc::sysconf(libc::_SC_PAGESIZE) };
        if page <= 0 {
            return Err(io::Error::last_os_error().into());
        }
        let page = page as usize;
        let addr = (self.map.as_ptr() as usize).wrapping_add(range.start as usize);
        let aligned = addr - addr % page;
        let len = (range.end - range.start) as usize + (addr - aligned);
        let rc = unsafe { libc::madvise(aligned as *mut libc::c_void, len, MADV_COLD) };
        if rc != 0 {
            return Err(io::Error::last_os_error().into());
        }
        Ok(())
    }
    #[cfg(not(target_os = "linux"))]
    fn advise_cold(&self, _range: PageRange) -> Result<(), AdviceError> {
        Ok(())
    }
}
impl AdviceTarget for MmapAdvisor {
    fn len(&self) -> u64 {
        self.map.len() as u64
    }
    fn advise(&self, advice: Advice, range: PageRange) -> Result<(), AdviceError> {
        let native = match advice {
            Advice::WillNeed => memmap2::Advice::WillNeed,
            Advice::Sequential => memmap2::Advice::Sequential,
            Advice::Random => memmap2::Advice::Random,
            Advice::Cold => return self.advise_cold(range),
        };
        self.map
            .advise_range(
                native,
                range.start as usize,
                (range.end - range.start) as usize,
            )?;
        Ok(())
    }
}
/// Hint target bound later to the mapping of a lazily opened file.
/// While unbound it accepts every range and ignores it.
#[derive(Default)]
pub struct AdviceSlot {
    target: RwLock<Option<Arc<dyn AdviceTarget>>>,
}
impl AdviceSlot {
    pub fn set(&self, target: Arc<dyn AdviceTarget>) {
        *self.target.write().unwrap_or_else(|e| e.into_inner()) = Some(target);
    }
    pub fn clear(&self) {
        *self.target.write().unwrap_or_else(|e| e.into_inner()) = None;
    }
    pub fn is_bound(&self) -> bool {
        self.current().is_some()
    }
    fn current(&self) -> Option<Arc<dyn AdviceTarget>> {
        self.target.read().unwrap_or_else(|e| e.into_inner()).clone()
    }
}
impl AdviceTarget for AdviceSlot {
    fn len(&self) -> u64 {
        self.current().map_or(u64::MAX, |t| t.len())
    }
    fn advise(&self, advice: Advice, range: PageRange) -> Result<(), AdviceError> {
        match self.current() {
            Some(target) => target.advise(advice, range),
            None => Ok(()),
        }
    }
}
/// Read without advice: runs `f` in the blocking pool.
pub async fn read_blocking<T, F>(f: F) -> Result<T, BlockingError>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(f)
        .await
        .map_err(|e| {
            if e.is_panic() { BlockingError::Panicked } else { BlockingError::Cancelled }
        })
}
/// Advises first, then reads in the blocking pool.
/// `advice: None` means `Advice::WillNeed`; `source: None` means no hint.
pub async fn read_with_advice<T, F>(
    source: Option<Arc<dyn AdviceTarget>>,
    range: PageRange,
    advice: Option<Advice>,
    f: F,
) -> Result<T, BlockingError>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    if let Some(target) = source {
        let advice = advice.unwrap_or(Advice::WillNeed);
        if let Err(err) = advise(target.as_ref(), advice, range) {
            tracing::warn!(? advice, ? range, % err, "advice rejected");
        }
    }
    read_blocking(f).await
}
#[cfg(test)]
#[path = "test/page_advice.rs"]
mod tests;
