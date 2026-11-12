//! Benchmark-driven hardware calibration producing a Pareto front over time and deviation.

use std::collections::{BTreeMap, HashSet};
use std::fmt;
use std::time::{Duration, Instant};

use super::super::l020::a::model_descriptor::{ModelConfigError, ModelDescriptor};
use super::super::l020::l010::capability_model::{axes, Axis, CapabilitySet, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    pub subengine: String,
    pub model_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Combination(pub BTreeMap<Axis, Value>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DispersionKind {
    Cv,
    RangeOverMedian,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EstimatorKind {
    Median,
    Minimum,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationConfig {
    pub max_deviation: f64,
    pub min_repeats: usize,
    pub time_window: Duration,
    pub stability_threshold: f64,
    pub chunk_budget: Duration,
    pub backoff: Duration,
    pub dispersion: DispersionKind,
    pub estimator: EstimatorKind,
    pub max_attempts: usize,
    pub max_duration: Option<Duration>,
    pub total_budget: Option<Duration>,
    pub dominance_margin: f64,
    pub separable_axes: Vec<Axis>,
    pub full_product: bool,
    pub seq_start: usize,
    pub seq_factor: usize,
}

impl Default for CalibrationConfig {
    fn default() -> Self {
        CalibrationConfig {
            max_deviation: 0.05,
            min_repeats: 3,
            time_window: Duration::from_millis(10),
            stability_threshold: 0.05,
            chunk_budget: Duration::from_millis(5),
            backoff: Duration::from_millis(5),
            dispersion: DispersionKind::Cv,
            estimator: EstimatorKind::Median,
            max_attempts: 40,
            max_duration: Some(Duration::from_secs(30)),
            total_budget: Some(Duration::from_secs(600)),
            dominance_margin: 0.10,
            separable_axes: vec![axes::batch_size(), axes::tile_size(), axes::thread_count()],
            full_product: false,
            seq_start: 16,
            seq_factor: 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProbeError {
    Failed(String),
    Rejected(String),
    Interrupted,
    Unavailable,
    Fatal(String),
}

pub trait CalibrationBackend: Send + Sync {
    fn deviation(&self, c: &Combination, seq_lens: &[usize]) -> Result<f64, ProbeError>;
    fn time(
        &self,
        c: &Combination,
        seq_lens: &[usize],
        window: Duration,
        chunk_budget: Duration,
    ) -> Result<Duration, ProbeError>;
}

pub trait Statistics: Send + Sync {
    fn dispersion(&self, samples: &[f64], kind: DispersionKind) -> Option<f64>;
    fn estimate(&self, samples: &[f64], kind: EstimatorKind) -> Option<f64>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    Axis,
    Verification,
    Full,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiscardReason {
    Failed(String),
    NonFinite,
    ExceedsMax,
    Other(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SkipReason {
    Budget,
    DeviceUnavailable,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    Dominated,
    Discarded(DiscardReason),
    Skipped(SkipReason),
    Measured {
        time: Duration,
        deviation: f64,
        stable: bool,
        samples: usize,
        uncertainty: f64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReportEntry {
    pub stage: Stage,
    pub combination: Combination,
    pub outcome: Outcome,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrontPoint {
    pub combination: Combination,
    pub time: Duration,
    pub deviation: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BudgetStatus {
    pub limit: Option<Duration>,
    pub elapsed: Duration,
    pub exhausted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Counters {
    pub measured: usize,
    pub stable: usize,
    pub unstable: usize,
    pub dominated: usize,
    pub discarded: usize,
    pub skipped: usize,
    pub attempts: usize,
    pub interruptions: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Summary {
    pub front: Vec<FrontPoint>,
    pub candidate_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalibrationReport {
    pub target: Target,
    pub entries: Vec<ReportEntry>,
    pub product_size: usize,
    pub budget: BudgetStatus,
    pub runtime: Duration,
    pub summary: Summary,
    pub counters: Counters,
    pub log: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CalibrationError {
    Capability,
    Model(String),
    TargetMismatch,
    Backend(String),
}

impl fmt::Display for CalibrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalibrationError::Capability => write!(f, "empty capability set"),
            CalibrationError::Model(r) => write!(f, "invalid model descriptor: {r}"),
            CalibrationError::TargetMismatch => write!(f, "target model id does not match model"),
            CalibrationError::Backend(m) => write!(f, "backend failure: {m}"),
        }
    }
}

impl std::error::Error for CalibrationError {}

pub fn sequence_lengths(max_seq_len: usize, start: usize, factor: usize) -> Vec<usize> {
    if max_seq_len < start {
        return vec![max_seq_len];
    }
    let factor = factor.max(2);
    let mut out = Vec::new();
    let mut v = start.max(1);
    while v <= max_seq_len {
        out.push(v);
        match v.checked_mul(factor) {
            Some(n) => v = n,
            None => break,
        }
    }
    if out.last() != Some(&max_seq_len) {
        out.push(max_seq_len);
    }
    out
}

fn sort_values(set: &std::collections::BTreeSet<Value>) -> Vec<Value> {
    let mut numeric: Vec<(u64, Value)> = Vec::new();
    let mut other: Vec<Value> = Vec::new();
    for v in set {
        let s = v.as_str();
        let parsed = if !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) {
            s.parse::<u64>().ok()
        } else {
            None
        };
        match parsed {
            Some(n) => numeric.push((n, v.clone())),
            None => other.push(v.clone()),
        }
    }
    numeric.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    other.sort_by(|a, b| a.as_str().as_bytes().cmp(b.as_str().as_bytes()));
    numeric
        .into_iter()
        .map(|(_, v)| v)
        .chain(other)
        .collect()
}

fn product(axes: &[(Axis, Vec<Value>)]) -> Vec<Combination> {
    let mut cur = vec![Combination(BTreeMap::new())];
    for (axis, values) in axes {
        let mut next = Vec::with_capacity(cur.len().saturating_mul(values.len()));
        for c in &cur {
            for v in values {
                let mut m = c.0.clone();
                m.insert(axis.clone(), v.clone());
                next.push(Combination(m));
            }
        }
        cur = next;
    }
    cur
}

fn describe(c: &Combination) -> String {
    c.0.iter()
        .map(|(a, v)| format!("{}={}", a.as_str(), v.as_str()))
        .collect::<Vec<_>>()
        .join(",")
}

struct Slot {
    samples: Vec<f64>,
    ratios: Vec<f64>,
    attempts: usize,
    first: Option<Instant>,
    deviation: f64,
    result: Option<Outcome>,
}

impl Slot {
    fn new() -> Self {
        Slot {
            samples: Vec::new(),
            ratios: Vec::new(),
            attempts: 0,
            first: None,
            deviation: 0.0,
            result: None,
        }
    }

    fn alive(&self) -> bool {
        matches!(self.result, None | Some(Outcome::Measured { .. }))
    }
}

struct Runner<'a> {
    backend: &'a dyn CalibrationBackend,
    stats: &'a dyn Statistics,
    cfg: &'a CalibrationConfig,
    seq: Vec<usize>,
    start: Instant,
    exhausted: bool,
    counters: Counters,
    entries: Vec<ReportEntry>,
    log: Vec<String>,
}

impl<'a> Runner<'a> {
    fn out_of_budget(&mut self) -> bool {
        if self.exhausted {
            return true;
        }
        if let Some(limit) = self.cfg.total_budget {
            if self.start.elapsed() >= limit {
                self.exhausted = true;
                self.log.push("total budget exhausted".to_string());
                return true;
            }
        }
        false
    }

    fn estimate(&self, samples: &[f64]) -> Option<f64> {
        self.stats
            .estimate(samples, self.cfg.estimator)
            .filter(|v| v.is_finite() && *v >= 0.0)
    }

    fn is_stable(&self, series: &[f64]) -> bool {
        series.len() >= self.cfg.min_repeats
            && self
                .stats
                .dispersion(series, self.cfg.dispersion)
                .map_or(false, |d| d <= self.cfg.stability_threshold)
    }

    fn at_cap(&self, s: &Slot) -> bool {
        s.attempts >= self.cfg.max_attempts
            || match (self.cfg.max_duration, s.first) {
                (Some(max), Some(first)) => first.elapsed() >= max,
                _ => false,
            }
    }

    fn measured(&self, s: &Slot, stable: bool) -> Outcome {
        if s.samples.is_empty() {
            return Outcome::Skipped(SkipReason::DeviceUnavailable);
        }
        let time = self
            .estimate(&s.samples)
            .and_then(|e| Duration::try_from_secs_f64(e).ok());
        match time {
            Some(time) => Outcome::Measured {
                time,
                deviation: s.deviation,
                stable,
                samples: s.samples.len(),
                uncertainty: self
                    .stats
                    .dispersion(&s.samples, self.cfg.dispersion)
                    .filter(|d| d.is_finite())
                    .unwrap_or(self.cfg.dominance_margin),
            },
            None => Outcome::Discarded(DiscardReason::Other("no samples".to_string())),
        }
    }

    fn pause(&self) {
        std::thread::sleep(self.cfg.backoff);
    }

    fn measure_group(
        &mut self,
        group: &[Combination],
        stage: Stage,
    ) -> Result<Vec<Outcome>, CalibrationError> {
        self.log
            .push(format!("group start: stage={:?} size={}", stage, group.len()));
        let mut slots: Vec<Slot> = group.iter().map(|_| Slot::new()).collect();
        for (c, slot) in group.iter().zip(slots.iter_mut()) {
            self.deviation_phase(c, slot)?;
        }
        self.time_phase(group, &mut slots)?;
        let outcomes: Vec<Outcome> = slots
            .into_iter()
            .map(|s| s.result.unwrap_or(Outcome::Skipped(SkipReason::Budget)))
            .collect();
        for (c, o) in group.iter().zip(&outcomes) {
            self.entries.push(ReportEntry {
                stage,
                combination: c.clone(),
                outcome: o.clone(),
            });
        }
        Ok(outcomes)
    }

    fn deviation_phase(&mut self, c: &Combination, slot: &mut Slot) -> Result<(), CalibrationError> {
        let backend = self.backend;
        let mut tries = 0usize;
        loop {
            if self.out_of_budget() {
                slot.result = Some(Outcome::Skipped(SkipReason::Budget));
                return Ok(());
            }
            tries += 1;
            self.counters.attempts += 1;
            match backend.deviation(c, &self.seq) {
                Ok(v) => {
                    if !v.is_finite() {
                        slot.result = Some(Outcome::Discarded(DiscardReason::NonFinite));
                    } else if v > self.cfg.max_deviation {
                        slot.result = Some(Outcome::Discarded(DiscardReason::ExceedsMax));
                    } else {
                        slot.deviation = v;
                    }
                    return Ok(());
                }
                Err(ProbeError::Failed(m)) => {
                    slot.result = Some(Outcome::Discarded(DiscardReason::Failed(m)));
                    return Ok(());
                }
                Err(ProbeError::Rejected(m)) => {
                    slot.result = Some(Outcome::Discarded(DiscardReason::Other(m)));
                    return Ok(());
                }
                Err(ProbeError::Unavailable) => {
                    slot.result = Some(Outcome::Skipped(SkipReason::DeviceUnavailable));
                    return Ok(());
                }
                Err(ProbeError::Interrupted) => {
                    self.counters.interruptions += 1;
                    if tries >= self.cfg.max_attempts {
                        slot.result = Some(Outcome::Discarded(DiscardReason::Other(
                            "interrupted".to_string(),
                        )));
                        return Ok(());
                    }
                    self.pause();
                }
                Err(ProbeError::Fatal(m)) => return Err(CalibrationError::Backend(m)),
            }
        }
    }

    fn finish_on_budget(&self, slots: &mut [Slot]) {
        for i in 0..slots.len() {
            if slots[i].result.is_none() {
                let outcome = if slots[i].samples.is_empty() {
                    Outcome::Skipped(SkipReason::Budget)
                } else {
                    self.measured(&slots[i], false)
                };
                slots[i].result = Some(outcome);
            }
        }
    }

    fn decide_non_anchor(&self, i: usize, slots: &[Slot]) -> Option<Outcome> {
        let s = &slots[i];
        let min = self.cfg.min_repeats;
        if s.samples.len() >= min {
            if let Some(ei) = self.estimate(&s.samples) {
                let limit = ei * (1.0 - self.cfg.dominance_margin);
                let dominated = slots.iter().enumerate().any(|(j, o)| {
                    j != i
                        && o.alive()
                        && o.samples.len() >= min
                        && o.deviation <= s.deviation
                        && self.estimate(&o.samples).map_or(false, |ej| ej < limit)
                });
                if dominated {
                    return Some(Outcome::Dominated);
                }
            }
        }
        if self.is_stable(&s.ratios) {
            return Some(self.measured(s, true));
        }
        if self.at_cap(s) {
            return Some(self.measured(s, false));
        }
        None
    }

    fn decide_anchor(&self, a: usize, slots: &[Slot]) -> Option<Outcome> {
        let mut any_stable = false;
        let mut any_unstable = false;
        for (j, o) in slots.iter().enumerate() {
            if j == a {
                continue;
            }
            match o.result {
                None => return None,
                Some(Outcome::Measured { stable: true, .. }) => any_stable = true,
                Some(Outcome::Measured { stable: false, .. }) => any_unstable = true,
                _ => {}
            }
        }
        let s = &slots[a];
        if (any_stable && !any_unstable) || self.is_stable(&s.samples) {
            return Some(self.measured(s, true));
        }
        if self.at_cap(s) {
            return Some(self.measured(s, false));
        }
        None
    }

    fn time_phase(
        &mut self,
        group: &[Combination],
        slots: &mut [Slot],
    ) -> Result<(), CalibrationError> {
        let n = group.len();
        let backend = self.backend;
        let cfg = self.cfg;
        let mut anchor = slots.iter().position(|s| s.result.is_none());
        let mut round = 0usize;
        loop {
            let active: Vec<usize> = (0..n).filter(|&i| slots[i].result.is_none()).collect();
            if active.is_empty() {
                return Ok(());
            }
            if anchor.map_or(true, |a| slots[a].result.is_some()) {
                anchor = active.first().copied();
            }
            let order: Vec<usize> = if round % 2 == 0 {
                active
            } else {
                active.into_iter().rev().collect()
            };
            let mut round_t: Vec<Option<f64>> = vec![None; n];
            for &i in &order {
                if slots[i].result.is_some() {
                    continue;
                }
                if self.out_of_budget() {
                    self.finish_on_budget(slots);
                    return Ok(());
                }
                let slot = &mut slots[i];
                slot.attempts += 1;
                slot.first.get_or_insert_with(Instant::now);
                self.counters.attempts += 1;
                let mut lost = false;
                match backend.time(&group[i], &self.seq, cfg.time_window, cfg.chunk_budget) {
                    Ok(d) => {
                        let t = d.as_secs_f64();
                        slot.samples.push(t);
                        round_t[i] = Some(t);
                    }
                    Err(ProbeError::Failed(m)) => {
                        slot.result = Some(Outcome::Discarded(DiscardReason::Failed(m)));
                        lost = true;
                    }
                    Err(ProbeError::Rejected(m)) => {
                        slot.result = Some(Outcome::Discarded(DiscardReason::Other(m)));
                        lost = true;
                    }
                    Err(ProbeError::Interrupted) => {
                        self.counters.interruptions += 1;
                        self.pause();
                    }
                    Err(ProbeError::Unavailable) => {
                        if slot.samples.is_empty() {
                            slot.result = Some(Outcome::Skipped(SkipReason::DeviceUnavailable));
                            lost = true;
                        } else {
                            self.counters.interruptions += 1;
                            self.pause();
                        }
                    }
                    Err(ProbeError::Fatal(m)) => return Err(CalibrationError::Backend(m)),
                }
                if lost && anchor == Some(i) {
                    anchor = slots.iter().position(|s| s.result.is_none());
                    for s in slots.iter_mut() {
                        s.ratios.clear();
                    }
                }
            }
            if let Some(a) = anchor {
                if let Some(ta) = round_t[a] {
                    if ta > 0.0 {
                        for i in 0..n {
                            if i != a && slots[i].result.is_none() {
                                if let Some(t) = round_t[i] {
                                    slots[i].ratios.push(t / ta);
                                }
                            }
                        }
                    }
                }
            }
            for i in 0..n {
                if Some(i) == anchor || slots[i].result.is_some() {
                    continue;
                }
                if let Some(o) = self.decide_non_anchor(i, slots) {
                    slots[i].result = Some(o);
                }
            }
            if let Some(a) = anchor {
                if slots[a].result.is_none() {
                    if let Some(o) = self.decide_anchor(a, slots) {
                        slots[a].result = Some(o);
                    }
                }
            }
            round += 1;
        }
    }
}

fn pareto_winners(points: &[(Value, Duration, f64)]) -> Vec<Value> {
    points
        .iter()
        .filter(|(_, t, d)| {
            !points
                .iter()
                .any(|(_, t2, d2)| t2 <= t && d2 <= d && (t2 < t || d2 < d))
        })
        .map(|(v, _, _)| v.clone())
        .collect()
}

fn build_front(entries: &[ReportEntry], log: &mut Vec<String>) -> (Vec<FrontPoint>, usize) {
    let mut points: Vec<FrontPoint> = Vec::new();
    let mut count = 0usize;
    for e in entries {
        if e.stage == Stage::Axis {
            continue;
        }
        count += 1;
        if let Outcome::Measured { time, deviation, .. } = &e.outcome {
            points.push(FrontPoint {
                combination: e.combination.clone(),
                time: *time,
                deviation: *deviation,
            });
        }
    }
    points.sort_by(|a, b| {
        a.time
            .cmp(&b.time)
            .then_with(|| a.deviation.total_cmp(&b.deviation))
    });
    let mut front: Vec<FrontPoint> = Vec::new();
    for p in points {
        let keep = front.last().map_or(true, |l| p.deviation < l.deviation);
        log.push(format!(
            "{}: {} time={:?} deviation={}",
            if keep { "front" } else { "excluded" },
            describe(&p.combination),
            p.time,
            p.deviation
        ));
        if keep {
            front.push(p);
        }
    }
    (front, count)
}

pub fn run_calibration(
    target: Target,
    capabilities: &CapabilitySet,
    model: &ModelDescriptor,
    backend: &dyn CalibrationBackend,
    stats: &dyn Statistics,
    config: &CalibrationConfig,
) -> Result<CalibrationReport, CalibrationError> {
    let start = Instant::now();
    let mut cfg = config.clone();
    cfg.min_repeats = cfg.min_repeats.max(3);
    if let Err(ModelConfigError::Incompatible { reason }) = model.validate() {
        return Err(CalibrationError::Model(reason));
    }
    if target.model_id != model.model_id {
        return Err(CalibrationError::TargetMismatch);
    }
    if capabilities.is_empty() {
        return Err(CalibrationError::Capability);
    }

    let all_axes: Vec<(Axis, Vec<Value>)> = capabilities
        .axes()
        .filter_map(|a| {
            capabilities
                .values(a)
                .filter(|v| !v.is_empty())
                .map(|v| (a.clone(), sort_values(v)))
        })
        .collect();
    let product_size = all_axes
        .iter()
        .fold(1usize, |p, (_, v)| p.saturating_mul(v.len()));

    let mut runner = Runner {
        backend,
        stats,
        cfg: &cfg,
        seq: sequence_lengths(model.max_seq_len, cfg.seq_start, cfg.seq_factor),
        start,
        exhausted: false,
        counters: Counters::default(),
        entries: Vec::new(),
        log: Vec::new(),
    };

    if cfg.full_product {
        let all = product(&all_axes);
        runner.measure_group(&all, Stage::Full)?;
    } else {
        let (sep, core): (Vec<_>, Vec<_>) = all_axes
            .iter()
            .cloned()
            .partition(|(a, _)| cfg.separable_axes.contains(a));
        let mut candidates: Vec<Combination> = Vec::new();
        let mut seen: HashSet<Combination> = HashSet::new();

        for core_combo in product(&core) {
            let mut base = core_combo.clone();
            for (a, vals) in &sep {
                base.0.insert(a.clone(), vals[0].clone());
            }
            let mut group = vec![base.clone()];
            for (a, vals) in &sep {
                for v in vals {
                    let mut c = base.clone();
                    c.0.insert(a.clone(), v.clone());
                    if !group.contains(&c) {
                        group.push(c);
                    }
                }
            }
            if runner.out_of_budget() {
                runner.measure_group(&group, Stage::Axis)?;
                continue;
            }
            let outcomes = runner.measure_group(&group, Stage::Axis)?;

            let mut winners: Vec<(Axis, Vec<Value>)> = Vec::new();
            let mut missing = false;
            for (a, vals) in &sep {
                let won = if vals.len() == 1 {
                    vals.clone()
                } else {
                    let mut points = Vec::new();
                    for v in vals {
                        let mut c = base.clone();
                        c.0.insert(a.clone(), v.clone());
                        if let Some(pos) = group.iter().position(|g| *g == c) {
                            if let Outcome::Measured { time, deviation, .. } = &outcomes[pos] {
                                points.push((v.clone(), *time, *deviation));
                            }
                        }
                    }
                    pareto_winners(&points)
                };
                if won.is_empty() {
                    missing = true;
                    break;
                }
                winners.push((a.clone(), won));
            }
            if missing {
                runner.log.push(format!(
                    "core combination skipped, axis without winner: {}",
                    describe(&core_combo)
                ));
                continue;
            }
            let mut expanded = vec![core_combo.clone()];
            for (a, won) in &winners {
                let mut next = Vec::new();
                for c in &expanded {
                    for v in won {
                        let mut m = c.0.clone();
                        m.insert(a.clone(), v.clone());
                        next.push(Combination(m));
                    }
                }
                expanded = next;
            }
            for c in expanded {
                if seen.insert(c.clone()) {
                    candidates.push(c);
                }
            }
        }
        if !candidates.is_empty() {
            runner.measure_group(&candidates, Stage::Verification)?;
        }
    }

    let (front, candidate_count) = build_front(&runner.entries, &mut runner.log);
    let mut counters = runner.counters.clone();
    for e in &runner.entries {
        match &e.outcome {
            Outcome::Measured { stable, .. } => {
                counters.measured += 1;
                if *stable {
                    counters.stable += 1;
                } else {
                    counters.unstable += 1;
                }
            }
            Outcome::Dominated => counters.dominated += 1,
            Outcome::Discarded(_) => counters.discarded += 1,
            Outcome::Skipped(_) => counters.skipped += 1,
        }
    }
    let exhausted = runner.out_of_budget();
    let elapsed = start.elapsed();
    Ok(CalibrationReport {
        target,
        entries: runner.entries,
        product_size,
        budget: BudgetStatus {
            limit: cfg.total_budget,
            elapsed,
            exhausted,
        },
        runtime: elapsed,
        summary: Summary {
            front,
            candidate_count,
        },
        counters,
        log: runner.log,
    })
}

const _: fn() = || {
    fn assert_send_sync<T: Send + Sync + 'static>() {}
    assert_send_sync::<Target>();
    assert_send_sync::<Combination>();
    assert_send_sync::<CalibrationConfig>();
    assert_send_sync::<CalibrationReport>();
    assert_send_sync::<CalibrationError>();
    assert_send_sync::<ProbeError>();
};

#[cfg(test)]
#[path = "test/calibration_run.rs"]
mod tests;
