# Spezifikation: Scopes und Entry-Mengen (Build-Order 16)

## 1. Task, restrictions and boundaries

Implement a Rust module (no external module dependencies) with two parts:
1. **`Scope`**: a value type for the scope kinds all, range, set, subtree, prefix and centroid. Each kind has a stable name for logs and metrics.
2. **`EntrySet`**: a compact, immutable set of 64-bit entry IDs with set operations.

Out of scope:
- Resolving scopes against an index, tree or vectors. This includes subtree traversal, prefix matching and centroid search.
- Persistence, serialization and I/O.
- Any scope semantics beyond holding the parameters and the name.

## 2. Context

Other modules use `Scope` to say which entries an operation applies to. They use `EntrySet` to hold the result sets of such operations. This module resolves nothing itself; it only holds data and computes set algebra.

- IDs are `u64`, so the full range `0..=u64::MAX` must work.
- The scope names are used verbatim as log and metric labels. They must never change.
- New scope kinds will be added later, so the enums are `#[non_exhaustive]`.

## 3. Interface

```rust
pub type EntryId = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ScopeKind { All, Range, Set, Subtree, Prefix, Centroid }

impl ScopeKind {
    /// "all" | "range" | "set" | "subtree" | "prefix" | "centroid"
    pub const fn name(self) -> &'static str;
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Scope {
    All,
    /// Half-open ID range [start, end). May be empty or reversed (then it matches nothing).
    Range { start: EntryId, end: EntryId },
    Set(EntrySet),
    /// Subtree rooted at the node with this ID.
    Subtree { root: u64 },
    /// Byte prefix (may be empty).
    Prefix { bytes: Vec<u8> },
    /// Neighborhood of a centroid: number of entries and optional radius.
    Centroid { count: u32, radius: Option<f32> },
}

impl Scope {
    pub fn kind(&self) -> ScopeKind;
    pub fn name(&self) -> &'static str;   // == self.kind().name()
}

/// Immutable compact set of u64 IDs. Representation (ranges or bitmap) is chosen internally.
#[derive(Clone, Debug)]
pub struct EntrySet { /* private */ }

impl EntrySet {
    pub fn empty() -> Self;
    /// Half-open ranges; empty (start==end) or reversed (start>end) ranges are ignored.
    /// Ranges may be unsorted, overlapping or adjacent.
    pub fn from_ranges<I: IntoIterator<Item = std::ops::Range<EntryId>>>(ranges: I) -> Self;
    /// Single IDs; duplicates and any order allowed; u64::MAX is allowed.
    pub fn from_ids<I: IntoIterator<Item = EntryId>>(ids: I) -> Self;

    pub fn is_empty(&self) -> bool;
    pub fn len(&self) -> u128;                 // number of IDs; u128 so that 2^64 fits
    pub fn contains(&self, id: EntryId) -> bool;
    pub fn iter(&self) -> impl Iterator<Item = EntryId> + '_;   // strictly ascending
    pub fn union(&self, other: &EntrySet) -> EntrySet;
    pub fn intersection(&self, other: &EntrySet) -> EntrySet;
    /// Maximal contiguous runs, ascending, inclusive bounds, non-adjacent.
    /// Cost grows with the number of runs (expensive for large sparse sets).
    pub fn runs(&self) -> Vec<std::ops::RangeInclusive<EntryId>>;
}

impl PartialEq for EntrySet {}
impl Eq for EntrySet {}
impl std::hash::Hash for EntrySet {}
impl Default for EntrySet {}   // empty
```

`Scope`, `ScopeKind` and `EntrySet` are `Send + Sync + 'static`. Do not add interior mutability.

## 4. Behavior and invariants

**Scope**
- The names are exactly `"all"`, `"range"`, `"set"`, `"subtree"`, `"prefix"` and `"centroid"`. Each `Scope` variant maps to the matching `ScopeKind`.
- Constructing a `Scope` never fails and does not normalize its fields. For `Range`, `start >= end` is allowed and is simply an empty range.
- `Scope` equality is structural. `Set` compares by `EntrySet` equality. For `Centroid`, `radius` is compared as `f32` (`PartialEq`).

**EntrySet construction**
- `from_ranges` ignores ranges with `start >= end`. Overlapping or adjacent ranges merge.
- A half-open `Range` cannot contain `u64::MAX`. That ID is only reachable through `from_ids`, `union` or `intersection`.
- `from_ids` and `from_ranges` give the same set for the same content, regardless of order or duplicates.

**Operations**
- `is_empty` is true iff `len() == 0`.
- `len()` is the exact count. The full set `0..=u64::MAX` has `len() == 1u128 << 64`.
- `contains` is exact for every `u64`, including `0` and `u64::MAX`.
- `iter` yields each member exactly once, strictly ascending. It is lazy and does not allocate a list of all IDs. It handles a run ending at `u64::MAX` without overflow or an endless loop.
- `union` and `intersection` return new sets and leave their inputs unchanged. They are commutative and associative and correct with respect to the mathematical definitions. `intersection` with an empty set is empty. `union` with an empty set equals the other operand.
- `runs` returns the maximal contiguous runs in ascending order. Two consecutive runs `a..=b` and `c..=d` always satisfy `b + 1 < c`. An empty set gives an empty `Vec`. `runs` is computed on demand and may be expensive for large sets.
- Equality is by content only and independent of the internal representation. A set built as ranges equals the same set held as a bitmap.
- `Hash` is consistent with `Eq`: equal sets always hash equally, whatever their representation or construction path.

**Storage**
- Memory must be proportional to the number of runs, or to the bitmap size for dense but fragmented sets. It must never be proportional to the number of IDs for contiguous ranges. For example, `from_ranges([0..u64::MAX])` takes O(1) memory and `len()` is O(1)-ish.
- The internal choice between ranges and a bitmap is not observable, except through memory use and speed.
- A bitmap must not need memory proportional to the whole `u64` space. It must be chunked or sparse.

**Concurrency and errors**
- All operations are pure and have no global state, so shared `&EntrySet` use across threads is safe.
- No operation returns an error or panics for valid input. Allocation failure is the only exception.

## 5. Relied-on elements

None. This module has no dependencies.