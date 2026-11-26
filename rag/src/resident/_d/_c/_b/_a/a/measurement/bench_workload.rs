use std::collections::HashSet;
use std::fmt;

use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::kernel_layout::Slot;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::cpu_compute::{
    l2_normalize, scan_slots, CpuComputeError,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::capability_model::{
    axes, CapabilitySet,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::query_batch::{
    DocSource, Query, QueryBatch, RamSource, SegmentId, ShapeError, SourceError, Workspace,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::topk::{
    Hit, RangeScorer, TopK, TopKError,
};

pub const DEFAULT_DOCS: usize = 20_000;
pub const DEFAULT_DIM: usize = 384;
pub const DEFAULT_MAX_QUERIES: usize = 32;
pub const DEFAULT_TOP_K: usize = 5;
pub const DEFAULT_QUERIES: usize = 8;
pub const NOISE_FLOOR: f64 = 1e-6;
pub const CONSTANT_ROW_EPS: f64 = 1e-9;

const MAX_CONSECUTIVE_REJECTIONS: usize = 1000;
const INVALID_CONFIGURATION: &str = "invalid configuration";

#[derive(Debug, Clone, PartialEq)]
pub enum BenchError {
    InvalidConfig { what: String },
    CannotBuildUnique { what: &'static str },
    TooManyQueries { requested: usize, max: usize },
    Shape(ShapeError),
    Source(SourceError),
    Compute(CpuComputeError),
    TopK(TopKError),
}

impl fmt::Display for BenchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BenchError::InvalidConfig { what } => write!(f, "invalid configuration: {what}"),
            BenchError::CannotBuildUnique { what } => {
                write!(f, "cannot build unique {what}")
            }
            BenchError::TooManyQueries { requested, max } => {
                write!(f, "too many queries: requested {requested}, maximum {max}")
            }
            BenchError::Shape(e) => write!(f, "shape error: {e}"),
            BenchError::Source(e) => write!(f, "source error: {e}"),
            BenchError::Compute(e) => write!(f, "compute error: {e}"),
            BenchError::TopK(e) => write!(f, "top-k error: {e}"),
        }
    }
}

impl std::error::Error for BenchError {}

impl From<ShapeError> for BenchError {
    fn from(e: ShapeError) -> Self {
        BenchError::Shape(e)
    }
}

impl From<SourceError> for BenchError {
    fn from(e: SourceError) -> Self {
        BenchError::Source(e)
    }
}

impl From<CpuComputeError> for BenchError {
    fn from(e: CpuComputeError) -> Self {
        BenchError::Compute(e)
    }
}

impl From<TopKError> for BenchError {
    fn from(e: TopKError) -> Self {
        BenchError::TopK(e)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BatchError {
    NotNumeric { value: String },
    Zero,
    TooLarge { value: String, max: usize },
    Ambiguous { count: usize },
}

impl fmt::Display for BatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BatchError::NotNumeric { value } => {
                write!(f, "batch size {value:?} is not a decimal number")
            }
            BatchError::Zero => write!(f, "batch size must not be zero"),
            BatchError::TooLarge { value, max } => {
                write!(f, "batch size {value} exceeds the maximum of {max}")
            }
            BatchError::Ambiguous { count } => {
                write!(f, "batch size axis carries {count} values, expected one")
            }
        }
    }
}

impl std::error::Error for BatchError {}

#[derive(Debug, Clone)]
pub struct WorkloadConfig {
    pub seed: u64,
    pub docs: usize,
    pub dim: usize,
    pub max_queries: usize,
    pub top_k: usize,
}

impl WorkloadConfig {
    pub fn new(seed: u64) -> Self {
        WorkloadConfig {
            seed,
            docs: DEFAULT_DOCS,
            dim: DEFAULT_DIM,
            max_queries: DEFAULT_MAX_QUERIES,
            top_k: DEFAULT_TOP_K,
        }
    }

    fn validate(&self) -> Result<(), BenchError> {
        let invalid = |what: &str| BenchError::InvalidConfig { what: what.to_string() };
        if self.docs == 0 {
            return Err(invalid("docs must be at least 1"));
        }
        if self.dim == 0 {
            return Err(invalid("dim must be at least 1"));
        }
        if self.max_queries == 0 {
            return Err(invalid("max_queries must be at least 1"));
        }
        if self.top_k == 0 {
            return Err(invalid("top_k must be at least 1"));
        }
        if u32::try_from(self.docs).is_err() {
            return Err(invalid("docs must fit in u32"));
        }
        if u32::try_from(self.dim).is_err() {
            return Err(invalid("dim must fit in u32"));
        }
        Ok(())
    }
}

struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn next_component(&mut self) -> f32 {
        ((self.next_u64() >> 40) as f32 / 16_777_216.0) * 2.0 - 1.0
    }
}

fn draw_unique_set(
    rng: &mut SplitMix64,
    count: usize,
    dim: usize,
    what: &'static str,
) -> Result<Vec<Vec<f32>>, BenchError> {
    let mut accepted: Vec<Vec<f32>> = Vec::with_capacity(count);
    let mut seen: HashSet<Vec<u32>> = HashSet::with_capacity(count);
    let mut rejections = 0usize;
    while accepted.len() < count {
        let mut v: Vec<f32> = (0..dim).map(|_| rng.next_component()).collect();
        l2_normalize(&mut v);
        let zero = v.iter().all(|x| *x == 0.0);
        let bits: Vec<u32> = v.iter().map(|x| x.to_bits()).collect();
        if zero || seen.contains(&bits) {
            rejections += 1;
            if rejections >= MAX_CONSECUTIVE_REJECTIONS {
                return Err(BenchError::CannotBuildUnique { what });
            }
            continue;
        }
        rejections = 0;
        seen.insert(bits);
        accepted.push(v);
    }
    Ok(accepted)
}

pub struct Workload {
    config: WorkloadConfig,
    source: RamSource,
    queries: QueryBatch,
}

impl Workload {
    pub fn build(cfg: WorkloadConfig) -> Result<Workload, BenchError> {
        cfg.validate()?;
        let dim32 = cfg.dim as u32;
        let mut rng = SplitMix64 { state: cfg.seed };
        let docs = draw_unique_set(&mut rng, cfg.docs, cfg.dim, "documents")?;
        let query_vecs = draw_unique_set(&mut rng, cfg.max_queries, cfg.dim, "queries")?;

        let mut source = RamSource::new(dim32)?;
        source.insert_segment(SegmentId(0), docs.into_iter().map(Some).collect())?;

        let mut queries = Vec::with_capacity(query_vecs.len());
        for v in query_vecs {
            queries.push(Query::new(dim32, v)?);
        }
        let queries = QueryBatch::from_queries(dim32, queries)?;

        Ok(Workload { config: cfg, source, queries })
    }

    pub fn config(&self) -> &WorkloadConfig {
        &self.config
    }

    pub fn source(&self) -> &RamSource {
        &self.source
    }

    pub fn segment(&self) -> SegmentId {
        SegmentId(0)
    }

    pub fn queries(&self) -> &QueryBatch {
        &self.queries
    }

    fn check_count(&self, n_queries: usize) -> Result<(), BenchError> {
        if n_queries > self.config.max_queries {
            return Err(BenchError::TooManyQueries {
                requested: n_queries,
                max: self.config.max_queries,
            });
        }
        Ok(())
    }

    pub fn reference_scores(&self, n_queries: usize) -> Result<Vec<f32>, BenchError> {
        self.check_count(n_queries)?;
        let docs = self.config.docs;
        let seg = self.segment();
        let query_data: Vec<&[f32]> = (0..n_queries)
            .map(|i| self.queries.data(i).unwrap_or(&[]))
            .collect();
        let slots: Vec<Option<&[f32]>> = (0..docs)
            .map(|i| self.source.slot_vectors(seg, Slot(i as u32)))
            .collect();
        Ok(scan_slots(&query_data, &slots, 0..docs, self.config.dim)?)
    }

    pub fn reference_top_k(&self, n_queries: usize) -> Result<Vec<Vec<Hit>>, BenchError> {
        let scores = self.reference_scores(n_queries)?;
        let mut scorer = RangeScorer::new(n_queries, self.config.top_k);
        scorer.add_range(
            &self.source,
            self.segment(),
            Workspace::new(0, self.config.docs as u32),
            &scores,
        )?;
        Ok(scorer.into_results())
    }

    pub fn evaluate(
        &self,
        n_queries: usize,
        candidate: Result<&[f32], String>,
        cfg: &DeviationConfig,
    ) -> Result<Assessment, BenchError> {
        self.check_count(n_queries)?;
        match candidate {
            Err(text) => Ok(execution_failed(text)),
            Ok(c) => {
                let reference = self.reference_scores(n_queries)?;
                Ok(assess(&reference, c, self.config.docs, cfg))
            }
        }
    }
}

pub fn query_count(combo: &CapabilitySet, max_queries: usize) -> Result<usize, BatchError> {
    match combo.values(&axes::batch_size()) {
        None => query_count_from_value(None, max_queries),
        Some(values) => match values.len() {
            0 => query_count_from_value(None, max_queries),
            1 => {
                let value = values.iter().next().map(|v| v.as_str());
                query_count_from_value(value, max_queries)
            }
            count => Err(BatchError::Ambiguous { count }),
        },
    }
}

pub fn query_count_from_value(value: Option<&str>, max_queries: usize) -> Result<usize, BatchError> {
    let Some(text) = value else {
        return Ok(DEFAULT_QUERIES.min(max_queries));
    };
    if text.is_empty() || !text.bytes().all(|b| b.is_ascii_digit()) {
        return Err(BatchError::NotNumeric { value: text.to_string() });
    }
    let too_large = || BatchError::TooLarge { value: text.to_string(), max: max_queries };
    let parsed: u64 = text.parse().map_err(|_| too_large())?;
    if parsed == 0 {
        return Err(BatchError::Zero);
    }
    match usize::try_from(parsed) {
        Ok(n) if n <= max_queries => Ok(n),
        _ => Err(too_large()),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidityKind {
    Valid,
    NonFinite,
    Degenerate,
    ExceedsMax,
    ExecutionFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Validity {
    pub kind: ValidityKind,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct DeviationConfig {
    pub max_deviation: f64,
    pub top_k: usize,
    pub ratio_min: f64,
    pub ratio_max: f64,
}

impl DeviationConfig {
    pub fn new(max_deviation: f64) -> Result<Self, BenchError> {
        let cfg = DeviationConfig {
            max_deviation,
            top_k: DEFAULT_TOP_K,
            ratio_min: 0.5,
            ratio_max: 2.0,
        };
        cfg.validate()?;
        Ok(cfg)
    }

    pub fn with_ratio_range(mut self, min: f64, max: f64) -> Result<Self, BenchError> {
        self.ratio_min = min;
        self.ratio_max = max;
        self.validate()?;
        Ok(self)
    }

    pub fn with_top_k(mut self, k: usize) -> Result<Self, BenchError> {
        self.top_k = k;
        self.validate()?;
        Ok(self)
    }

    fn validate(&self) -> Result<(), BenchError> {
        let invalid = |what: &str| BenchError::InvalidConfig { what: what.to_string() };
        if self.max_deviation.is_nan() || self.max_deviation < 0.0 {
            return Err(invalid("max_deviation must be a non-negative number"));
        }
        if !(self.ratio_min <= self.ratio_max) {
            return Err(invalid("ratio_min must not exceed ratio_max"));
        }
        if self.top_k == 0 {
            return Err(invalid("top_k must be at least 1"));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RowMetrics {
    pub cosine_deviation: f64,
    pub relative_error: f64,
    pub overlap: f64,
    pub row_deviation: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assessment {
    pub validity: Validity,
    pub rows: Vec<RowMetrics>,
    pub worst_deviation: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LengthMismatch {
    pub left: usize,
    pub right: usize,
}

impl fmt::Display for LengthMismatch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "length mismatch: {} vs {}", self.left, self.right)
    }
}

impl std::error::Error for LengthMismatch {}

fn check_lengths(a: &[f32], b: &[f32]) -> Result<(), LengthMismatch> {
    if a.len() != b.len() {
        return Err(LengthMismatch { left: a.len(), right: b.len() });
    }
    Ok(())
}

fn norm(v: &[f32]) -> f64 {
    v.iter().map(|x| f64::from(*x) * f64::from(*x)).sum::<f64>().sqrt()
}

pub fn cosine_deviation(reference: &[f32], candidate: &[f32]) -> Result<f64, LengthMismatch> {
    check_lengths(reference, candidate)?;
    let na = norm(reference);
    let nb = norm(candidate);
    if na == 0.0 || nb == 0.0 {
        return Ok(1.0);
    }
    let dot: f64 = reference
        .iter()
        .zip(candidate)
        .map(|(a, b)| f64::from(*a) * f64::from(*b))
        .sum();
    let d = 1.0 - dot / (na * nb);
    if d.is_nan() {
        return Ok(1.0);
    }
    Ok(d.max(0.0))
}

pub fn is_noise(deviation: f64) -> bool {
    deviation < NOISE_FLOOR
}

pub fn relative_error(reference: &[f32], candidate: &[f32]) -> Result<f64, LengthMismatch> {
    check_lengths(reference, candidate)?;
    let dist = reference
        .iter()
        .zip(candidate)
        .map(|(r, c)| {
            let d = f64::from(*c) - f64::from(*r);
            d * d
        })
        .sum::<f64>()
        .sqrt();
    let nr = norm(reference);
    if nr == 0.0 {
        return Ok(if dist == 0.0 { 0.0 } else { f64::INFINITY });
    }
    Ok(dist / nr)
}

fn top_positions(scores: &[f32], k: usize) -> Vec<Hit> {
    let mut top = TopK::new(k);
    for (i, s) in scores.iter().enumerate() {
        top.push(i as u64, *s);
    }
    top.into_sorted_vec()
}

pub fn top_k_overlap(reference: &[f32], candidate: &[f32], k: usize) -> Result<f64, LengthMismatch> {
    check_lengths(reference, candidate)?;
    let ref_hits = top_positions(reference, k);
    if ref_hits.is_empty() {
        return Ok(1.0);
    }
    let cand: HashSet<u64> = top_positions(candidate, k).iter().map(|h| h.offset).collect();
    let common = ref_hits.iter().filter(|h| cand.contains(&h.offset)).count();
    Ok(common as f64 / ref_hits.len() as f64)
}

fn verdict(kind: ValidityKind, text: String) -> Assessment {
    Assessment { validity: Validity { kind, text }, rows: Vec::new(), worst_deviation: None }
}

fn degenerate(text: String) -> Assessment {
    verdict(ValidityKind::Degenerate, text)
}

fn is_constant_row(row: &[f32]) -> bool {
    let n = row.len() as f64;
    let mean_square = row.iter().map(|x| f64::from(*x) * f64::from(*x)).sum::<f64>() / n;
    if mean_square == 0.0 {
        return true;
    }
    let mean = row.iter().map(|x| f64::from(*x)).sum::<f64>() / n;
    let variance = row
        .iter()
        .map(|x| {
            let d = f64::from(*x) - mean;
            d * d
        })
        .sum::<f64>()
        / n;
    variance < CONSTANT_ROW_EPS * mean_square
}

fn first_non_finite(m: &[f32]) -> Option<usize> {
    m.iter().position(|x| !x.is_finite())
}

pub fn assess(
    reference: &[f32],
    candidate: &[f32],
    row_len: usize,
    cfg: &DeviationConfig,
) -> Assessment {
    if cfg.validate().is_err() {
        return degenerate(INVALID_CONFIGURATION.to_string());
    }
    if reference.len() != candidate.len() {
        return degenerate(format!(
            "shape mismatch: reference has {} values, candidate has {}",
            reference.len(),
            candidate.len()
        ));
    }
    if row_len == 0 {
        return degenerate("row length is zero".to_string());
    }
    if reference.is_empty() {
        return degenerate("matrices are empty".to_string());
    }
    if reference.len() % row_len != 0 {
        return degenerate(format!(
            "matrix length {} is not a multiple of row length {}",
            reference.len(),
            row_len
        ));
    }

    for (name, m) in [("reference", reference), ("candidate", candidate)] {
        if let Some(pos) = first_non_finite(m) {
            return verdict(
                ValidityKind::NonFinite,
                format!("{name} matrix holds a non-finite value at position {pos} (row {})", pos / row_len),
            );
        }
    }

    let ref_rows = reference.chunks_exact(row_len);
    let cand_rows = candidate.chunks_exact(row_len);
    for (i, (r, c)) in ref_rows.zip(cand_rows).enumerate() {
        if is_constant_row(r) {
            return degenerate(format!("row {i}: reference row is constant"));
        }
        if is_constant_row(c) {
            return degenerate(format!("row {i}: candidate row is constant"));
        }
        let nr = norm(r);
        let ratio = norm(c) / nr;
        if nr == 0.0 || !(ratio >= cfg.ratio_min && ratio <= cfg.ratio_max) {
            return degenerate(format!(
                "row {i}: length ratio {ratio} outside [{}, {}]",
                cfg.ratio_min, cfg.ratio_max
            ));
        }
    }

    let mut rows = Vec::with_capacity(reference.len() / row_len);
    for (i, (r, c)) in reference
        .chunks_exact(row_len)
        .zip(candidate.chunks_exact(row_len))
        .enumerate()
    {
        let metrics = (|| {
            Ok::<_, LengthMismatch>((
                cosine_deviation(r, c)?,
                relative_error(r, c)?,
                top_k_overlap(r, c, cfg.top_k)?,
            ))
        })();
        let Ok((cd, re, overlap)) = metrics else {
            return degenerate(format!("row {i}: row length mismatch"));
        };
        let cd_eff = if is_noise(cd) { 0.0 } else { cd };
        rows.push(RowMetrics {
            cosine_deviation: cd,
            relative_error: re,
            overlap,
            row_deviation: cd_eff.max(1.0 - overlap),
        });
    }

    let mut worst_row = 0usize;
    let mut worst = rows[0].row_deviation;
    for (i, m) in rows.iter().enumerate() {
        if m.row_deviation > worst {
            worst = m.row_deviation;
            worst_row = i;
        }
    }

    let (kind, text) = if worst > cfg.max_deviation {
        let m = &rows[worst_row];
        (
            ValidityKind::ExceedsMax,
            format!(
                "row {worst_row}: deviation {worst} exceeds maximum {} (cosine {}, overlap {})",
                cfg.max_deviation, m.cosine_deviation, m.overlap
            ),
        )
    } else {
        (
            ValidityKind::Valid,
            format!(
                "{} rows within maximum deviation {} (worst {worst})",
                rows.len(),
                cfg.max_deviation
            ),
        )
    };
    Assessment { validity: Validity { kind, text }, rows, worst_deviation: Some(worst) }
}

pub fn execution_failed(text: impl Into<String>) -> Assessment {
    let mut text = text.into();
    if text.is_empty() {
        text = "execution failed".to_string();
    }
    verdict(ValidityKind::ExecutionFailed, text)
}

#[cfg(test)]
#[path = "test/bench_workload.rs"]
mod tests;
