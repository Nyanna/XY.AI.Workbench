//! Seqlock over word-granular atomics, for reading index data that another writer may modify
//! concurrently (shared mappings). Plain reads of such memory would be a data race.

#[cfg(loom)]
use loom::sync::atomic::{fence, AtomicU64, Ordering};
#[cfg(not(loom))]
use std::sync::atomic::{fence, AtomicU64, Ordering};

#[derive(Clone, Copy)]
pub struct Seqlock<'a> {
    seq: &'a AtomicU64,
    words: &'a [AtomicU64],
}

impl<'a> Seqlock<'a> {
    pub fn new(seq: &'a AtomicU64, words: &'a [AtomicU64]) -> Self {
        Self { seq, words }
    }

    /// # Safety
    /// `seq` and `words` must be 8-byte aligned, valid for `len` words for `'a`, and every
    /// writer of the region must follow the same protocol.
    #[cfg(not(loom))]
    pub unsafe fn from_raw(seq: *mut u64, words: *mut u64, len: usize) -> Self {
        Self {
            seq: AtomicU64::from_ptr(seq),
            words: std::slice::from_raw_parts(words as *const AtomicU64, len),
        }
    }

    /// Copies a consistent snapshot into `out`; returns the number of retries.
    pub fn read_into(&self, out: &mut Vec<u64>) -> u32 {
        let mut retries = 0;
        loop {
            let before = self.seq.load(Ordering::Acquire);
            if before & 1 == 0 {
                out.clear();
                out.extend(self.words.iter().map(|w| w.load(Ordering::Relaxed)));
                fence(Ordering::Acquire);
                if self.seq.load(Ordering::Relaxed) == before {
                    return retries;
                }
            }
            retries += 1;
            spin();
        }
    }

    /// Writers exclude each other through the odd counter value.
    pub fn write(&self, data: &[u64]) {
        assert_eq!(data.len(), self.words.len(), "seqlock write must cover the whole region");
        let start = loop {
            let s = self.seq.load(Ordering::Relaxed);
            if s & 1 == 0
                && self
                    .seq
                    .compare_exchange_weak(s, s + 1, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
            {
                break s;
            }
            spin();
        };
        fence(Ordering::Release);
        for (w, v) in self.words.iter().zip(data) {
            w.store(*v, Ordering::Relaxed);
        }
        self.seq.store(start + 2, Ordering::Release);
    }
}

#[cfg(loom)]
fn spin() {
    loom::thread::yield_now();
}

#[cfg(not(loom))]
fn spin() {
    std::hint::spin_loop();
}

#[cfg(all(test, not(loom)))]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use std::sync::Arc;

    #[test]
    fn read_returns_last_write() {
        let seq = AtomicU64::new(0);
        let words = [AtomicU64::new(0), AtomicU64::new(0)];
        let lock = Seqlock::new(&seq, &words);
        lock.write(&[3, 4]);
        let mut out = Vec::new();
        assert_eq!(lock.read_into(&mut out), 0);
        assert_eq!(out, [3, 4]);
        assert_eq!(seq.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn concurrent_reader_never_sees_a_torn_snapshot() {
        let seq = Arc::new(AtomicU64::new(0));
        let words: Arc<Vec<AtomicU64>> = Arc::new((0..8).map(|_| AtomicU64::new(0)).collect());
        let stop = Arc::new(AtomicBool::new(false));
        let writer = {
            let (seq, words, stop) = (seq.clone(), words.clone(), stop.clone());
            std::thread::spawn(move || {
                let lock = Seqlock::new(&seq, &words);
                let mut n = 0u64;
                while !stop.load(Ordering::Relaxed) {
                    n += 1;
                    lock.write(&[n; 8]);
                }
            })
        };
        let lock = Seqlock::new(&seq, &words);
        let mut out = Vec::new();
        for _ in 0..20_000 {
            lock.read_into(&mut out);
            assert!(out.iter().all(|v| *v == out[0]), "torn snapshot {out:?}");
        }
        stop.store(true, Ordering::Relaxed);
        writer.join().unwrap();
    }
}

#[cfg(loom)]
mod loom_tests {
    use super::*;
    use loom::sync::Arc;
    use loom::thread;

    #[test]
    fn reader_never_sees_a_torn_pair() {
        loom::model(|| {
            let seq = Arc::new(AtomicU64::new(0));
            let words = Arc::new([AtomicU64::new(0), AtomicU64::new(0)]);
            let writer = {
                let (seq, words) = (seq.clone(), words.clone());
                thread::spawn(move || Seqlock::new(&seq, &words[..]).write(&[1, 1]))
            };
            let mut out = Vec::new();
            Seqlock::new(&seq, &words[..]).read_into(&mut out);
            assert_eq!(out[0], out[1]);
            writer.join().unwrap();
        });
    }
}
