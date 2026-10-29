//! Raw similarity scores plus the facts an outer layer needs to decide on follow-up tasks.
//! The engine decides nothing here; thresholds stay outside.
use super::params::SimilarityParams;
use crate::resident::index::payload::Payload as SourceRef;
use crate::resident::index::EntryId;
use crate::resident::manager::slot::Hit;
#[derive(Clone, Debug, PartialEq)]
pub struct RankedHit {
    pub entry: EntryId,
    pub score: f32,
    pub outdated: bool,
    pub source: Option<SourceRef>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SimilarityReport {
    pub task_id: u64,
    /// Entries that were scored (the resolved scope).
    pub candidates: u64,
    pub discarded: usize,
    pub outdated: usize,
    /// Best first; ties prefer the lower entry id.
    pub hits: Vec<RankedHit>,
    s1: Option<f32>,
    sk: Option<f32>,
    mean: Option<f32>,
    std: Option<f32>,
}
impl SimilarityReport {
    /// `hits` may be unordered and longer than `top_k`; NaN scores count as discarded.
    /// `discarded` carries entries already dropped upstream (removed from the index).
    pub fn build(
        task_id: u64,
        params: &SimilarityParams,
        candidates: u64,
        hits: Vec<Hit>,
        discarded: usize,
        source_of: &dyn Fn(EntryId) -> Option<SourceRef>,
    ) -> Self {
        let span = tracing::info_span!(
            "similarity", task_id, scope_kind = params.get_scope().kind(), scorer =
            params.get_scorer().label(), top_k = params.get_top_k(), candidate_count =
            candidates,
        );
        let _guard = span.enter();
        let total = hits.len();
        let mut valid: Vec<Hit> = hits
            .into_iter()
            .filter(|h| !h.score.is_nan())
            .collect();
        let discarded = discarded + (total - valid.len());
        valid
            .sort_by(|a, b| {
                b.score
                    .total_cmp(&a.score)
                    .then_with(|| a.entry.get().cmp(&b.entry.get()))
            });
        valid.truncate(params.get_top_k());
        let outdated = valid.iter().filter(|h| h.outdated).count();
        let hits: Vec<RankedHit> = valid
            .iter()
            .map(|h| RankedHit {
                entry: h.entry,
                score: h.score,
                outdated: h.outdated,
                source: source_of(h.entry),
            })
            .collect();
        let n = hits.len();
        let (mean, std) = if n == 0 {
            (None, None)
        } else {
            let mean = hits.iter().map(|h| f64::from(h.score)).sum::<f64>() / n as f64;
            let var = hits
                .iter()
                .map(|h| (f64::from(h.score) - mean).powi(2))
                .sum::<f64>() / n as f64;
            (Some(mean as f32), Some(var.sqrt() as f32))
        };
        let report = Self {
            task_id,
            candidates,
            discarded,
            outdated,
            s1: hits.first().map(|h| h.score),
            sk: hits.last().map(|h| h.score),
            mean,
            std,
            hits,
        };
        tracing::debug!(
            s1 = ? report.s1, sk = ? report.sk, mean = ? report.mean, std = ? report.std,
            returned = n, discarded, outdated, "similarity result"
        );
        report
    }
    pub fn count(&self) -> usize {
        self.hits.len()
    }
    pub fn s1(&self) -> Option<f32> {
        self.s1
    }
    pub fn sk(&self) -> Option<f32> {
        self.sk
    }
    /// Mean over the returned scores.
    pub fn mean(&self) -> Option<f32> {
        self.mean
    }
    /// Population standard deviation over the returned scores.
    pub fn std(&self) -> Option<f32> {
        self.std
    }
    /// `s1 - s2`; a small gap means the order of the leaders is unreliable.
    pub fn gap_top2(&self) -> Option<f32> {
        Some(self.hits.first()?.score - self.hits.get(1)?.score)
    }
    /// `s1 - sk`: spread of the whole result list.
    pub fn gap_to_k(&self) -> Option<f32> {
        (self.hits.len() > 1)
            .then(|| self.s1.unwrap_or_default() - self.sk.unwrap_or_default())
    }
    /// Distance of `s1` from the mean in standard deviations; `None` for a flat distribution.
    pub fn z_top1(&self) -> Option<f32> {
        let std = self.std?;
        if std <= f32::EPSILON {
            return None;
        }
        Some((self.s1? - self.mean?) / std)
    }
    /// Softmax over the returned scores. e5 scores live on a narrow band, so a temperature of
    /// about 0.01 to 0.05 is the useful range.
    pub fn softmax(&self, temperature: f32) -> Option<Vec<f32>> {
        if !(temperature.is_finite() && temperature > 0.0) || self.hits.is_empty() {
            return None;
        }
        let max = self.s1?;
        let exps: Vec<f64> = self
            .hits
            .iter()
            .map(|h| (f64::from(h.score - max) / f64::from(temperature)).exp())
            .collect();
        let sum: f64 = exps.iter().sum();
        Some(exps.iter().map(|e| (e / sum) as f32).collect())
    }
    /// Softmax probability mass of the best hit.
    pub fn top1_mass(&self, temperature: f32) -> Option<f32> {
        self.softmax(temperature)?.first().copied()
    }
    /// Shannon entropy (nats) of the softmax distribution; 0 means one clear winner.
    pub fn entropy(&self, temperature: f32) -> Option<f32> {
        let p = self.softmax(temperature)?;
        Some(p.iter().filter(|&&x| x > 0.0).map(|&x| -x * x.ln()).sum())
    }
    /// Index of the knee of the sorted score curve: the point farthest below the chord from the
    /// first to the last score. `None` for fewer than three hits or a flat curve.
    pub fn elbow(&self) -> Option<usize> {
        let n = self.hits.len();
        if n < 3 {
            return None;
        }
        let (first, last) = (self.hits[0].score, self.hits[n - 1].score);
        if first - last <= f32::EPSILON {
            return None;
        }
        let span = (n - 1) as f32;
        self.hits
            .iter()
            .enumerate()
            .map(|(i, h)| {
                let chord = first + (last - first) * (i as f32 / span);
                (i, chord - h.score)
            })
            .filter(|&(_, depth)| depth > f32::EPSILON)
            .max_by(|a, b| a.1.total_cmp(&b.1).then(b.0.cmp(&a.0)))
            .map(|(i, _)| i)
    }
}
#[cfg(test)]
#[path = "test/report.rs"]
mod tests;
