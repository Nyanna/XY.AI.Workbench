//! Real CUDA measurement of the scoring path (`cosine_dot`).
//!
//! Time comes from CUDA events on the stream, so launch overhead is excluded. Each timing
//! sample flushes L2 first and covers a window of back-to-back launches, split into groups that
//! stay within the slice budget because a running kernel cannot be preempted. Only the axes
//! this workload reacts to are read from a combination: `batch` (queries per launch) and the
//! document type from `scoring` (falling back to `operands`).
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::Arc;
use std::time::Duration;
use cudarc::driver::sys::CUevent_flags;
use cudarc::driver::{CudaContext, CudaFunction, CudaSlice, CudaStream};
use half::f16;
use crate::resident::error::PrecisionError;
use crate::resident::kernel::cuda::{launch_cosine, CudaKernels, COSINE_DOT};
use crate::resident::memory::DType;
use crate::resident::subengine::DeviceHandle;
use super::known::{FP16, FP32, OPERANDS, SCORING};
use super::measure::{Cfg, DeviationReport, Measure, Sample, WindowTuning, YieldCheck};
use super::workload::{ScoringWorkload, WorkloadSpec};
fn incompatible(e: impl Display) -> PrecisionError {
    PrecisionError::Incompatible {
        reason: format!("cuda measurement: {e}"),
    }
}
#[derive(Clone, Copy)]
struct Plan {
    iters: u32,
    group: u32,
}
pub struct CudaScoringMeasure {
    stream: Arc<CudaStream>,
    kernels: CudaKernels,
    load: ScoringWorkload,
    tuning: WindowTuning,
    yield_check: Arc<dyn YieldCheck>,
    docs32: CudaSlice<f32>,
    docs16: CudaSlice<u16>,
    queries: CudaSlice<f32>,
    out: CudaSlice<f32>,
    flush: CudaSlice<u8>,
    plans: HashMap<(bool, usize), Plan>,
}
impl CudaScoringMeasure {
    /// L2 of the target generation is a few MiB; this exceeds it by a wide margin.
    const FLUSH_BYTES: usize = 64 << 20;
    const WARMUP_LAUNCHES: u32 = 16;
    /// Long enough for the SM clock to leave its idle state (observed 1185 -> 2145 MHz).
    const WARMUP_MS: u64 = 100;
    const MAX_ITERS: u32 = 10_000;
    pub fn new(
        device: &DeviceHandle,
        spec: WorkloadSpec,
        tuning: WindowTuning,
        yield_check: Arc<dyn YieldCheck>,
    ) -> Result<Self, PrecisionError> {
        let ctx: &Arc<CudaContext> = device
            .cuda_context()
            .ok_or_else(|| {
                PrecisionError::Capability {
                    reason: "device has no cuda context".into(),
                }
            })?;
        let stream = ctx.new_stream().map_err(incompatible)?;
        let kernels = CudaKernels::new(ctx).map_err(incompatible)?;
        let load = ScoringWorkload::generate(spec);
        let bits: Vec<u16> = load
            .docs()
            .iter()
            .map(|&v| f16::from_f32(v).to_bits())
            .collect();
        let (docs, max_q) = (load.spec().docs, load.spec().max_queries);
        Ok(Self {
            docs32: stream.clone_htod(load.docs()).map_err(incompatible)?,
            docs16: stream.clone_htod(&bits).map_err(incompatible)?,
            queries: stream.clone_htod(load.queries(max_q)).map_err(incompatible)?,
            out: stream.alloc_zeros::<f32>(docs * max_q).map_err(incompatible)?,
            flush: stream.alloc_zeros::<u8>(Self::FLUSH_BYTES).map_err(incompatible)?,
            stream,
            kernels,
            load,
            tuning,
            yield_check,
            plans: HashMap::new(),
        })
    }
    fn dtype(cfg: &Cfg) -> Result<DType, PrecisionError> {
        for axis in [&*SCORING, &*OPERANDS] {
            if let Some(v) = cfg.combo.get(axis) {
                return if v == &*FP32 {
                    Ok(DType::F32)
                } else if v == &*FP16 {
                    Ok(DType::F16)
                } else {
                    Err(incompatible(format!("no kernel for {axis}={v}")))
                };
            }
        }
        Ok(DType::F32)
    }
    fn function(&self, dtype: DType) -> Result<CudaFunction, PrecisionError> {
        self.kernels
            .function(COSINE_DOT, dtype, self.load.spec().dim as u32)
            .map_err(incompatible)
    }
    fn launch(
        &mut self,
        f: &CudaFunction,
        dtype: DType,
        nq: usize,
    ) -> Result<(), PrecisionError> {
        let n = self.load.spec().docs as u32;
        match dtype {
            DType::F16 => {
                launch_cosine(
                    &self.stream,
                    f,
                    &self.docs16,
                    &self.queries,
                    &mut self.out,
                    n,
                    nq as u32,
                )
            }
            _ => {
                launch_cosine(
                    &self.stream,
                    f,
                    &self.docs32,
                    &self.queries,
                    &mut self.out,
                    n,
                    nq as u32,
                )
            }
        }
            .map_err(incompatible)
    }
    fn timed(
        &mut self,
        f: &CudaFunction,
        dtype: DType,
        nq: usize,
        launches: u32,
    ) -> Result<f64, PrecisionError> {
        let flags = Some(CUevent_flags::CU_EVENT_DEFAULT);
        let start = self.stream.record_event(flags).map_err(incompatible)?;
        for _ in 0..launches {
            self.launch(f, dtype, nq)?;
        }
        let end = self.stream.record_event(flags).map_err(incompatible)?;
        start.elapsed_ms(&end).map(f64::from).map_err(incompatible)
    }
    /// Warms up, then sizes the window and the launch groups from one timed launch.
    fn plan(
        &mut self,
        f: &CudaFunction,
        dtype: DType,
        nq: usize,
    ) -> Result<Plan, PrecisionError> {
        let warm = std::time::Instant::now();
        while warm.elapsed() < Duration::from_millis(Self::WARMUP_MS) {
            for _ in 0..Self::WARMUP_LAUNCHES {
                self.launch(f, dtype, nq)?;
            }
            self.stream.synchronize().map_err(incompatible)?;
        }
        let one_ms = self.timed(f, dtype, nq, 1)?.max(1e-3);
        let iters = (self.tuning.window_ms / one_ms)
            .ceil()
            .clamp(1.0, f64::from(Self::MAX_ITERS));
        let group = (self.tuning.slice_budget_ms / one_ms).floor().clamp(1.0, iters);
        Ok(Plan {
            iters: iters as u32,
            group: group as u32,
        })
    }
}
impl Measure for CudaScoringMeasure {
    fn deviation(&mut self, cfg: &Cfg) -> Result<DeviationReport, PrecisionError> {
        let dtype = Self::dtype(cfg)?;
        let nq = self.load.query_count(cfg)?;
        let f = self.function(dtype)?;
        self.launch(&f, dtype, nq)?;
        self.stream.synchronize().map_err(incompatible)?;
        let all = self.stream.clone_dtoh(&self.out).map_err(incompatible)?;
        let n = self.load.spec().docs;
        Ok(self.load.judge(&all[..nq * n], nq))
    }
    fn sample(&mut self, cfg: &Cfg) -> Result<Sample, PrecisionError> {
        let dtype = Self::dtype(cfg)?;
        let nq = self.load.query_count(cfg)?;
        let f = self.function(dtype)?;
        let key = (dtype == DType::F16, nq);
        let plan = match self.plans.get(&key) {
            Some(p) => *p,
            None => {
                let p = self.plan(&f, dtype, nq)?;
                self.plans.insert(key, p);
                p
            }
        };
        self.stream.memset_zeros(&mut self.flush).map_err(incompatible)?;
        let (mut done, mut total_ms) = (0u32, 0f64);
        while done < plan.iters {
            let group = plan.group.min(plan.iters - done);
            total_ms += self.timed(&f, dtype, nq, group)?;
            done += group;
            if done < plan.iters && self.yield_check.should_yield() {
                return Ok(Sample::Interrupted);
            }
        }
        let per_item = total_ms / 1e3 / f64::from(plan.iters) / nq as f64;
        Ok(Sample::Time(Duration::from_secs_f64(per_item)))
    }
}
#[cfg(test)]
#[path = "test/measure_cuda.rs"]
mod tests;
