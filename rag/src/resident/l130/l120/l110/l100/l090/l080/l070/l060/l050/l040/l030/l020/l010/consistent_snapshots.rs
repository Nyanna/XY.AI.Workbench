//! Seqlock over a region of machine words.

use std::sync::atomic::{fence, AtomicU64, AtomicUsize, Ordering};

/// Copyable, non-owning view of one seqlock-protected region.
#[derive(Clone, Copy)]
pub struct SeqRegion {
    seq: *const AtomicU64,
    data: *const AtomicUsize,
    len: usize,
}

unsafe impl Send for SeqRegion {}
unsafe impl Sync for SeqRegion {}

impl SeqRegion {
    /// # Safety
    /// - `seq` is aligned for `AtomicU64`; `data` is aligned for `AtomicUsize`.
    /// - `seq` and `data[0..len]` are valid for reads and writes for the whole lifetime of every
    ///   copy of this view, and do not overlap.
    /// - Every access to this memory, in any thread or process, goes only through this protocol
    ///   (`SeqRegion` or `SeqBuf`). No plain non-atomic access.
    /// - `*seq` is even when first used (normally 0).
    pub unsafe fn from_raw_parts(
        seq: *const AtomicU64,
        data: *const AtomicUsize,
        len: usize,
    ) -> SeqRegion {
        SeqRegion { seq, data, len }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    fn seq_ref(&self) -> &AtomicU64 {
        // SAFETY: validity guaranteed by `from_raw_parts` contract.
        unsafe { &*self.seq }
    }

    #[inline]
    fn words(&self) -> &[AtomicUsize] {
        // SAFETY: validity guaranteed by `from_raw_parts` contract; an empty slice may use
        // any non-null aligned pointer.
        unsafe { std::slice::from_raw_parts(self.data, self.len) }
    }

    /// Copies the entire region into `out` and returns the retry count.
    pub fn read_into(&self, out: &mut [usize]) -> u64 {
        assert_eq!(
            out.len(),
            self.len,
            "SeqRegion::read_into: output length {} does not match region length {}",
            out.len(),
            self.len
        );
        let seq = self.seq_ref();
        let words = self.words();
        let mut retries: u64 = 0;
        loop {
            let s1 = seq.load(Ordering::Acquire);
            if s1 & 1 == 1 {
                retries += 1;
                std::hint::spin_loop();
                continue;
            }
            for (o, w) in out.iter_mut().zip(words.iter()) {
                *o = w.load(Ordering::Relaxed);
            }
            fence(Ordering::Acquire);
            let s2 = seq.load(Ordering::Relaxed);
            if s2 != s1 {
                retries += 1;
                std::hint::spin_loop();
                continue;
            }
            return retries;
        }
    }

    pub fn read(&self) -> (Vec<usize>, u64) {
        let mut v = vec![0usize; self.len];
        let retries = self.read_into(&mut v);
        (v, retries)
    }

    /// Writes the entire region from `src`. The counter increases by 2.
    pub fn write(&self, src: &[usize]) {
        assert_eq!(
            src.len(),
            self.len,
            "SeqRegion::write: source length {} does not match region length {}",
            src.len(),
            self.len
        );
        let seq = self.seq_ref();
        let mut s = seq.load(Ordering::Relaxed);
        loop {
            if s & 1 == 1 {
                std::hint::spin_loop();
                s = seq.load(Ordering::Relaxed);
                continue;
            }
            match seq.compare_exchange(s, s.wrapping_add(1), Ordering::Acquire, Ordering::Relaxed)
            {
                Ok(_) => break,
                Err(cur) => {
                    std::hint::spin_loop();
                    s = cur;
                }
            }
        }
        fence(Ordering::Release);
        for (w, v) in self.words().iter().zip(src.iter()) {
            w.store(*v, Ordering::Relaxed);
        }
        seq.store(s.wrapping_add(2), Ordering::Release);
    }

    /// Current counter value (Acquire load). Even means no writer is active.
    pub fn sequence(&self) -> u64 {
        self.seq_ref().load(Ordering::Acquire)
    }
}

/// Owning, heap-backed region (counter plus words, initially 0).
pub struct SeqBuf {
    seq: Box<AtomicU64>,
    data: Box<[AtomicUsize]>,
}

unsafe impl Send for SeqBuf {}
unsafe impl Sync for SeqBuf {}

impl SeqBuf {
    pub fn new(len: usize) -> SeqBuf {
        let data: Vec<AtomicUsize> = (0..len).map(|_| AtomicUsize::new(0)).collect();
        SeqBuf {
            seq: Box::new(AtomicU64::new(0)),
            data: data.into_boxed_slice(),
        }
    }

    pub fn from_words(init: &[usize]) -> SeqBuf {
        let data: Vec<AtomicUsize> = init.iter().map(|&v| AtomicUsize::new(v)).collect();
        SeqBuf {
            seq: Box::new(AtomicU64::new(0)),
            data: data.into_boxed_slice(),
        }
    }

    /// View valid as long as the `SeqBuf` lives.
    pub fn region(&self) -> SeqRegion {
        SeqRegion {
            seq: &*self.seq as *const AtomicU64,
            data: self.data.as_ptr(),
            len: self.data.len(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn read_into(&self, out: &mut [usize]) -> u64 {
        self.region().read_into(out)
    }

    pub fn read(&self) -> (Vec<usize>, u64) {
        self.region().read()
    }

    pub fn write(&self, src: &[usize]) {
        self.region().write(src)
    }

    pub fn sequence(&self) -> u64 {
        self.region().sequence()
    }
}

#[cfg(test)]
#[path = "test/consistent_snapshots.rs"]
mod tests;
