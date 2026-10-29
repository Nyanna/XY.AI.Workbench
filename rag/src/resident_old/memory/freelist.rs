//! Backend-agnostic offset allocator: first-fit over a coalescing free map.

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct FreeList {
    capacity: u64,
    free: BTreeMap<u64, u64>,
}

impl FreeList {
    pub fn new(capacity: u64) -> Self {
        let mut free = BTreeMap::new();
        if capacity > 0 {
            free.insert(0, capacity);
        }
        Self { capacity, free }
    }

    pub fn capacity(&self) -> u64 {
        self.capacity
    }

    pub fn available(&self) -> u64 {
        self.free.values().sum()
    }

    pub fn alloc(&mut self, len: u64, align: u64) -> Option<u64> {
        let align = align.max(1);
        if len == 0 {
            return None;
        }
        let (start, blk_len, off) = self.free.iter().find_map(|(&s, &l)| {
            let off = s.checked_add(align - 1)? / align * align;
            let end = off.checked_add(len)?;
            (end <= s + l).then_some((s, l, off))
        })?;
        self.free.remove(&start);
        if off > start {
            self.free.insert(start, off - start);
        }
        let end = off + len;
        if end < start + blk_len {
            self.free.insert(end, start + blk_len - end);
        }
        Some(off)
    }

    pub fn free(&mut self, offset: u64, len: u64) {
        let mut start = offset;
        let mut size = len;
        if let Some((&ps, &pl)) = self.free.range(..offset).next_back() {
            if ps + pl == offset {
                self.free.remove(&ps);
                start = ps;
                size += pl;
            }
        }
        if let Some(&nl) = self.free.get(&(offset + len)) {
            self.free.remove(&(offset + len));
            size += nl;
        }
        self.free.insert(start, size);
    }
}

#[cfg(test)]
#[path = "test/freelist.rs"]
mod tests;
