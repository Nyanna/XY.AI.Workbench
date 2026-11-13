# Specification: Benchmark-Workload und Abweichungsmaße (build order 88)

Crate module: `bench_workload`.

## 1. Task, restrictions and boundaries

**Task**
- Build a reproducible synthetic workload (documents and queries) from a seed. It is normalized and duplicate-free.
- Compute the FP32 reference on the CPU.
- Derive the query count from a batch-size value in a capability combination.
- Compute the deviation measures and classify the result into validity states.

**Restrictions**
- Use only `std` and the dependency modules. Add no RNG crates, BLAS crates or `unsafe`.
- Do not redefine types of the dependencies.
- The module does no GPU access, no candidate execution, no I/O and no global state.
- It does not choose a maximum deviation. The caller supplies it.
- A "row" means one query's score vector over all documents. Documents and queries are single-token, so `max_sim` equals `dot`.

## 2. Context

- A benchmark/validation layer runs a candidate backend (GPU, quantized or otherwise) on this workload. It compares the candidate's flat score matrix with the FP32 CPU reference from this module and gets a validity verdict.
- `cpu_compute` supplies the reference scoring and the normalization.
- `topk` supplies the top-k ordering and tie rule.
- `query_batch` supplies `RamSource` as the document source and `QueryBatch` as the query container.
- `capability` supplies the capability combination that carries the `batch_size` axis.

## 3. Interface

```rust
use capability::CapabilitySet;
use cpu_compute::CpuComputeError;
use query_batch::{QueryBatch, RamSource, ShapeError, SourceError};
use topk::{Hit, TopKError};

pub const DEFAULT_DOCS: usize = 20_000;
pub const DEFAULT_DIM: usize = 384;
pub const DEFAULT_MAX_QUERIES: usize = 32;
pub const DEFAULT_TOP_K: usize = 5;
pub const DEFAULT_QUERIES: usize = 8;      // used when batch_size is absent
pub const NOISE_FLOOR: f64 = 1e-6;
pub const CONSTANT_ROW_EPS: f64 = 1e-9;

#[derive(Debug, Clone, PartialEq)]
pub enum BenchError {
    InvalidConfig { what: String },
    CannotBuildUnique { what: &'static str },   // "documents" | "queries"
    TooManyQueries { requested: usize, max: usize },
    Shape(ShapeError), Source(SourceError), Compute(CpuComputeError), TopK(TopKError),
}
// Display + std::error::Error + Send + Sync; From for the four wrapped types.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchError {
    NotNumeric { value: String },
    Zero,
    TooLarge { value: String, max: usize },
    Ambiguous { count: usize },       // more than one value on the batch_size axis
}
// Display + std::error::Error + Send + Sync.

#[derive(Debug, Clone)]
pub struct WorkloadConfig {
    pub seed: u64, pub docs: usize, pub dim: usize, pub max_queries: usize, pub top_k: usize,
}
impl WorkloadConfig { pub fn new(seed: u64) -> Self; }  // defaults above

pub struct Workload { /* config, RamSource (SegmentId(0)), QueryBatch of max_queries queries */ }
impl Workload {
    pub fn build(cfg: WorkloadConfig) -> Result<Workload, BenchError>;
    pub fn config(&self) -> &WorkloadConfig;
    pub fn source(&self) -> &RamSource;
    pub fn segment(&self) -> query_batch::SegmentId;      // SegmentId(0)
    /// All max_queries queries; "smaller batches use the first queries".
    pub fn queries(&self) -> &QueryBatch;
    /// Query-major flat FP32 CPU reference, length n*docs, index q*docs + d.
    pub fn reference_scores(&self, n_queries: usize) -> Result<Vec<f32>, BenchError>;
    /// Reference top-k (k = config.top_k) per query via topk::RangeScorer; index = query.
    pub fn reference_top_k(&self, n_queries: usize) -> Result<Vec<Vec<Hit>>, BenchError>;
    /// Reference + assess. `candidate`: Ok(flat scores, query-major) or Err(failure text).
    pub fn evaluate(&self, n_queries: usize, candidate: Result<&[f32], String>,
                    cfg: &DeviationConfig) -> Result<Assessment, BenchError>;
}

/// Query count from a capability combination (axis capability::axes::batch_size()).
pub fn query_count(combo: &CapabilitySet, max_queries: usize) -> Result<usize, BatchError>;
/// Same rule for a raw value; None = absent.
pub fn query_count_from_value(value: Option<&str>, max_queries: usize) -> Result<usize, BatchError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidityKind { Valid, NonFinite, Degenerate, ExceedsMax, ExecutionFailed }
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Validity { pub kind: ValidityKind, pub text: String }   // text is always non-empty

#[derive(Debug, Clone)]
pub struct DeviationConfig {
    pub max_deviation: f64, pub top_k: usize,
    pub ratio_min: f64, pub ratio_max: f64,
}
impl DeviationConfig {
    /// top_k = 5, ratio 0.5..=2.0. Err(InvalidConfig) if max_deviation is NaN or < 0,
    /// or ratio_min > ratio_max, or top_k == 0 (checked also for direct field edits in assess).
    pub fn new(max_deviation: f64) -> Result<Self, BenchError>;
    pub fn with_ratio_range(self, min: f64, max: f64) -> Result<Self, BenchError>;
    pub fn with_top_k(self, k: usize) -> Result<Self, BenchError>;
}

#[derive(Debug, Clone, PartialEq)]
pub struct RowMetrics { pub cosine_deviation: f64, pub relative_error: f64,
                        pub overlap: f64, pub row_deviation: f64 }
#[derive(Debug, Clone, PartialEq)]
pub struct Assessment {
    pub validity: Validity,
    pub rows: Vec<RowMetrics>,            // empty unless metrics were computed
    pub worst_deviation: Option<f64>,     // max row_deviation; None if not computed
}

// Metrics (pure, f64 accumulation, equal-length required)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LengthMismatch { pub left: usize, pub right: usize }
pub fn cosine_deviation(reference: &[f32], candidate: &[f32]) -> Result<f64, LengthMismatch>;
pub fn is_noise(deviation: f64) -> bool;       // deviation < NOISE_FLOOR
pub fn relative_error(reference: &[f32], candidate: &[f32]) -> Result<f64, LengthMismatch>;
pub fn top_k_overlap(reference: &[f32], candidate: &[f32], k: usize) -> Result<f64, LengthMismatch>;

/// Row-wise assessment of flat matrices with row length `row_len`.
pub fn assess(reference: &[f32], candidate: &[f32], row_len: usize, cfg: &DeviationConfig) -> Assessment;
pub fn execution_failed(text: impl Into<String>) -> Assessment;
```

All types are `Send + Sync`. All functions are pure or `&self`.

## 4. Behavior and invariants

**Workload build**
- `build` is deterministic. The same config gives bit-identical documents and queries on every platform.
- Generator: SplitMix64 with state = `seed`. Each f32 component is `((next_u64() >> 40) as f32 / 16_777_216.0) * 2.0 - 1.0`.
- Draw order: documents 0..docs (component order), then queries 0..max_queries, from one continuous stream.
- Each vector is normalized with `cpu_compute::l2_normalize`.
- A drawn vector is rejected and redrawn if it is the zero vector, or if its bit pattern (`f32::to_bits`) equals one already accepted in its own set (documents, or queries).
- After 1000 consecutive rejections, `build` returns `CannotBuildUnique { what }`. Example: `dim == 1` with more than 2 vectors.
- Config checks, each returning `InvalidConfig`: `docs`, `dim`, `max_queries` and `top_k` must each be ≥ 1. `docs` and `dim` must fit in `u32`.
- Source layout: `RamSource::new(dim)`, one segment `SegmentId(0)`, `docs` slots with `Some(vec)` of exactly one token each.
- Queries: `Query::new(dim, vec)` collected into a `QueryBatch`, in draw order.
- Every vector has L2 norm 1 (within f32 rounding), all values are finite, and there are no duplicates.

**Reference**
- `reference_scores(n)` and `reference_top_k(n)` return `TooManyQueries` if `n > max_queries`. `n == 0` yields an empty result.
- `reference_scores(n)` calls `cpu_compute::scan_slots` with the first `n` queries' `data()`. The slots are `source.slot_vectors(seg, Slot(i))` for `0..docs`, over the range `0..docs`. The layout is query-major.
- The reference is computed freshly on each call and is identical across calls.
- `reference_top_k(n)` uses `RangeScorer::new(n, config.top_k)` and one `add_range(source, seg, Workspace::new(0, docs), &scores)`, then `into_results()`.

**Query count**
- `query_count_from_value(None, max)` returns `min(8, max)`.
- Given a value: it must be non-empty and consist only of ASCII digits (no sign, no whitespace). Otherwise `NotNumeric`.
- A value of 0 gives `Zero`.
- A value greater than `max_queries`, or one that overflows `u64`, gives `TooLarge`.
- Otherwise the value is returned. Leading zeros are allowed.
- `query_count(combo, max)` reads `combo.values(&axes::batch_size())`.
  - Axis absent → treated as absent value.
  - Exactly one value → the rule above.
  - More than one value → `Ambiguous { count }`.
- Any `BatchError` means the combination is unusable. The caller must not run it.
- Smaller batches use `queries()[0..n]`.

**Metrics**
- `cosine_deviation`:
  - Zero if either vector has norm exactly 0.0, wait: one if either vector has norm exactly 0.0.
  - Otherwise `max(0.0, 1.0 - dot/(|a|·|b|))`.
  - It never returns a negative value.
  - Empty inputs have norm 0, so the result is 1.0.
- `is_noise(d)` is `d < 1e-6`.
- `relative_error` is `‖c − r‖₂ / ‖r‖₂`. If `‖r‖ == 0`, it is `0.0` when `c` is also all-zero (distance 0), otherwise `+∞`.
- `top_k_overlap`:
  - Take the top `k` positions of the reference scores and of the candidate scores, using `topk::TopK` with `offset = position as u64`. This gives descending score and ties by lower position.
  - Result = |intersection of position sets| / (number of reference hits).
  - If there are zero reference hits (k = 0, empty row, or all NaN), the result is `1.0`.
  - The result is in [0, 1].
- All three metrics return `LengthMismatch` for unequal lengths.

**assess**
Checks run in this order. The first failing check decides. Each text names the cause, with the row index where relevant.
1. **Degenerate (shape)**: `reference.len() != candidate.len()`, `row_len == 0`, empty data, or `len % row_len != 0`. No rows are computed.
2. **NonFinite**: any NaN or ±∞ in either matrix. The text gives the matrix and the first position.
3. **Degenerate (row)**: for each row, in either the reference or the candidate row:
   - The row is constant: `variance < 1e-9 · mean_square`, with variance = mean((x − mean)²). An all-zero row also counts as constant.
   - Or the length ratio `‖cand‖/‖ref‖` is outside `[ratio_min, ratio_max]`. A zero reference norm counts as outside.
   - The text gives the first offending row and the reason.
4. **Metrics**: per row, compute `cosine_deviation`, `relative_error` and `top_k_overlap(cfg.top_k)`.
   - `row_deviation = max(cd', 1.0 − overlap)`, where `cd' = 0.0` if `is_noise(cd)`, else `cd`.
   - `worst_deviation` is the maximum `row_deviation` (the worst row).
   - `relative_error` is reported per row only and does not enter `row_deviation`.
5. **ExceedsMax**: `worst_deviation > cfg.max_deviation`. The text gives the worst row and the values.
6. **Valid**: otherwise. The text is non-empty (e.g. a summary).
- Results for checks 1–3 have empty `rows` and `worst_deviation = None`. Check 5 and the Valid result carry all rows and `Some(worst)`.
- `execution_failed(text)` gives `ExecutionFailed` with that text (replaced by `"execution failed"` if empty). It has empty rows and `None`.
- `evaluate`:
  - Candidate `Err(t)` → `execution_failed(t)`.
  - Candidate `Ok(c)` → `assess(reference_scores(n), c, docs, cfg)`.
  - `n == 0` or `n > max_queries` → `Err(TooManyQueries)` only for `n > max_queries`.
  - With `n == 0` the data is empty, so the result is Degenerate.
- `cfg` fields that were edited directly are re-validated in `assess`. Invalid values (NaN `max_deviation`, `top_k == 0`, `ratio_min > ratio_max`) give `Degenerate` with text `"invalid configuration"`.

**Concurrency and errors**
- No shared mutable state and no locks. `Workload` is immutable after `build`. Concurrent calls on `&Workload` and on the pure functions are safe.
- No panics for any input. All failures are returned as `BenchError`, `BatchError`, `LengthMismatch` or a `Validity`.

## 5. Relied-on elements

- `cpu_compute::scan_slots(queries: &[&[f32]], slots: &[Option<&[f32]>], range: Range<usize>, dim: usize) -> Result<Vec<f32>, CpuComputeError>`: flat result, index `q*range.len() + (slot - range.start)`.
- `cpu_compute::l2_normalize(&mut [f32])`: a zero norm leaves the vector unchanged.
- `topk::TopK::{new, push, into_sorted_vec}`:
  - Order is descending score, ties by lower offset.
  - NaN is discarded.
- `topk::RangeScorer::{new(num_queries, k), add_range(&dyn DocSource, SegmentId, Workspace, &[f32]) -> Result<(), TopKError>, into_results() -> Vec<Vec<Hit>>}`:
  - `scores` is query-major, length `num_queries * ws.len()`.
  - The offset is `DocSource::slot_offset`.
- `topk::Hit { offset: u64, score: f32 }`.
- `query_batch::{Query::new(dim: u32, Vec<f32>), Query::data(), QueryBatch::{new, push, from_queries, len, get, data}, RamSource::{new(dim: u32), insert_segment(SegmentId, Vec<Option<Vec<f32>>>)}, DocSource::slot_vectors(SegmentId, Slot) -> Option<&[f32]>, Workspace::new(start, end), SegmentId(pub u64), ShapeError, SourceError}`.
- `kernel_layout::Slot(pub u32)`.
- `capability::{CapabilitySet::values(&Axis) -> Option<&BTreeSet<Value>>, Name::as_str(), axes::batch_size() -> Axis}`. Numeric values are strings, e.g. `Name::from_number(8)` is `"8"`.