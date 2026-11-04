use std::sync::{Mutex, MutexGuard};
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct StreamId(pub u32);
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Fence(pub u64);
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Lifetime {
    Persistent,
    Step,
    Dynamic,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Region {
    pub lifetime: Lifetime,
    pub offset: u64,
    pub len: u64,
}
#[derive(Clone, Debug)]
pub struct Config {
    pub persistent_bytes: u64,
    pub step_bytes: u64,
    pub dynamic_bytes: u64,
    pub max_streams: u32,
    pub max_dynamic_blocks: usize,
    pub default_align: u64,
}
#[derive(Debug, PartialEq, Eq)]
pub enum MemError {
    InvalidConfig,
    ZeroSize,
    BadAlign,
    OutOfMemory { lifetime: Lifetime, requested: u64 },
    NotReady,
    UnknownStream,
    InvalidRegion,
    WrongLifetime,
    BlockTableFull,
}
type Tag = Option<(StreamId, Fence)>;
#[derive(Clone, Copy, Debug)]
enum BlockState {
    Live { len: u64 },
    Free(Tag),
}
#[derive(Clone, Copy, Debug)]
struct Block {
    offset: u64,
    size: u64,
    state: BlockState,
}
struct Inner {
    cfg: Config,
    completed: Vec<Option<Fence>>,
    persistent_off: u64,
    step_off: u64,
    step_barrier: Vec<Option<Fence>>,
    blocks: Vec<Block>,
    dynamic_used: u64,
}
pub struct MemoryManager {
    inner: Mutex<Inner>,
}
fn round_up(value: u64, align: u64) -> Option<u64> {
    let mask = align - 1;
    value.checked_add(mask).map(|v| v & !mask)
}
fn merge_blocks(a: &Block, b: &Block) -> Option<Block> {
    let tag = match (a.state, b.state) {
        (BlockState::Free(None), BlockState::Free(None)) => None,
        (
            BlockState::Free(Some((s1, f1))),
            BlockState::Free(Some((s2, f2))),
        ) if s1 == s2 => Some((s1, f1.max(f2))),
        _ => return None,
    };
    Some(Block {
        offset: a.offset,
        size: a.size + b.size,
        state: BlockState::Free(tag),
    })
}
impl Inner {
    fn check_stream(&self, s: StreamId) -> Result<usize, MemError> {
        if s.0 < self.cfg.max_streams {
            Ok(s.0 as usize)
        } else {
            Err(MemError::UnknownStream)
        }
    }
    fn is_done(&self, s: StreamId, f: Fence) -> bool {
        self.completed[s.0 as usize] >= Some(f)
    }
    fn coalesce(&mut self) {
        let completed = &self.completed;
        let blocks = &mut self.blocks;
        for b in blocks.iter_mut() {
            if let BlockState::Free(Some((s, f))) = b.state {
                if completed[s.0 as usize] >= Some(f) {
                    b.state = BlockState::Free(None);
                }
            }
        }
        let mut w = 0;
        for r in 0..blocks.len() {
            if w > 0 {
                if let Some(m) = merge_blocks(&blocks[w - 1], &blocks[r]) {
                    blocks[w - 1] = m;
                    continue;
                }
            }
            blocks[w] = blocks[r];
            w += 1;
        }
        blocks.truncate(w);
    }
    fn alloc_bump(
        &mut self,
        l: Lifetime,
        size: u64,
        align: u64,
        stream: StreamId,
    ) -> Result<Region, MemError> {
        let (cap, cur) = match l {
            Lifetime::Persistent => (self.cfg.persistent_bytes, self.persistent_off),
            _ => (self.cfg.step_bytes, self.step_off),
        };
        let oom = MemError::OutOfMemory {
            lifetime: l,
            requested: size,
        };
        let start = round_up(cur, align).ok_or(oom)?;
        let end = match start.checked_add(size) {
            Some(e) if e <= cap => e,
            _ => {
                return Err(MemError::OutOfMemory {
                    lifetime: l,
                    requested: size,
                });
            }
        };
        if l == Lifetime::Step {
            let q = stream.0 as usize;
            for (i, b) in self.step_barrier.iter().enumerate() {
                if i != q && *b > self.completed[i] {
                    return Err(MemError::NotReady);
                }
            }
            self.step_off = end;
        } else {
            self.persistent_off = end;
        }
        Ok(Region {
            lifetime: l,
            offset: start,
            len: size,
        })
    }
    fn alloc_dynamic(
        &mut self,
        size: u64,
        align: u64,
        stream: StreamId,
    ) -> Result<Region, MemError> {
        let l = Lifetime::Dynamic;
        let oom = MemError::OutOfMemory {
            lifetime: l,
            requested: size,
        };
        let rounded = round_up(size, align).ok_or(oom)?;
        let mut best: Option<(usize, u64, u64)> = None;
        let mut blocked = false;
        let mut free_total = 0u64;
        for (i, b) in self.blocks.iter().enumerate() {
            let tag = match b.state {
                BlockState::Free(t) => t,
                BlockState::Live { .. } => continue,
            };
            free_total = free_total.saturating_add(b.size);
            let Some(start) = round_up(b.offset, align) else { continue };
            let Some(need) = (start - b.offset).checked_add(rounded) else { continue };
            if need > b.size {
                continue;
            }
            let reusable = match tag {
                None => true,
                Some((s, f)) => s == stream || self.is_done(s, f),
            };
            if reusable {
                if best.map_or(true, |x| b.size < x.2) {
                    best = Some((i, start, b.size));
                }
            } else {
                blocked = true;
            }
        }
        let Some((i, start, _)) = best else {
            return Err(
                if blocked || free_total >= rounded {
                    MemError::NotReady
                } else if self.cfg.max_dynamic_blocks == 0 {
                    MemError::BlockTableFull
                } else {
                    MemError::OutOfMemory {
                        lifetime: l,
                        requested: size,
                    }
                },
            );
        };
        let b = self.blocks[i];
        let tag = match b.state {
            BlockState::Free(t) => t,
            BlockState::Live { .. } => None,
        };
        let pad = start - b.offset;
        let tail = b.size - pad - rounded;
        let extra = (pad > 0) as usize + (tail > 0) as usize;
        if self.blocks.len() + extra > self.cfg.max_dynamic_blocks {
            return Err(MemError::BlockTableFull);
        }
        if tail > 0 {
            self.blocks
                .insert(
                    i + 1,
                    Block {
                        offset: start + rounded,
                        size: tail,
                        state: BlockState::Free(tag),
                    },
                );
        }
        self.blocks[i] = Block {
            offset: start,
            size: rounded,
            state: BlockState::Live { len: size },
        };
        if pad > 0 {
            self.blocks
                .insert(
                    i,
                    Block {
                        offset: b.offset,
                        size: pad,
                        state: BlockState::Free(tag),
                    },
                );
        }
        self.dynamic_used += rounded;
        Ok(Region {
            lifetime: l,
            offset: start,
            len: size,
        })
    }
}
impl MemoryManager {
    pub fn new(cfg: Config) -> Result<MemoryManager, MemError> {
        if cfg.max_streams == 0 || !cfg.default_align.is_power_of_two()
            || (cfg.dynamic_bytes == 0 && cfg.max_dynamic_blocks > 0)
        {
            return Err(MemError::InvalidConfig);
        }
        let n = cfg.max_streams as usize;
        let mut blocks = Vec::with_capacity(cfg.max_dynamic_blocks);
        if cfg.dynamic_bytes > 0 && cfg.max_dynamic_blocks > 0 {
            blocks
                .push(Block {
                    offset: 0,
                    size: cfg.dynamic_bytes,
                    state: BlockState::Free(None),
                });
        }
        Ok(MemoryManager {
            inner: Mutex::new(Inner {
                completed: vec![None; n],
                step_barrier: vec![None; n],
                persistent_off: 0,
                step_off: 0,
                blocks,
                dynamic_used: 0,
                cfg,
            }),
        })
    }
    fn lock(&self) -> MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }
    pub fn capacity(&self, l: Lifetime) -> u64 {
        let g = self.lock();
        match l {
            Lifetime::Persistent => g.cfg.persistent_bytes,
            Lifetime::Step => g.cfg.step_bytes,
            Lifetime::Dynamic => g.cfg.dynamic_bytes,
        }
    }
    pub fn used(&self, l: Lifetime) -> u64 {
        let g = self.lock();
        match l {
            Lifetime::Persistent => g.persistent_off,
            Lifetime::Step => g.step_off,
            Lifetime::Dynamic => g.dynamic_used,
        }
    }
    pub fn alloc(
        &self,
        l: Lifetime,
        size: u64,
        align: Option<u64>,
        stream: StreamId,
    ) -> Result<Region, MemError> {
        let mut g = self.lock();
        if size == 0 {
            return Err(MemError::ZeroSize);
        }
        let align = align.unwrap_or(g.cfg.default_align);
        if !align.is_power_of_two() {
            return Err(MemError::BadAlign);
        }
        g.check_stream(stream)?;
        match l {
            Lifetime::Dynamic => g.alloc_dynamic(size, align, stream),
            _ => g.alloc_bump(l, size, align, stream),
        }
    }
    pub fn free(
        &self,
        r: Region,
        stream: StreamId,
        fence: Fence,
    ) -> Result<(), MemError> {
        let mut g = self.lock();
        g.check_stream(stream)?;
        if r.lifetime != Lifetime::Dynamic {
            return Err(MemError::WrongLifetime);
        }
        let idx = g
            .blocks
            .binary_search_by_key(&r.offset, |b| b.offset)
            .map_err(|_| MemError::InvalidRegion)?;
        let b = g.blocks[idx];
        match b.state {
            BlockState::Live { len } if len == r.len => {}
            _ => return Err(MemError::InvalidRegion),
        }
        g.blocks[idx].state = BlockState::Free(Some((stream, fence)));
        g.dynamic_used -= b.size;
        g.coalesce();
        Ok(())
    }
    pub fn reset_step(&self, stream: StreamId, fence: Fence) -> Result<(), MemError> {
        let mut g = self.lock();
        let q = g.check_stream(stream)?;
        g.step_off = 0;
        let cur = g.step_barrier[q];
        g.step_barrier[q] = cur.max(Some(fence));
        Ok(())
    }
    pub fn complete(&self, stream: StreamId, fence: Fence) -> Result<(), MemError> {
        let mut g = self.lock();
        let q = g.check_stream(stream)?;
        let cur = g.completed[q];
        g.completed[q] = cur.max(Some(fence));
        g.coalesce();
        Ok(())
    }
}
#[cfg(test)]
#[path = "test/memory_management.rs"]
mod tests;
