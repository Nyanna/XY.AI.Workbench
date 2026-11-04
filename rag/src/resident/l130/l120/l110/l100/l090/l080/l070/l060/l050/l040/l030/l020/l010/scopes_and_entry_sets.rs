use std::borrow::Cow;
use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};
use std::ops::{Range, RangeInclusive};

pub type EntryId = u64;

const CHUNK_BITS: u32 = 16;
const WORDS_PER_CHUNK: usize = 1024;
const CHUNK_BYTES: u128 = (WORDS_PER_CHUNK * 8) as u128;
const MIN_RUNS_FOR_BITMAP: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ScopeKind {
    All,
    Range,
    Set,
    Subtree,
    Prefix,
    Centroid,
}

impl ScopeKind {
    pub const fn name(self) -> &'static str {
        match self {
            ScopeKind::All => "all",
            ScopeKind::Range => "range",
            ScopeKind::Set => "set",
            ScopeKind::Subtree => "subtree",
            ScopeKind::Prefix => "prefix",
            ScopeKind::Centroid => "centroid",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Scope {
    All,
    /// Half-open ID range `[start, end)`; empty or reversed matches nothing.
    Range { start: EntryId, end: EntryId },
    Set(EntrySet),
    Subtree { root: u64 },
    Prefix { bytes: Vec<u8> },
    Centroid { count: u32, radius: Option<f32> },
}

impl Scope {
    pub fn kind(&self) -> ScopeKind {
        match self {
            Scope::All => ScopeKind::All,
            Scope::Range { .. } => ScopeKind::Range,
            Scope::Set(_) => ScopeKind::Set,
            Scope::Subtree { .. } => ScopeKind::Subtree,
            Scope::Prefix { .. } => ScopeKind::Prefix,
            Scope::Centroid { .. } => ScopeKind::Centroid,
        }
    }

    pub fn name(&self) -> &'static str {
        self.kind().name()
    }
}

type Run = (u64, u64);

#[derive(Clone, Debug)]
enum Repr {
    /// Sorted, non-overlapping, non-adjacent inclusive runs.
    Runs(Vec<Run>),
    /// Sparse chunked bitmap keyed by `id >> CHUNK_BITS`; no empty chunks.
    Bitmap(BTreeMap<u64, Box<[u64; WORDS_PER_CHUNK]>>),
}

/// Immutable compact set of `u64` IDs.
#[derive(Clone, Debug)]
pub struct EntrySet {
    repr: Repr,
    len: u128,
}

fn coalesce(sorted: Vec<Run>) -> Vec<Run> {
    let mut out: Vec<Run> = Vec::with_capacity(sorted.len());
    for (s, e) in sorted {
        match out.last_mut() {
            Some(last) if last.1 == u64::MAX || s <= last.1 + 1 => {
                if e > last.1 {
                    last.1 = e;
                }
            }
            _ => out.push((s, e)),
        }
    }
    out
}

fn set_bits(words: &mut [u64; WORDS_PER_CHUNK], lo: usize, hi: usize) {
    for wi in (lo / 64)..=(hi / 64) {
        let from = if wi == lo / 64 { lo % 64 } else { 0 };
        let to = if wi == hi / 64 { hi % 64 } else { 63 };
        let n = (to - from + 1) as u32;
        words[wi] |= (u64::MAX >> (64 - n)) << from;
    }
}

fn bitmap_worthwhile(runs: &[Run]) -> bool {
    if runs.len() < MIN_RUNS_FOR_BITMAP {
        return false;
    }
    let budget = runs.len() as u128 * 16;
    let mut bytes: u128 = 0;
    let mut last_chunk: Option<u64> = None;
    for &(s, e) in runs {
        let lo = s >> CHUNK_BITS;
        let hi = e >> CHUNK_BITS;
        let mut n = (hi - lo) as u128 + 1;
        if last_chunk == Some(lo) {
            n -= 1;
        }
        bytes += n * CHUNK_BYTES;
        if bytes >= budget {
            return false;
        }
        last_chunk = Some(hi);
    }
    true
}

fn build_bitmap(runs: &[Run]) -> BTreeMap<u64, Box<[u64; WORDS_PER_CHUNK]>> {
    let mut map: BTreeMap<u64, Box<[u64; WORDS_PER_CHUNK]>> = BTreeMap::new();
    let mask = (1u64 << CHUNK_BITS) - 1;
    for &(s, e) in runs {
        for c in (s >> CHUNK_BITS)..=(e >> CHUNK_BITS) {
            let lo = if c == s >> CHUNK_BITS { (s & mask) as usize } else { 0 };
            let hi = if c == e >> CHUNK_BITS { (e & mask) as usize } else { mask as usize };
            let words = map.entry(c).or_insert_with(|| Box::new([0u64; WORDS_PER_CHUNK]));
            set_bits(words, lo, hi);
        }
    }
    map
}

fn bitmap_runs(map: &BTreeMap<u64, Box<[u64; WORDS_PER_CHUNK]>>) -> Vec<Run> {
    let mut out: Vec<Run> = Vec::new();
    let mut push = |s: u64, e: u64| match out.last_mut() {
        Some(last) if last.1 != u64::MAX && last.1 + 1 == s => last.1 = e,
        _ => out.push((s, e)),
    };
    for (&c, words) in map {
        for (wi, &w) in words.iter().enumerate() {
            let base = (c << CHUNK_BITS) + (wi as u64) * 64;
            let mut pos = 0u32;
            while pos < 64 {
                let rest = w >> pos;
                if rest == 0 {
                    break;
                }
                let s = pos + rest.trailing_zeros();
                let len = (!(w >> s)).trailing_zeros();
                push(base + s as u64, base + (s + len - 1) as u64);
                pos = s + len;
            }
        }
    }
    out
}

struct BitIter<'a> {
    chunks: std::collections::btree_map::Iter<'a, u64, Box<[u64; WORDS_PER_CHUNK]>>,
    chunk: u64,
    words: Option<&'a [u64; WORDS_PER_CHUNK]>,
    word: usize,
    bits: u64,
}

impl Iterator for BitIter<'_> {
    type Item = EntryId;

    fn next(&mut self) -> Option<EntryId> {
        loop {
            if self.bits != 0 {
                let tz = self.bits.trailing_zeros() as u64;
                self.bits &= self.bits - 1;
                return Some((self.chunk << CHUNK_BITS) | (self.word as u64 * 64 + tz));
            }
            if let Some(words) = self.words {
                self.word += 1;
                if self.word < WORDS_PER_CHUNK {
                    self.bits = words[self.word];
                    continue;
                }
            }
            let (k, w) = self.chunks.next()?;
            self.chunk = *k;
            self.words = Some(&**w);
            self.word = 0;
            self.bits = w[0];
        }
    }
}

enum IterInner<'a> {
    Runs {
        runs: std::slice::Iter<'a, Run>,
        cur: Option<Run>,
    },
    Bitmap(BitIter<'a>),
}

impl Iterator for IterInner<'_> {
    type Item = EntryId;

    fn next(&mut self) -> Option<EntryId> {
        match self {
            IterInner::Runs { runs, cur } => loop {
                if let Some((n, e)) = *cur {
                    *cur = if n == e { None } else { Some((n + 1, e)) };
                    return Some(n);
                }
                *cur = Some(*runs.next()?);
            },
            IterInner::Bitmap(b) => b.next(),
        }
    }
}

impl EntrySet {
    pub fn empty() -> Self {
        EntrySet { repr: Repr::Runs(Vec::new()), len: 0 }
    }

    pub fn from_ranges<I: IntoIterator<Item = Range<EntryId>>>(ranges: I) -> Self {
        let mut v: Vec<Run> = ranges
            .into_iter()
            .filter(|r| r.start < r.end)
            .map(|r| (r.start, r.end - 1))
            .collect();
        v.sort_unstable();
        Self::from_normalized(coalesce(v))
    }

    pub fn from_ids<I: IntoIterator<Item = EntryId>>(ids: I) -> Self {
        let mut v: Vec<EntryId> = ids.into_iter().collect();
        v.sort_unstable();
        v.dedup();
        let runs: Vec<Run> = v.into_iter().map(|i| (i, i)).collect();
        Self::from_normalized(coalesce(runs))
    }

    fn from_normalized(runs: Vec<Run>) -> Self {
        let len = runs.iter().map(|&(s, e)| (e - s) as u128 + 1).sum();
        let repr = if bitmap_worthwhile(&runs) {
            Repr::Bitmap(build_bitmap(&runs))
        } else {
            let mut runs = runs;
            runs.shrink_to_fit();
            Repr::Runs(runs)
        };
        EntrySet { repr, len }
    }

    fn run_slice(&self) -> Cow<'_, [Run]> {
        match &self.repr {
            Repr::Runs(v) => Cow::Borrowed(v),
            Repr::Bitmap(m) => Cow::Owned(bitmap_runs(m)),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> u128 {
        self.len
    }

    pub fn contains(&self, id: EntryId) -> bool {
        match &self.repr {
            Repr::Runs(v) => {
                let idx = v.partition_point(|&(s, _)| s <= id);
                idx > 0 && v[idx - 1].1 >= id
            }
            Repr::Bitmap(m) => {
                let mask = (1u64 << CHUNK_BITS) - 1;
                match m.get(&(id >> CHUNK_BITS)) {
                    Some(w) => {
                        let off = id & mask;
                        (w[(off >> 6) as usize] >> (off & 63)) & 1 == 1
                    }
                    None => false,
                }
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = EntryId> + '_ {
        match &self.repr {
            Repr::Runs(v) => IterInner::Runs { runs: v.iter(), cur: None },
            Repr::Bitmap(m) => IterInner::Bitmap(BitIter {
                chunks: m.iter(),
                chunk: 0,
                words: None,
                word: 0,
                bits: 0,
            }),
        }
    }

    pub fn union(&self, other: &EntrySet) -> EntrySet {
        if other.is_empty() {
            return self.clone();
        }
        if self.is_empty() {
            return other.clone();
        }
        let a = self.run_slice();
        let b = other.run_slice();
        let mut merged: Vec<Run> = Vec::with_capacity(a.len() + b.len());
        let (mut i, mut j) = (0, 0);
        while i < a.len() || j < b.len() {
            if j >= b.len() || (i < a.len() && a[i] <= b[j]) {
                merged.push(a[i]);
                i += 1;
            } else {
                merged.push(b[j]);
                j += 1;
            }
        }
        Self::from_normalized(coalesce(merged))
    }

    pub fn intersection(&self, other: &EntrySet) -> EntrySet {
        if self.is_empty() || other.is_empty() {
            return EntrySet::empty();
        }
        let a = self.run_slice();
        let b = other.run_slice();
        let mut out: Vec<Run> = Vec::new();
        let (mut i, mut j) = (0, 0);
        while i < a.len() && j < b.len() {
            let lo = a[i].0.max(b[j].0);
            let hi = a[i].1.min(b[j].1);
            if lo <= hi {
                out.push((lo, hi));
            }
            if a[i].1 < b[j].1 {
                i += 1;
            } else {
                j += 1;
            }
        }
        Self::from_normalized(out)
    }

    pub fn runs(&self) -> Vec<RangeInclusive<EntryId>> {
        self.run_slice().iter().map(|&(s, e)| s..=e).collect()
    }
}

impl Default for EntrySet {
    fn default() -> Self {
        EntrySet::empty()
    }
}

impl PartialEq for EntrySet {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && *self.run_slice() == *other.run_slice()
    }
}

impl Eq for EntrySet {}

impl Hash for EntrySet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.len.hash(state);
        for run in self.run_slice().iter() {
            run.hash(state);
        }
    }
}

#[cfg(test)]
#[path = "test/scopes_and_entry_sets.rs"]
mod tests;
