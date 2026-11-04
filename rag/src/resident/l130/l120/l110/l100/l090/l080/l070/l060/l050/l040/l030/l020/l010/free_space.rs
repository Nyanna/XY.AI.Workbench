use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FreeSpace {
    gaps: BTreeMap<u64, u64>,
    capacity: u64,
    free_total: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreeError {
    /// len == 0, offset+len overflows, or range exceeds capacity
    InvalidRange,
    /// range overlaps a region that is already free (double free)
    NotAllocated,
}

fn round_up(value: u64, align: u64) -> Option<u64> {
    let rem = value % align;
    if rem == 0 {
        Some(value)
    } else {
        value.checked_add(align - rem)
    }
}

impl FreeSpace {
    pub fn new(capacity: u64) -> Self {
        let mut gaps = BTreeMap::new();
        if capacity > 0 {
            gaps.insert(0, capacity);
        }
        Self {
            gaps,
            capacity,
            free_total: capacity,
        }
    }

    pub fn allocate(&mut self, len: u64, align: u64) -> Option<u64> {
        if len == 0 {
            return None;
        }
        let align = align.max(1);
        let mut found = None;
        for (&start, &gap_len) in &self.gaps {
            let end = start + gap_len;
            let Some(a) = round_up(start, align) else {
                continue;
            };
            let Some(a_end) = a.checked_add(len) else {
                continue;
            };
            if a_end <= end {
                found = Some((start, end, a, a_end));
                break;
            }
        }
        let (start, end, a, a_end) = found?;
        self.gaps.remove(&start);
        if a > start {
            self.gaps.insert(start, a - start);
        }
        if a_end < end {
            self.gaps.insert(a_end, end - a_end);
        }
        self.free_total -= len;
        Some(a)
    }

    pub fn free(&mut self, offset: u64, len: u64) -> Result<(), FreeError> {
        if len == 0 {
            return Err(FreeError::InvalidRange);
        }
        let end = offset.checked_add(len).ok_or(FreeError::InvalidRange)?;
        if end > self.capacity {
            return Err(FreeError::InvalidRange);
        }
        let prev = self.gaps.range(..=offset).next_back().map(|(&s, &l)| (s, l));
        if let Some((s, l)) = prev {
            if s + l > offset {
                return Err(FreeError::NotAllocated);
            }
        }
        let next = self.gaps.range(offset..).next().map(|(&s, &l)| (s, l));
        if let Some((s, _)) = next {
            if s < end {
                return Err(FreeError::NotAllocated);
            }
        }

        let mut new_start = offset;
        let mut new_end = end;
        if let Some((s, l)) = prev {
            if s + l == offset {
                new_start = s;
                self.gaps.remove(&s);
            }
        }
        if let Some((s, l)) = next {
            if s == end {
                new_end = s + l;
                self.gaps.remove(&s);
            }
        }
        self.gaps.insert(new_start, new_end - new_start);
        self.free_total += len;
        Ok(())
    }

    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    pub fn free_bytes(&self) -> u64 {
        self.free_total
    }
}

#[cfg(test)]
#[path = "test/free_space.rs"]
mod tests;
