//! Page-cache hints for mmap-backed index data. Hints only; the kernel may ignore them.

use std::io;
use std::ops::Range;

use memmap2::{Advice, Mmap};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hint {
    WillNeed,
    Sequential,
    Random,
    /// Deprioritize pages for reclaim without discarding them.
    Cold,
}

pub trait AdviceSink: Send + Sync {
    fn advise(&self, range: Range<usize>, hint: Hint) -> io::Result<()>;
}

pub struct MmapAdvice {
    map: Mmap,
}

impl MmapAdvice {
    pub fn new(map: Mmap) -> Self {
        Self { map }
    }
}

impl AdviceSink for MmapAdvice {
    fn advise(&self, range: Range<usize>, hint: Hint) -> io::Result<()> {
        if range.start > range.end || range.end > self.map.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "advice range out of bounds"));
        }
        let len = range.end - range.start;
        if len == 0 {
            return Ok(());
        }
        match hint {
            Hint::WillNeed => self.map.advise_range(Advice::WillNeed, range.start, len),
            Hint::Sequential => self.map.advise_range(Advice::Sequential, range.start, len),
            Hint::Random => self.map.advise_range(Advice::Random, range.start, len),
            Hint::Cold => advise_cold(&self.map, range.start, len),
        }
    }
}

#[cfg(target_os = "linux")]
fn advise_cold(map: &Mmap, offset: usize, len: usize) -> io::Result<()> {
    // SAFETY: sysconf has no preconditions.
    let page = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as usize;
    let addr = map.as_ptr() as usize + offset;
    let start = addr & !(page - 1);
    let len = len + (addr - start);
    // SAFETY: the range lies inside a live mapping; MADV_COLD never changes contents.
    let rc = unsafe { libc::madvise(start as *mut libc::c_void, len, libc::MADV_COLD) };
    if rc == 0 { Ok(()) } else { Err(io::Error::last_os_error()) }
}

#[cfg(not(target_os = "linux"))]
fn advise_cold(_map: &Mmap, _offset: usize, _len: usize) -> io::Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use memmap2::MmapMut;

    fn anon(len: usize) -> MmapAdvice {
        MmapAdvice::new(MmapMut::map_anon(len).unwrap().make_read_only().unwrap())
    }

    #[test]
    fn all_hints_accepted_on_valid_range() {
        let a = anon(3 * 4096);
        for hint in [Hint::WillNeed, Hint::Sequential, Hint::Random, Hint::Cold] {
            a.advise(100..9000, hint).unwrap();
        }
    }

    #[test]
    fn out_of_bounds_and_empty_ranges() {
        let a = anon(4096);
        assert!(a.advise(0..4097, Hint::WillNeed).is_err());
        a.advise(10..10, Hint::Cold).unwrap();
    }
}
