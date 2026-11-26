use std::cmp::Ordering;
use std::fmt;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::b::similarity_task::SimilarityParams;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::b::operation::ScoredHitResult;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::result_kinds::Hit2;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::EntryId;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::b::task_context::TaskId;
const MIN_STD_DEV: f64 = 1e-9;
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ReportOptions {
    pub temperature: f32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReportHit {
    pub entry_id: EntryId,
    pub score: f32,
    pub stale: bool,
    pub source_link: Option<String>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Softmax {
    pub temperature: f32,
    pub probabilities: Vec<f64>,
    pub top_probability: f64,
    pub entropy: f64,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Knee {
    pub rank: usize,
    pub entry_id: EntryId,
    pub depth: f64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Metrics {
    pub count: usize,
    pub best: f32,
    pub worst: f32,
    pub mean: f64,
    pub std_dev: f64,
    pub gap: Option<f64>,
    pub spread: Option<f64>,
    pub z_score: Option<f64>,
    pub softmax: Option<Softmax>,
    pub knee: Option<Knee>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SimilarityReport {
    pub task_id: TaskId,
    pub params: SimilarityParams,
    pub candidate_count: usize,
    pub dropped_count: usize,
    pub stale_count: usize,
    pub hits: Vec<ReportHit>,
    pub metrics: Option<Metrics>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReportError {
    MissingSearchParams,
}
impl fmt::Display for ReportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReportError::MissingSearchParams => {
                f.write_str("similarity search parameters are missing")
            }
        }
    }
}
impl std::error::Error for ReportError {}
fn rank_order3(a: &Hit2, b: &Hit2) -> Ordering {
    b.score
        .partial_cmp(&a.score)
        .unwrap_or(Ordering::Equal)
        .then_with(|| a.id.cmp(&b.id))
}
pub fn build_report(
    task_id: TaskId,
    params: Option<&SimilarityParams>,
    input: &ScoredHitResult,
    options: ReportOptions,
    source_link: &dyn Fn(EntryId) -> Option<String>,
) -> Result<SimilarityReport, ReportError> {
    let params = params.ok_or(ReportError::MissingSearchParams)?;
    let mut valid: Vec<&Hit2> = input
        .hits
        .iter()
        .filter(|h| h.score.is_finite())
        .collect();
    let local_dropped = input.hits.len() - valid.len();
    valid.sort_by(|a, b| rank_order3(a, b));
    valid.truncate(params.top_k);
    let hits: Vec<ReportHit> = valid
        .iter()
        .map(|h| ReportHit {
            entry_id: h.id,
            score: h.score,
            stale: h.stale,
            source_link: source_link(h.id),
        })
        .collect();
    let stale_count = hits.iter().filter(|h| h.stale).count();
    let scores: Vec<f32> = hits.iter().map(|h| h.score).collect();
    let ids: Vec<EntryId> = hits.iter().map(|h| h.entry_id).collect();
    let metrics = compute_metrics(&scores, &ids, options.temperature);
    Ok(SimilarityReport {
        task_id,
        params: params.clone(),
        candidate_count: input.scored_count,
        dropped_count: input.dropped_count.saturating_add(local_dropped),
        stale_count,
        hits,
        metrics,
    })
}
pub fn compute_metrics(
    scores: &[f32],
    ids: &[EntryId],
    temperature: f32,
) -> Option<Metrics> {
    let n = scores.len();
    if n == 0 {
        return None;
    }
    let s: Vec<f64> = scores.iter().map(|&x| f64::from(x)).collect();
    let s0 = s[0];
    let last = s[n - 1];
    let nf = n as f64;
    let mean = s.iter().sum::<f64>() / nf;
    let variance = s.iter().map(|&x| (x - mean) * (x - mean)).sum::<f64>() / nf;
    let std_dev = variance.sqrt();
    let (gap, spread) = if n >= 2 {
        (Some(s0 - s[1]), Some(s0 - last))
    } else {
        (None, None)
    };
    let z_score = if std_dev < MIN_STD_DEV { None } else { Some((s0 - mean) / std_dev) };
    Some(Metrics {
        count: n,
        best: scores[0],
        worst: scores[n - 1],
        mean,
        std_dev,
        gap,
        spread,
        z_score,
        softmax: softmax(&s, temperature),
        knee: knee(&s, ids),
    })
}
fn softmax(s: &[f64], temperature: f32) -> Option<Softmax> {
    if !temperature.is_finite() || temperature <= 0.0 {
        return None;
    }
    let t = f64::from(temperature);
    let s0 = *s.first()?;
    let weights: Vec<f64> = s.iter().map(|&x| ((x - s0) / t).exp()).collect();
    let total: f64 = weights.iter().sum();
    let probabilities: Vec<f64> = weights.iter().map(|w| w / total).collect();
    let raw_entropy: f64 = -probabilities
        .iter()
        .filter(|&&p| p > 0.0)
        .map(|&p| p * p.ln())
        .sum::<f64>();
    let entropy = if raw_entropy > 0.0 { raw_entropy } else { 0.0 };
    Some(Softmax {
        temperature,
        top_probability: probabilities[0],
        probabilities,
        entropy,
    })
}
fn knee(s: &[f64], ids: &[EntryId]) -> Option<Knee> {
    let n = s.len();
    if n < 3 {
        return None;
    }
    let s0 = s[0];
    let slope = (s[n - 1] - s0) / (n - 1) as f64;
    let mut best: Option<(usize, f64)> = None;
    for (i, &si) in s.iter().enumerate().take(n - 1).skip(1) {
        let depth = s0 + slope * i as f64 - si;
        if depth > 0.0 && best.map_or(true, |(_, d)| depth > d) {
            best = Some((i, depth));
        }
    }
    let (rank, depth) = best?;
    let entry_id = *ids.get(rank)?;
    Some(Knee { rank, entry_id, depth })
}
const _: fn() = || {
    fn assert_traits<T: Send + Sync + 'static>() {}
    assert_traits::<ReportOptions>();
    assert_traits::<ReportHit>();
    assert_traits::<Softmax>();
    assert_traits::<Knee>();
    assert_traits::<Metrics>();
    assert_traits::<SimilarityReport>();
    assert_traits::<ReportError>();
};
#[cfg(test)]
#[path = "test/similarity_report.rs"]
mod tests;
