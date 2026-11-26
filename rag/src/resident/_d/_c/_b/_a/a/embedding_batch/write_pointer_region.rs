use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::RegionHandle;
pub const INVALID_OFFSET: u64 = u64::MAX;
pub const NO_OVERFLOW: u64 = u64::MAX;
pub const WARP_SIZE: u32 = 32;
pub const STATE_SIZE: usize = 32;
/// Shared host/device state; the device source declares the identical layout.
#[repr(C)]
pub struct WriteState {
    ptr: AtomicU64,
    overflow_min: AtomicU64,
    capacity: u64,
    overflow_flag: AtomicU32,
    logged: AtomicU32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StateSnapshot {
    pub ptr: u64,
    pub overflow_min: u64,
    pub capacity: u64,
    pub overflow: bool,
}
impl StateSnapshot {
    /// `min(overflow_min, ptr, capacity)`
    pub fn valid_len(&self) -> u64 {
        self.overflow_min.min(self.ptr).min(self.capacity)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Claim3 {
    Ok { offset: u64 },
    Invalid,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClaimError3 {
    LengthMismatch { lens: usize, out: usize },
}
impl std::fmt::Display for ClaimError3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ClaimError3::LengthMismatch { lens, out } => {
                write!(f, "group claim length mismatch: {lens} lengths, {out} outputs")
            }
        }
    }
}
impl std::error::Error for ClaimError3 {}
fn le_u64(image: &[u8; STATE_SIZE], at: usize) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&image[at..at + 8]);
    u64::from_le_bytes(bytes)
}
impl WriteState {
    pub fn new(capacity: u64) -> WriteState {
        WriteState {
            ptr: AtomicU64::new(0),
            overflow_min: AtomicU64::new(NO_OVERFLOW),
            capacity,
            overflow_flag: AtomicU32::new(0),
            logged: AtomicU32::new(0),
        }
    }
    /// Little-endian image of a fresh state, for initialising device memory.
    pub fn init_image(capacity: u64) -> [u8; STATE_SIZE] {
        let mut image = [0u8; STATE_SIZE];
        image[8..16].copy_from_slice(&NO_OVERFLOW.to_le_bytes());
        image[16..24].copy_from_slice(&capacity.to_le_bytes());
        image
    }
    /// Reads an image copied back from the device.
    pub fn snapshot_from_image(image: &[u8; STATE_SIZE]) -> StateSnapshot {
        let mut flag = [0u8; 4];
        flag.copy_from_slice(&image[24..28]);
        StateSnapshot {
            ptr: le_u64(image, 0),
            overflow_min: le_u64(image, 8),
            capacity: le_u64(image, 16),
            overflow: u32::from_le_bytes(flag) != 0,
        }
    }
    pub fn claim(&self, len: u64) -> Claim3 {
        let old = self.ptr.fetch_add(len, Ordering::AcqRel);
        match old.checked_add(len) {
            Some(end) if end <= self.capacity => Claim3::Ok { offset: old },
            _ => {
                self.record_overflow(old, len);
                Claim3::Invalid
            }
        }
    }
    /// One pointer update for the whole group; a failed member fails all later ones.
    pub fn claim_group(&self, lens: &[u64], out: &mut [u64]) -> Result<(), ClaimError3> {
        if lens.len() != out.len() {
            return Err(ClaimError3::LengthMismatch {
                lens: lens.len(),
                out: out.len(),
            });
        }
        if lens.is_empty() {
            return Ok(());
        }
        let total = lens.iter().fold(0u64, |sum, &len| sum.saturating_add(len));
        let mut pos = self.ptr.fetch_add(total, Ordering::AcqRel);
        let mut failed = false;
        for (&len, slot) in lens.iter().zip(out.iter_mut()) {
            if !failed {
                match pos.checked_add(len) {
                    Some(end) if end <= self.capacity => {
                        *slot = pos;
                        pos = end;
                        continue;
                    }
                    _ => {
                        failed = true;
                        self.record_overflow(pos, len);
                    }
                }
            }
            *slot = INVALID_OFFSET;
        }
        Ok(())
    }
    pub fn snapshot(&self) -> StateSnapshot {
        StateSnapshot {
            ptr: self.ptr.load(Ordering::Acquire),
            overflow_min: self.overflow_min.load(Ordering::Acquire),
            capacity: self.capacity,
            overflow: self.overflow_flag.load(Ordering::Acquire) != 0,
        }
    }
    pub fn reset(&mut self) {
        *self.ptr.get_mut() = 0;
        *self.overflow_min.get_mut() = NO_OVERFLOW;
        *self.overflow_flag.get_mut() = 0;
        *self.logged.get_mut() = 0;
    }
    fn record_overflow(&self, boundary: u64, claim_len: u64) {
        self.overflow_flag.store(1, Ordering::Release);
        self.overflow_min.fetch_min(boundary, Ordering::AcqRel);
        self.log_once(boundary, Some(claim_len));
    }
    /// Logs only for the caller that flips `logged` from 0 to 1.
    fn log_once(&self, boundary: u64, claim_len: Option<u64>) -> bool {
        if self
            .logged
            .compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire)
            .is_err()
        {
            return false;
        }
        match claim_len {
            Some(len) => {
                log::warn!(
                    "write pointer overflow: capacity {}, boundary {}, claim length {}",
                    self.capacity, boundary, len
                )
            }
            None => {
                log::warn!(
                    "write pointer overflow: capacity {}, boundary {}, claim length unknown",
                    self.capacity, boundary
                )
            }
        }
        true
    }
}
/// CUDA C++ source for producer kernels; define it once per translation unit.
pub const WRITE_POINTER_DEVICE_SRC: &str = r#"
#ifndef XY_WRITE_POINTER_DEVICE_H
#define XY_WRITE_POINTER_DEVICE_H

#define WP_INVALID_OFFSET 0xffffffffffffffffull

struct WriteState {
  unsigned long long ptr;
  unsigned long long overflow_min;
  unsigned long long capacity;
  unsigned int overflow_flag;
  unsigned int logged;
};

static_assert(sizeof(WriteState) == 32, "WriteState layout");

__device__ __forceinline__ unsigned long long wp_sat_add(unsigned long long a, unsigned long long b) {
  const unsigned long long s = a + b;
  return s < a ? 0xffffffffffffffffull : s;
}

__device__ __forceinline__ bool wp_fits(const WriteState* s, unsigned long long pos, unsigned long long len) {
  const unsigned long long end = pos + len;
  return end >= pos && end <= s->capacity;
}

__device__ __forceinline__ void wp_fail(WriteState* s, unsigned long long pos) {
  atomicMin(&s->overflow_min, pos);
  atomicExch(&s->overflow_flag, 1u);
  atomicCAS(&s->logged, 0u, 1u);
}

__device__ __forceinline__ unsigned long long wp_claim(WriteState* s, unsigned long long len) {
  const unsigned long long old = atomicAdd(&s->ptr, len);
  if (wp_fits(s, old, len)) return old;
  wp_fail(s, old);
  return WP_INVALID_OFFSET;
}

__device__ __forceinline__ unsigned long long wp_claim_warp(WriteState* s, unsigned long long len) {
  const unsigned int mask = __activemask();
  unsigned int lane;
  asm volatile("mov.u32 %0, %%laneid;" : "=r"(lane));
  const unsigned int last = 31u - (unsigned int)__clz((int)mask);
  unsigned long long prefix = 0ull;
  unsigned long long total = 0ull;
  if ((mask & (mask + 1u)) == 0u) {
    unsigned long long inclusive = len;
    for (unsigned int d = 1u; d < 32u; d <<= 1) {
      const unsigned long long v = __shfl_up_sync(mask, inclusive, d);
      if (lane >= d) inclusive = wp_sat_add(inclusive, v);
    }
    const unsigned long long before = __shfl_up_sync(mask, inclusive, 1);
    prefix = (lane == 0u) ? 0ull : before;
    total = __shfl_sync(mask, inclusive, last);
  } else {
    unsigned int rest = mask;
    while (rest != 0u) {
      const unsigned int src = (unsigned int)__ffs((int)rest) - 1u;
      const unsigned long long v = __shfl_sync(mask, len, src);
      if (src < lane) prefix = wp_sat_add(prefix, v);
      total = wp_sat_add(total, v);
      rest &= rest - 1u;
    }
  }
  unsigned long long base = 0ull;
  if (lane == last) base = atomicAdd(&s->ptr, total);
  base = __shfl_sync(mask, base, last);
  const unsigned long long pos = base + prefix;
  const bool ok = pos >= base && wp_fits(s, pos, len);
  const unsigned int failed = __ballot_sync(mask, !ok);
  if (failed != 0u && lane == (unsigned int)__ffs((int)failed) - 1u) wp_fail(s, pos);
  return ok ? pos : WP_INVALID_OFFSET;
}

#endif
"#;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WriteResult {
    /// No overflow; `len` bytes are valid.
    Complete { region: RegionHandle, len: u64 },
    /// Overflow; `[0, valid_len)` is valid and the remainder belongs in the next region.
    Partial { region: RegionHandle, valid_len: u64, overflow_pos: u64 },
}
impl WriteResult {
    pub fn from_snapshot(region: RegionHandle, s: &StateSnapshot) -> WriteResult {
        let valid_len = s.valid_len();
        if s.overflow {
            WriteResult::Partial {
                region,
                valid_len,
                overflow_pos: s.overflow_min,
            }
        } else {
            WriteResult::Complete {
                region,
                len: valid_len,
            }
        }
    }
    pub fn valid_len(&self) -> u64 {
        match self {
            WriteResult::Complete { len, .. } => *len,
            WriteResult::Partial { valid_len, .. } => *valid_len,
        }
    }
}
pub struct WritePointerRegion {
    region: RegionHandle,
    state: WriteState,
}
impl WritePointerRegion {
    pub fn new(region: RegionHandle) -> WritePointerRegion {
        WritePointerRegion {
            region,
            state: WriteState::new(region.len()),
        }
    }
    pub fn region(&self) -> RegionHandle {
        self.region
    }
    pub fn state(&self) -> &WriteState {
        &self.state
    }
    pub fn result(&self) -> WriteResult {
        let snapshot = self.state.snapshot();
        if snapshot.overflow {
            self.state.log_once(snapshot.overflow_min, None);
        }
        WriteResult::from_snapshot(self.region, &snapshot)
    }
}
#[cfg(test)]
#[path = "test/write_pointer_region.rs"]
mod tests;
