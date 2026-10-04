//! Quantised radix-4 trie for sets of integer keys (trigram IDs).
//!
//! Two representations of the same set of `u32` keys:
//!
//! * [`CompactTrie`] - a compact, serialisable pre-order radix-4 bitstream.
//! * [`CellMask`] - a flat `u64` bitmask; matching against a query is `AND` +
//!   popcount only.
//!
//! Keys are quantised to *cells* (`cell = key >> quant_bits`); several keys may
//! share a cell, which only ever produces false positives at query time.
#![forbid(unsafe_code)]
use std::fmt;
/// Errors raised by the trie; the decoder never panics on malformed input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Parameter combination violates the constraints in `TrieParams::new`.
    InvalidParams,
    /// Container magic was not `"GTRI"`.
    BadMagic,
    /// Container version was not `1`.
    BadVersion,
    /// The bitstream ended before a group could be read in full.
    Truncated,
    /// Bits remained after decoding finished.
    Overhang,
    /// A padding bit past `payload_bits` was set.
    Padding,
    /// A set flag pointed at a child group with no set flags.
    EmptySubgroup,
    /// A key (or cell) was `>= 2^key_bits` (resp. out of the cell range).
    KeyOutOfRange,
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Error::InvalidParams => "invalid trie parameters",
            Error::BadMagic => "bad container magic",
            Error::BadVersion => "unsupported container version",
            Error::Truncated => "truncated bitstream",
            Error::Overhang => "unexpected trailing bits",
            Error::Padding => "non-zero padding bit",
            Error::EmptySubgroup => "empty child subgroup",
            Error::KeyOutOfRange => "key out of range",
        };
        f.write_str(s)
    }
}
impl std::error::Error for Error {}
/// Trie shape parameters; fixed per index.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TrieParams {
    /// Width of a key in bits.
    pub key_bits: u8,
    /// Number of removed low key bits (`q`); `cell = key >> q`.
    pub quant_bits: u8,
    /// Top `r` bits of the cell form the direct root mask (`2^r` flags).
    pub root_bits: u8,
}
impl TrieParams {
    /// Builds parameters, validating the constraints from the spec:
    /// `1 <= r <= min(8, b)`, `(b - r)` even, `b <= 24`, where
    /// `b = key_bits - quant_bits`.
    pub fn new(key_bits: u8, quant_bits: u8, root_bits: u8) -> Result<Self, Error> {
        if key_bits == 0 || quant_bits > key_bits {
            return Err(Error::InvalidParams);
        }
        let b = (key_bits - quant_bits) as u32;
        let r = root_bits as u32;
        if b == 0 || b > 24 {
            return Err(Error::InvalidParams);
        }
        if r < 1 || r > b.min(8) {
            return Err(Error::InvalidParams);
        }
        if !(b - r).is_multiple_of(2) {
            return Err(Error::InvalidParams);
        }
        Ok(Self {
            key_bits,
            quant_bits,
            root_bits,
        })
    }
    /// Cell resolution `b = key_bits - quant_bits`.
    #[inline]
    pub fn cell_bits(&self) -> u32 {
        (self.key_bits - self.quant_bits) as u32
    }
    /// Number of radix-4 levels `L = (b - r) / 2`.
    #[inline]
    pub fn levels(&self) -> u32 {
        (self.cell_bits() - self.root_bits as u32) / 2
    }
    /// Highest valid key, inclusive.
    #[inline]
    fn max_key(&self) -> u64 {
        if self.key_bits >= 64 { u64::MAX } else { (1u64 << self.key_bits) - 1 }
    }
    /// Number of `u64` words needed for a [`CellMask`] (`2^b` bits).
    #[inline]
    fn mask_words(&self) -> usize {
        let cells = 1u64 << self.cell_bits();
        ((cells.div_ceil(64)) as usize).max(1)
    }
    /// Maps a key to its cell, erroring if the key is out of range.
    #[inline]
    fn cell_of(&self, key: u32) -> Result<u64, Error> {
        if key as u64 > self.max_key() {
            return Err(Error::KeyOutOfRange);
        }
        Ok((key as u64) >> self.quant_bits)
    }
}
#[inline]
fn get_bit(bytes: &[u8], i: usize) -> bool {
    (bytes[i / 8] >> (i % 8)) & 1 == 1
}
/// Collects the distinct, sorted cells for a set of keys.
fn sorted_cells(
    params: &TrieParams,
    keys: impl IntoIterator<Item = u32>,
) -> Result<Vec<u32>, Error> {
    let mut cells = Vec::new();
    for k in keys {
        cells.push(params.cell_of(k)? as u32);
    }
    cells.sort_unstable();
    cells.dedup();
    Ok(cells)
}
/// Compact, serialisable pre-order radix-4 bitstream of a key set.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompactTrie {
    params: TrieParams,
    payload: Vec<u8>,
    payload_bits: u32,
}
impl CompactTrie {
    /// Builds the canonical compact form from a set of keys.
    ///
    /// Output is deterministic: independent of input order and duplicates.
    pub fn build(
        params: TrieParams,
        keys: impl IntoIterator<Item = u32>,
    ) -> Result<Self, Error> {
        let cells = sorted_cells(&params, keys)?;
        let mut bits: Vec<bool> = Vec::new();
        emit(
            &mut bits,
            &cells,
            params.cell_bits() - params.root_bits as u32,
            1usize << params.root_bits,
        );
        let payload_bits = bits.len() as u32;
        let mut payload = vec![0u8; bits.len().div_ceil(8)];
        for (i, &bit) in bits.iter().enumerate() {
            if bit {
                payload[i / 8] |= 1 << (i % 8);
            }
        }
        Ok(Self {
            params,
            payload,
            payload_bits,
        })
    }
    /// Parameters this trie was built with.
    pub fn params(&self) -> TrieParams {
        self.params
    }
    /// Serialises to the self-describing container (header + payload).
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(12 + self.payload.len());
        out.extend_from_slice(b"GTRI");
        out.push(1);
        out.push(self.params.key_bits);
        out.push(self.params.quant_bits);
        out.push(self.params.root_bits);
        out.extend_from_slice(&self.payload_bits.to_le_bytes());
        out.extend_from_slice(&self.payload);
        out
    }
    /// Parses a self-describing container and validates its structure.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() < 12 {
            return Err(Error::Truncated);
        }
        if &bytes[0..4] != b"GTRI" {
            return Err(Error::BadMagic);
        }
        if bytes[4] != 1 {
            return Err(Error::BadVersion);
        }
        let params = TrieParams::new(bytes[5], bytes[6], bytes[7])?;
        let payload_bits = u32::from_le_bytes([
            bytes[8],
            bytes[9],
            bytes[10],
            bytes[11],
        ]);
        let payload = bytes[12..].to_vec();
        Self::from_payload(params, payload, payload_bits)
    }
    /// Payload bytes and bit length, without a header (shared `TrieParams`).
    pub fn to_payload(&self) -> (Vec<u8>, u32) {
        (self.payload.clone(), self.payload_bits)
    }
    /// Builds from a headerless payload with externally supplied parameters,
    /// validating the structure immediately.
    pub fn from_payload(
        params: TrieParams,
        bytes: Vec<u8>,
        payload_bits: u32,
    ) -> Result<Self, Error> {
        let trie = Self {
            params,
            payload: bytes,
            payload_bits,
        };
        trie.decode()?;
        Ok(trie)
    }
    /// Expands the compact form into a flat [`CellMask`] in one pass.
    ///
    /// Infallible: every constructor validates the structure beforehand.
    pub fn expand(&self) -> CellMask {
        let cells = self.decode().expect("trie structure validated at construction");
        let mut mask = CellMask::zeros(&self.params);
        for c in cells {
            mask.words[(c / 64) as usize] |= 1u64 << (c % 64);
        }
        mask
    }
    /// Decodes the bitstream to the set of set leaf cells (explicit stack,
    /// no recursion), validating canonicity and framing.
    fn decode(&self) -> Result<Vec<u32>, Error> {
        let payload_bits = self.payload_bits as usize;
        let total_bytes = payload_bits.div_ceil(8);
        if self.payload.len() < total_bytes {
            return Err(Error::Truncated);
        }
        for i in payload_bits..total_bytes * 8 {
            if get_bit(&self.payload, i) {
                return Err(Error::Padding);
            }
        }
        let b = self.params.cell_bits();
        let r = self.params.root_bits as u32;
        let mut pos = 0usize;
        let mut cells = Vec::new();
        let mut stack: Vec<(u32, u64, usize)> = vec![(b - r, 0u64, 1usize << r)];
        let mut is_root = true;
        while let Some((shift, prefix, nflags)) = stack.pop() {
            if pos + nflags > payload_bits {
                return Err(Error::Truncated);
            }
            let mut set: Vec<usize> = Vec::new();
            for j in 0..nflags {
                if get_bit(&self.payload, pos + j) {
                    set.push(j);
                }
            }
            pos += nflags;
            if is_root {
                is_root = false;
            } else if set.is_empty() {
                return Err(Error::EmptySubgroup);
            }
            if shift == 0 {
                for &k in &set {
                    cells.push((prefix | k as u64) as u32);
                }
            } else {
                for &k in set.iter().rev() {
                    let child_prefix = prefix | ((k as u64) << shift);
                    stack.push((shift - 2, child_prefix, 4));
                }
            }
        }
        if pos != payload_bits {
            return Err(Error::Overhang);
        }
        Ok(cells)
    }
}
/// Emits the canonical pre-order bitstream of `cells` (sorted, within range).
fn emit(bits: &mut Vec<bool>, cells: &[u32], shift: u32, nflags: usize) {
    let mask = (nflags as u32) - 1;
    let mut present = vec![false; nflags];
    let mut ranges = vec![(0usize, 0usize); nflags];
    let mut i = 0;
    while i < cells.len() {
        let v = ((cells[i] >> shift) & mask) as usize;
        let start = i;
        while i < cells.len() && ((cells[i] >> shift) & mask) as usize == v {
            i += 1;
        }
        present[v] = true;
        ranges[v] = (start, i);
    }
    for &p in &present {
        bits.push(p);
    }
    if shift == 0 {
        return;
    }
    for v in 0..nflags {
        if present[v] {
            let (s, e) = ranges[v];
            emit(bits, &cells[s..e], shift - 2, 4);
        }
    }
}
/// Flat in-memory bitmask over `2^b` cells, LSB-first.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellMask {
    words: Box<[u64]>,
    params: TrieParams,
}
impl CellMask {
    fn zeros(params: &TrieParams) -> Self {
        Self {
            words: vec![0u64; params.mask_words()].into_boxed_slice(),
            params: *params,
        }
    }
    /// Builds a mask directly from keys.
    pub fn from_keys(
        params: TrieParams,
        keys: impl IntoIterator<Item = u32>,
    ) -> Result<Self, Error> {
        let mut mask = Self::zeros(&params);
        for k in keys {
            let c = params.cell_of(k)?;
            mask.words[(c / 64) as usize] |= 1u64 << (c % 64);
        }
        Ok(mask)
    }
    /// Whether the cell of `key` is set.
    pub fn contains_key(&self, key: u32) -> bool {
        let c = match self.params.cell_of(key) {
            Ok(c) => c,
            Err(_) => return false,
        };
        (self.words[(c / 64) as usize] >> (c % 64)) & 1 == 1
    }
    /// Number of set cells.
    pub fn count_ones(&self) -> u32 {
        self.words.iter().map(|w| w.count_ones()).sum()
    }
    /// In-place union with another mask of the same parameters.
    pub fn union_with(&mut self, other: &CellMask) {
        for (a, b) in self.words.iter_mut().zip(other.words.iter()) {
            *a |= *b;
        }
    }
    /// Raw words (LSB-first).
    pub fn words(&self) -> &[u64] {
        &self.words
    }
}
/// Query side: sparse word list of set cells plus a popcount suffix array for
/// the early-abort in [`matches_at_least`].
#[derive(Clone, Debug)]
pub struct QueryMask {
    /// Distinct query keys.
    pub n_keys: u32,
    /// Distinct set cells.
    pub n_cells: u32,
    /// `(word_idx, mask)`, ascending by index.
    sparse: Vec<(usize, u64)>,
    /// `suffix_pop[i]` = sum of popcounts of masks `i..`.
    suffix_pop: Vec<u32>,
}
impl QueryMask {
    /// Builds a query mask from the query keys with the given parameters.
    pub fn new(
        params: TrieParams,
        keys: impl IntoIterator<Item = u32>,
    ) -> Result<Self, Error> {
        let mut distinct_keys = std::collections::BTreeSet::new();
        for k in keys {
            params.cell_of(k)?;
            distinct_keys.insert(k);
        }
        let mut words: std::collections::BTreeMap<usize, u64> = std::collections::BTreeMap::new();
        for &k in &distinct_keys {
            let c = (k as u64) >> params.quant_bits;
            *words.entry((c / 64) as usize).or_insert(0) |= 1u64 << (c % 64);
        }
        let sparse: Vec<(usize, u64)> = words.into_iter().collect();
        let n_cells: u32 = sparse.iter().map(|(_, m)| m.count_ones()).sum();
        let mut suffix_pop = vec![0u32; sparse.len() + 1];
        for i in (0..sparse.len()).rev() {
            suffix_pop[i] = suffix_pop[i + 1] + sparse[i].1.count_ones();
        }
        Ok(Self {
            n_keys: distinct_keys.len() as u32,
            n_cells,
            sparse,
            suffix_pop,
        })
    }
    /// Corrects a cell threshold for query-internal collisions.
    pub fn corrected_threshold(&self, t: u32) -> u32 {
        t.saturating_sub(self.n_keys - self.n_cells)
    }
}
/// Number of query cells also present in `file`.
pub fn matched_cells(file: &CellMask, q: &QueryMask) -> u32 {
    q.sparse.iter().map(|&(w, m)| (file.words[w] & m).count_ones()).sum()
}
/// Whether at least `t_cells` query cells are present in `file`, with early
/// abort from both sides.
pub fn matches_at_least(file: &CellMask, q: &QueryMask, t_cells: u32) -> bool {
    if t_cells == 0 {
        return true;
    }
    let mut sum = 0u32;
    for (i, &(w, m)) in q.sparse.iter().enumerate() {
        sum += (file.words[w] & m).count_ones();
        if sum >= t_cells {
            return true;
        }
        if sum + q.suffix_pop[i + 1] < t_cells {
            return false;
        }
    }
    sum >= t_cells
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test_vector_bytes_and_mask() {
        let params = TrieParams::new(6, 0, 2).unwrap();
        let trie = CompactTrie::build(params, [13u32, 14, 48]).unwrap();
        let (payload, bits) = trie.to_payload();
        assert_eq!(bits, 20);
        assert_eq!(payload, vec![0x89, 0x16, 0x01]);
        let mask = trie.expand();
        assert_eq!(mask.words(), & [0x0001_0000_0000_6000u64]);
    }
    #[test]
    fn container_roundtrip() {
        let params = TrieParams::new(6, 0, 2).unwrap();
        let trie = CompactTrie::build(params, [13u32, 14, 48]).unwrap();
        let bytes = trie.to_bytes();
        let back = CompactTrie::from_bytes(&bytes).unwrap();
        assert_eq!(trie, back);
    }
    #[test]
    fn order_and_dup_invariance() {
        let params = TrieParams::new(6, 0, 2).unwrap();
        let a = CompactTrie::build(params, [48u32, 14, 13, 13, 48]).unwrap();
        let b = CompactTrie::build(params, [13u32, 14, 48]).unwrap();
        assert_eq!(a.to_bytes(), b.to_bytes());
    }
    #[test]
    fn edge_cases() {
        for (kb, q, r) in [
            (6u8, 0u8, 2u8),
            (6, 0, 6),
            (16, 0, 4),
            (16, 4, 4),
            (10, 0, 2),
        ] {
            let params = TrieParams::new(kb, q, r).unwrap();
            let maxk = (1u32 << kb) - 1;
            for keys in [vec![], vec![0u32], vec![maxk], vec![0, maxk]] {
                let trie = CompactTrie::build(params, keys.clone()).unwrap();
                let expanded = trie.expand();
                let direct = CellMask::from_keys(params, keys.clone()).unwrap();
                assert_eq!(expanded, direct, "kb={kb} q={q} r={r} keys={keys:?}");
                for k in &keys {
                    assert!(expanded.contains_key(* k));
                }
            }
        }
    }
    #[test]
    fn full_set() {
        let params = TrieParams::new(6, 0, 2).unwrap();
        let all: Vec<u32> = (0..64).collect();
        let trie = CompactTrie::build(params, all.clone()).unwrap();
        let mask = trie.expand();
        assert_eq!(mask.count_ones(), 64);
        assert_eq!(mask.words(), & [u64::MAX]);
    }
    #[test]
    fn key_out_of_range() {
        let params = TrieParams::new(6, 0, 2).unwrap();
        assert_eq!(CompactTrie::build(params, [64u32]), Err(Error::KeyOutOfRange));
    }
    #[test]
    fn invalid_params() {
        assert!(TrieParams::new(6, 0, 3).is_err());
        assert!(TrieParams::new(6, 0, 0).is_err());
        assert!(TrieParams::new(6, 0, 7).is_err());
        assert!(TrieParams::new(26, 0, 2).is_err());
    }
    #[test]
    fn corrupt_inputs_never_panic() {
        let params = TrieParams::new(16, 0, 4).unwrap();
        let trie = CompactTrie::build(params, [1u32, 2, 1000, 50000, 65535]).unwrap();
        let good = trie.to_bytes();
        for n in 0..good.len() {
            let _ = CompactTrie::from_bytes(&good[..n]);
        }
        for byte in 0..good.len() {
            for bit in 0..8 {
                let mut c = good.clone();
                c[byte] ^= 1 << bit;
                let _ = CompactTrie::from_bytes(&c);
            }
        }
    }
    #[test]
    fn bad_magic_and_version() {
        let params = TrieParams::new(6, 0, 2).unwrap();
        let mut bytes = CompactTrie::build(params, [1u32]).unwrap().to_bytes();
        let mut bad = bytes.clone();
        bad[0] = b'X';
        assert_eq!(CompactTrie::from_bytes(& bad), Err(Error::BadMagic));
        bytes[4] = 2;
        assert_eq!(CompactTrie::from_bytes(& bytes), Err(Error::BadVersion));
    }
    #[test]
    fn query_matching_basic() {
        let params = TrieParams::new(16, 0, 4).unwrap();
        let file = CellMask::from_keys(params, [1u32, 2, 3, 100, 200]).unwrap();
        let q = QueryMask::new(params, [2u32, 3, 999]).unwrap();
        assert_eq!(matched_cells(& file, & q), 2);
        assert!(matches_at_least(& file, & q, 2));
        assert!(! matches_at_least(& file, & q, 3));
    }
    #[test]
    fn corrected_threshold_for_collisions() {
        let params = TrieParams::new(6, 2, 2).unwrap();
        let q = QueryMask::new(params, [0u32, 1, 2, 3]).unwrap();
        assert_eq!(q.n_keys, 4);
        assert_eq!(q.n_cells, 1);
        assert_eq!(q.corrected_threshold(4), 1);
    }
    use proptest::prelude::*;
    proptest! {
        #[test] fn prop_expand_equals_from_keys(keys in prop::collection::vec(0u32
        ..65536, 0..200)) { let params = TrieParams::new(16, 0, 4).unwrap(); let trie =
        CompactTrie::build(params, keys.clone()).unwrap(); prop_assert_eq!(trie.expand(),
        CellMask::from_keys(params, keys).unwrap()); } #[test] fn
        prop_contains_all_keys(keys in prop::collection::vec(0u32..65536, 0..200)) { let
        params = TrieParams::new(16, 0, 4).unwrap(); let mask =
        CompactTrie::build(params, keys.clone()).unwrap().expand(); for k in keys {
        prop_assert!(mask.contains_key(k)); } } #[test] fn
        prop_matched_equals_naive_q0(fkeys in prop::collection::vec(0u32..4096, 0..200),
        qkeys in prop::collection::vec(0u32..4096, 0..60),) { let params =
        TrieParams::new(12, 0, 4).unwrap(); let file = CellMask::from_keys(params, fkeys
        .clone()).unwrap(); let q = QueryMask::new(params, qkeys.clone()).unwrap(); let
        fset : HashSet < u32 > = fkeys.into_iter().collect(); let qset : HashSet < u32 >
        = qkeys.into_iter().collect(); let naive = qset.iter().filter(| k | fset
        .contains(k)).count() as u32; prop_assert_eq!(matched_cells(& file, & q), naive);
        } #[test] fn prop_matched_upper_bounds_real_qgt0(fkeys in
        prop::collection::vec(0u32..65536, 0..200), qkeys in prop::collection::vec(0u32
        ..65536, 0..60),) { let params = TrieParams::new(16, 4, 4).unwrap(); let file =
        CellMask::from_keys(params, fkeys.clone()).unwrap(); let q =
        QueryMask::new(params, qkeys.clone()).unwrap(); let fset : HashSet < u32 > =
        fkeys.into_iter().collect(); let qset : HashSet < u32 > = qkeys.into_iter()
        .collect(); let real = qset.iter().filter(| k | fset.contains(k)).count() as u32;
        prop_assert!(matched_cells(& file, & q) >= real); } #[test] fn
        prop_bytes_order_invariant(mut keys in prop::collection::vec(0u32..65536, 0
        ..200)) { let params = TrieParams::new(16, 0, 4).unwrap(); let a =
        CompactTrie::build(params, keys.clone()).unwrap().to_bytes(); keys.reverse();
        keys.extend_from_within(..); let b = CompactTrie::build(params, keys).unwrap()
        .to_bytes(); prop_assert_eq!(a, b); } #[test] fn
        prop_matches_at_least_agrees(fkeys in prop::collection::vec(0u32..65536, 0..200),
        qkeys in prop::collection::vec(0u32..65536, 0..60), t in 0u32..40,) { let params
        = TrieParams::new(16, 0, 4).unwrap(); let file = CellMask::from_keys(params,
        fkeys).unwrap(); let q = QueryMask::new(params, qkeys).unwrap();
        prop_assert_eq!(matches_at_least(& file, & q, t), matched_cells(& file, & q) >=
        t); }
    }
}
