use std::collections::HashMap;
use std::ops::Range;
use std::sync::{Arc, Mutex};
use crate::resident::_d::_c::_b::_a::_9::a::gpu_kernel::{GpuKernels, KernelParams};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::capability_model::{
    axes, values, Axis, Value,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::{
    DeviceHandle, Event, Lane,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
use super::bench_workload::{DeviationConfig, Workload};
use super::{
    incompatible6, judge3, lock, preflight, sample_ns, Combination, DeviationOutcome,
    Gate3, Hooks, Measure, MeasureConfig, MeasureError, ReferenceCache, TimeProbe,
    GPU_FLUSH_BYTES, GPU_WARMUP_NS, MAX_REPS, MIN_REPS, MIN_SAMPLE_NS,
};
/// Slots of the first warm-up launch, before the launch rate is known.
const WARMUP_PILOT_SLOTS: usize = 64;
/// Upper bound of warm-up groups, in case the clock never advances.
const MAX_WARMUP_GROUPS: usize = 100_000;
/// A point in time on the stream of a lane.
#[derive(Debug, Clone)]
pub struct TimingMark {
    pub event: Event,
    pub id: u64,
}
/// Implemented by the GPU binding. Holds the documents in F32 and F16 on the device.
pub trait GpuRig: Send + Sync {
    /// Parameters for `dense_dot`: the first `n_queries` workload queries (F32), a slot ring over
    /// `slots`, and an output of `n_queries * slots.len()` floats (index `q * slots.len() + s`).
    fn kernel_params(
        &self,
        dtype: DType,
        n_queries: usize,
        slots: Range<u32>,
    ) -> Result<KernelParams, String>;
    /// Polls the doorbell, closes completed batches and returns the acknowledged batch count.
    fn ack_completed(&self) -> Result<u64, String>;
    /// Enqueues `bytes` of cache-flush work on the lane's stream; the event covers it.
    fn flush_cache(&self, lane: &mut Lane, bytes: u64) -> Result<Event, String>;
    /// Records a time mark on the lane's stream.
    fn mark(&self, lane: &Lane) -> Result<TimingMark, String>;
    /// GPU time between two marks in ns; valid once `end.event` has completed.
    fn elapsed_ns(&self, start: &TimingMark, end: &TimingMark) -> Result<f64, String>;
    /// Scores of the last launch over the full slot range (`q * docs + d`).
    fn read_scores(&self, n_queries: usize) -> Result<Vec<f32>, String>;
}
/// Repetition plan of one combination.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Plan {
    reps: u32,
    slots_per_launch: u32,
    group_reps: u32,
}
impl Plan {
    fn from_pass(
        t_pass_ns: f64,
        docs: u32,
        window_ns: u64,
        chunk_budget_ns: u64,
    ) -> Plan {
        let t = if t_pass_ns.is_finite() {
            t_pass_ns.max(MIN_SAMPLE_NS)
        } else {
            MIN_SAMPLE_NS
        };
        let chunk = chunk_budget_ns as f64;
        let reps = (window_ns as f64 / t)
            .ceil()
            .clamp(f64::from(MIN_REPS), f64::from(MAX_REPS)) as u32;
        let slots_per_launch = if t <= chunk {
            docs
        } else {
            ((f64::from(docs) * chunk / t).floor() as u32).clamp(1, docs)
        };
        let group_reps = ((chunk / t).floor() as u32).clamp(1, MAX_REPS);
        Plan {
            reps,
            slots_per_launch,
            group_reps,
        }
    }
}
struct GpuState3 {
    lane: Lane,
    plans: HashMap<(String, usize), Plan>,
    warmed: bool,
    /// Last event of work that was enqueued but not yet waited for.
    in_flight: Option<Event>,
}
enum Warm {
    Done,
    Interrupted,
}
pub struct GpuMeasurer<const DIM: usize> {
    workload: Workload,
    kernels: Arc<GpuKernels<DIM>>,
    rig: Arc<dyn GpuRig>,
    cfg: MeasureConfig,
    hooks: Hooks,
    docs: u32,
    gate: Gate3,
    state: Mutex<GpuState3>,
    references: ReferenceCache,
}
fn dtype_of(value: &Value) -> Option<DType> {
    if *value == values::fp32() {
        Some(DType::F32)
    } else if *value == values::fp16() {
        Some(DType::F16)
    } else {
        None
    }
}
/// Document dtype from `scoring`, else from `operands`; fp32 when neither is present.
fn gpu_dtype(combo: &Combination) -> Result<DType, String> {
    let check = |axis: &Axis| -> Result<Option<DType>, String> {
        match combo.value(axis) {
            None => Ok(None),
            Some(value) => {
                dtype_of(value)
                    .map(Some)
                    .ok_or_else(|| {
                        format!(
                            "axis {axis}: value {value} is not supported by the gpu scorer (fp32, fp16)"
                        )
                    })
            }
        }
    };
    let scoring = axes::scoring();
    let operands = axes::operands();
    let by_scoring = check(&scoring)?;
    let by_operands = check(&operands)?;
    match (by_scoring, by_operands) {
        (Some(a), Some(b)) if a != b => {
            Err(
                format!(
                    "axes {scoring} and {operands} disagree: {} vs {}", combo.value(&
                    scoring).map_or(String::new(), | v | v.to_string()), combo.value(&
                    operands).map_or(String::new(), | v | v.to_string()),
                ),
            )
        }
        (Some(a), _) | (None, Some(a)) => Ok(a),
        (None, None) => Ok(DType::F32),
    }
}
fn event_error(e: impl std::fmt::Display) -> MeasureError {
    MeasureError::Rig(e.to_string())
}
impl<const DIM: usize> GpuMeasurer<DIM> {
    pub fn new(
        workload: Workload,
        kernels: Arc<GpuKernels<DIM>>,
        rig: Arc<dyn GpuRig>,
        lane: Lane,
        cfg: MeasureConfig,
        hooks: Hooks,
    ) -> Result<Self, MeasureError> {
        if !matches!(lane.device(), DeviceHandle::Gpu(_)) {
            return Err(
                MeasureError::Capability(
                    format!("lane belongs to device {}, not to a gpu", lane.device()),
                ),
            );
        }
        if lane.stream().is_none() {
            return Err(MeasureError::Capability("lane has no stream".to_string()));
        }
        if workload.config().dim != DIM {
            return Err(
                MeasureError::Config(
                    format!(
                        "workload dimension {} differs from kernel dimension {DIM}",
                        workload.config().dim
                    ),
                ),
            );
        }
        cfg.validate_timing()?;
        DeviationConfig::new(cfg.max_deviation)
            .map_err(|e| MeasureError::Config(e.to_string()))?;
        let docs = u32::try_from(workload.config().docs)
            .map_err(|_| MeasureError::Config(
                "document count does not fit in u32".to_string(),
            ))?;
        Ok(GpuMeasurer {
            workload,
            kernels,
            rig,
            cfg,
            hooks,
            docs,
            gate: Gate3::new(),
            state: Mutex::new(GpuState3 {
                lane,
                plans: HashMap::new(),
                warmed: false,
                in_flight: None,
            }),
            references: ReferenceCache::default(),
        })
    }
    /// One `dense_dot` launch. The predecessor is complete before the launch; the launch is
    /// complete when this returns. Returns the event time of the launch if `timed`.
    fn launch(
        &self,
        st: &mut GpuState3,
        dtype: DType,
        params: &KernelParams,
        timed: bool,
    ) -> Result<f64, MeasureError> {
        if let Some(previous) = st.in_flight.take() {
            previous.wait().map_err(event_error)?;
        }
        let acked = if self.kernels.launched() == 0 {
            0
        } else {
            self.rig.ack_completed().map_err(MeasureError::Rig)?
        };
        let start = if timed {
            Some(self.rig.mark(&st.lane).map_err(MeasureError::Rig)?)
        } else {
            None
        };
        let done = self.kernels.dense_dot(&mut st.lane, dtype, params, acked)?;
        let Some(start) = start else {
            st.in_flight = Some(done.clone());
            done.wait().map_err(event_error)?;
            st.in_flight = None;
            return Ok(0.0);
        };
        let end = self.rig.mark(&st.lane).map_err(MeasureError::Rig)?;
        st.in_flight = Some(end.event.clone());
        end.event.wait().map_err(event_error)?;
        st.in_flight = None;
        let ns = self.rig.elapsed_ns(&start, &end).map_err(MeasureError::Rig)?;
        if !ns.is_finite() || ns < 0.0 {
            return Err(MeasureError::Rig(format!("invalid elapsed time {ns} ns")));
        }
        Ok(ns)
    }
    fn timed_launch(
        &self,
        dtype: DType,
        n: usize,
        slots: Range<u32>,
    ) -> Result<f64, MeasureError> {
        let params = self.rig.kernel_params(dtype, n, slots).map_err(MeasureError::Rig)?;
        let mut st = lock(&self.state);
        self.launch(&mut st, dtype, &params, true)
    }
    fn flush(&self) -> Result<(), MeasureError> {
        let mut st = lock(&self.state);
        if let Some(previous) = st.in_flight.take() {
            previous.wait().map_err(event_error)?;
        }
        let event = self
            .rig
            .flush_cache(&mut st.lane, GPU_FLUSH_BYTES)
            .map_err(MeasureError::Rig)?;
        st.in_flight = Some(event.clone());
        event.wait().map_err(event_error)?;
        st.in_flight = None;
        Ok(())
    }
    /// Dense-dot work for at least `GPU_WARMUP_NS` of clock time, in groups of one launch that
    /// fill at most the chunk budget. Runs once per instance.
    fn warm_up(
        &self,
        dtype: DType,
        n: usize,
        interruptible: bool,
    ) -> Result<Warm, MeasureError> {
        if lock(&self.state).warmed {
            return Ok(Warm::Done);
        }
        let docs = self.docs as usize;
        let chunk = self.cfg.chunk_budget_ns as f64;
        let clock = &self.hooks.clock;
        let began = clock.now_ns();
        let mut slots = docs.min(WARMUP_PILOT_SLOTS);
        for _ in 0..MAX_WARMUP_GROUPS {
            let ns = self.timed_launch(dtype, n, 0..slots as u32)?;
            if clock.now_ns().saturating_sub(began) >= GPU_WARMUP_NS {
                break;
            }
            if interruptible && self.hooks.pending.productive_waiting() {
                return Ok(Warm::Interrupted);
            }
            let per_slot = ns / slots as f64;
            slots = ((chunk / per_slot).floor() as usize).clamp(1, docs);
        }
        lock(&self.state).warmed = true;
        Ok(Warm::Done)
    }
    fn plan(&self, key: &(String, usize), dtype: DType) -> Result<Plan, MeasureError> {
        if let Some(plan) = lock(&self.state).plans.get(key).copied() {
            return Ok(plan);
        }
        let t_pass = self.timed_launch(dtype, key.1, 0..self.docs)?;
        let plan = Plan::from_pass(
            t_pass,
            self.docs,
            self.cfg.window_ns,
            self.cfg.chunk_budget_ns,
        );
        lock(&self.state).plans.insert(key.clone(), plan);
        Ok(plan)
    }
    fn execute(&self, dtype: DType, n: usize) -> Result<Vec<f32>, MeasureError> {
        let params = self
            .rig
            .kernel_params(dtype, n, 0..self.docs)
            .map_err(MeasureError::Rig)?;
        {
            let mut st = lock(&self.state);
            self.launch(&mut st, dtype, &params, false)?;
        }
        self.rig.read_scores(n).map_err(MeasureError::Rig)
    }
}
impl<const DIM: usize> Measure for GpuMeasurer<DIM> {
    fn deviation(&self, combo: &Combination) -> Result<DeviationOutcome, MeasureError> {
        let _turn = self.gate.enter();
        let max_queries = self.workload.config().max_queries;
        let (n, dtype) = match preflight(combo, max_queries, gpu_dtype) {
            Ok(checked) => checked,
            Err(text) => return Ok(incompatible6(text)),
        };
        let dev_cfg = self.cfg.deviation_config()?;
        let scores = self
            .warm_up(dtype, n, false)
            .and_then(|_| self.execute(dtype, n))
            .map_err(|e| e.cause_text());
        judge3(
            &self.workload,
            &self.references,
            &dev_cfg,
            n,
            scores.as_deref().map_err(|e| e.clone()),
        )
    }
    fn time_probe(&self, combo: &Combination) -> Result<TimeProbe, MeasureError> {
        let _turn = self.gate.enter();
        let max_queries = self.workload.config().max_queries;
        let (n, dtype) = preflight(combo, max_queries, gpu_dtype)
            .map_err(MeasureError::Config)?;
        if let Warm::Interrupted = self.warm_up(dtype, n, true)? {
            return Ok(TimeProbe::Interrupted);
        }
        let key = (combo.to_text(), n);
        let plan = self.plan(&key, dtype)?;
        if self.hooks.pending.productive_waiting() {
            return Ok(TimeProbe::Interrupted);
        }
        self.flush()?;
        let mut total = 0.0;
        let mut done = 0u32;
        while done < plan.reps {
            if done > 0 && self.hooks.pending.productive_waiting() {
                return Ok(TimeProbe::Interrupted);
            }
            let group = plan.group_reps.min(plan.reps - done);
            for _ in 0..group {
                let mut start = 0u32;
                while start < self.docs {
                    let end = start.saturating_add(plan.slots_per_launch).min(self.docs);
                    total += self.timed_launch(dtype, n, start..end)?;
                    start = end;
                }
            }
            done += group;
        }
        Ok(TimeProbe::Time(sample_ns(total, plan.reps, n)))
    }
}
#[cfg(test)]
#[path = "test/gpu_measurer.rs"]
mod tests;
