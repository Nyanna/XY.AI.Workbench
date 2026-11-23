use super::*;
use std::any::Any;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

use super::super::bench_workload::{BenchError, ValidityKind, WorkloadConfig};
use super::super::{Clock, NeverPending, PendingFlag, PendingTasks};
use crate::resident::l130::l120::l110::l100::l090::a::gpu_kernel::{
    register_gpu_kernels, GpuKernelError, GpuLauncher, LaunchConfig,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::kernel_register::{
    CodeImage, CompileRequest, CompilerError, KernelCompiler, KernelFunction, KernelRegister,
    LoadFailure, ModuleLoader,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::library_adapter::library_kernel_source;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    GpuEventBackend, LaneConfig, LaneKind, StreamBackend,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::target_env::ComputeCapability;

#[derive(Debug)]
struct Func(String);

impl KernelFunction for Func {
    fn symbol(&self) -> &str {
        &self.0
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

struct NullCompiler;

impl KernelCompiler for NullCompiler {
    fn compile(&self, req: &CompileRequest) -> Result<CodeImage, CompilerError> {
        Ok(CodeImage { kind: req.kind, bytes: vec![1] })
    }
}

struct MockLoader;

impl ModuleLoader for MockLoader {
    fn device_capability(&self) -> Option<ComputeCapability> {
        Some(ComputeCapability::new(7, 5))
    }

    fn load(&self, _image: &CodeImage, symbol: &str) -> Result<Arc<dyn KernelFunction>, LoadFailure> {
        Ok(Arc::new(Func(symbol.to_string())))
    }
}

#[derive(Debug)]
struct MockEvent;

impl GpuEventBackend for MockEvent {
    fn query(&self) -> Result<bool, String> {
        Ok(true)
    }

    fn synchronize(&self) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug)]
struct MockStream;

impl StreamBackend for MockStream {
    fn record_event(&self) -> Result<Box<dyn GpuEventBackend>, String> {
        Ok(Box::new(MockEvent))
    }
}

#[derive(Default)]
struct MockLauncher {
    launches: AtomicUsize,
    fail: Mutex<Option<String>>,
}

impl GpuLauncher for MockLauncher {
    fn launch(
        &self,
        _function: &Arc<dyn KernelFunction>,
        _lane: &Lane,
        _cfg: &LaunchConfig,
        _params: &[u8],
    ) -> Result<(), String> {
        if let Some(cause) = self.fail.lock().unwrap().clone() {
            return Err(cause);
        }
        self.launches.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

fn done_event() -> Event {
    let (event, control) = Event::mock();
    control.complete();
    event
}

struct MockRig {
    launcher: Arc<MockLauncher>,
    reference: Workload,
    ns_per_slot: f64,
    last_slots: Mutex<u32>,
    calls: Mutex<Vec<(DType, usize, Range<u32>)>>,
    flushes: AtomicUsize,
    acks: AtomicUsize,
    marks: AtomicU64,
    ack_nothing: AtomicBool,
    scores: Mutex<Option<Result<Vec<f32>, String>>>,
    fail_params: Mutex<Option<String>>,
    fail_flush: Mutex<Option<String>>,
}

impl GpuRig for MockRig {
    fn kernel_params(
        &self,
        dtype: DType,
        n_queries: usize,
        slots: Range<u32>,
    ) -> Result<KernelParams, String> {
        if let Some(cause) = self.fail_params.lock().unwrap().clone() {
            return Err(cause);
        }
        *self.last_slots.lock().unwrap() = slots.end - slots.start;
        self.calls.lock().unwrap().push((dtype, n_queries, slots.clone()));
        Ok(KernelParams {
            slots_addr: 0x1000,
            segments_addr: 0x2000,
            query_addr: 0x3000,
            out_addr: 0x4000,
            completed_addr: 0x5000,
            block_ctr_addr: 0x6000,
            ring_cap: 64,
            head: slots.start,
            tail: slots.end,
            segment_count: 1,
            query_len: n_queries as u32,
            _reserved: 0,
        })
    }

    fn ack_completed(&self) -> Result<u64, String> {
        self.acks.fetch_add(1, Ordering::SeqCst);
        if self.ack_nothing.load(Ordering::SeqCst) {
            return Ok(0);
        }
        Ok(self.launcher.launches.load(Ordering::SeqCst) as u64)
    }

    fn flush_cache(&self, _lane: &mut Lane, bytes: u64) -> Result<Event, String> {
        assert_eq!(bytes, GPU_FLUSH_BYTES);
        if let Some(cause) = self.fail_flush.lock().unwrap().clone() {
            return Err(cause);
        }
        self.flushes.fetch_add(1, Ordering::SeqCst);
        Ok(done_event())
    }

    fn mark(&self, _lane: &Lane) -> Result<TimingMark, String> {
        Ok(TimingMark { event: done_event(), id: self.marks.fetch_add(1, Ordering::SeqCst) })
    }

    fn elapsed_ns(&self, start: &TimingMark, end: &TimingMark) -> Result<f64, String> {
        assert!(end.id > start.id);
        Ok(self.ns_per_slot * f64::from(*self.last_slots.lock().unwrap()))
    }

    fn read_scores(&self, n_queries: usize) -> Result<Vec<f32>, String> {
        if let Some(scores) = self.scores.lock().unwrap().clone() {
            return scores;
        }
        self.reference.reference_scores(n_queries).map_err(|e| e.to_string())
    }
}

/// Advances by a fixed step on every call.
struct StepClock {
    now: AtomicU64,
    step: u64,
}

impl Clock for StepClock {
    fn now_ns(&self) -> u64 {
        self.now.fetch_add(self.step, Ordering::SeqCst)
    }
}

/// Reports `true` from the given call on (1-based).
struct PendingFrom {
    calls: AtomicUsize,
    from: usize,
}

impl PendingTasks for PendingFrom {
    fn productive_waiting(&self) -> bool {
        self.calls.fetch_add(1, Ordering::SeqCst) + 1 >= self.from
    }
}

struct Fx {
    gpu: GpuMeasurer<8>,
    rig: Arc<MockRig>,
    launcher: Arc<MockLauncher>,
}

impl Fx {
    fn launches(&self) -> usize {
        self.launcher.launches.load(Ordering::SeqCst)
    }

    fn flushes(&self) -> usize {
        self.rig.flushes.load(Ordering::SeqCst)
    }

    fn plans(&self) -> Vec<Plan> {
        lock(&self.gpu.state).plans.values().copied().collect()
    }

    fn warmed(&self) -> bool {
        lock(&self.gpu.state).warmed
    }
}

fn wcfg() -> WorkloadConfig {
    WorkloadConfig { seed: 11, docs: 50, dim: 8, max_queries: 6, top_k: 3 }
}

fn lane() -> Lane {
    Lane::new(
        LaneConfig::preset(LaneKind::Query, DeviceHandle::Gpu(0)),
        Some(Box::new(MockStream)),
    )
}

fn kernels(launcher: &Arc<MockLauncher>) -> Arc<GpuKernels<8>> {
    let register = Arc::new(KernelRegister::new(
        DeviceHandle::Gpu(0),
        Arc::new(NullCompiler),
        Arc::new(MockLoader),
        library_kernel_source(),
    ));
    register_gpu_kernels(&register).unwrap();
    Arc::new(GpuKernels::<8>::new(register, launcher.clone()))
}

fn rig(launcher: &Arc<MockLauncher>, ns_per_slot: f64) -> Arc<MockRig> {
    Arc::new(MockRig {
        launcher: launcher.clone(),
        reference: Workload::build(wcfg()).unwrap(),
        ns_per_slot,
        last_slots: Mutex::new(0),
        calls: Mutex::new(Vec::new()),
        flushes: AtomicUsize::new(0),
        acks: AtomicUsize::new(0),
        marks: AtomicU64::new(0),
        ack_nothing: AtomicBool::new(false),
        scores: Mutex::new(None),
        fail_params: Mutex::new(None),
        fail_flush: Mutex::new(None),
    })
}

/// Each clock call advances 10 ms, so the 100 ms warm-up takes 10 launches.
fn stepping() -> Arc<dyn Clock> {
    Arc::new(StepClock { now: AtomicU64::new(0), step: 10_000_000 })
}

fn fixture_with(ns_per_slot: f64, pending: Arc<dyn PendingTasks>) -> Fx {
    let launcher = Arc::new(MockLauncher::default());
    let rig = rig(&launcher, ns_per_slot);
    let gpu = GpuMeasurer::<8>::new(
        Workload::build(wcfg()).unwrap(),
        kernels(&launcher),
        rig.clone(),
        lane(),
        MeasureConfig::new(1e-3),
        Hooks { clock: stepping(), pending },
    )
    .unwrap();
    Fx { gpu, rig, launcher }
}

/// One pass takes 1 ms: a single launch per repetition, 10 repetitions, 5 per group.
fn fixture() -> Fx {
    fixture_with(20_000.0, Arc::new(NeverPending))
}

fn combo(batch: &str) -> Combination {
    Combination::new().with(axes::batch_size(), batch).unwrap()
}

const WARMUP_LAUNCHES: usize = 10;

#[test]
fn construction_checks_lane_dimension_and_config() {
    let launcher = Arc::new(MockLauncher::default());
    let build = |workload: Workload, lane: Lane, cfg: MeasureConfig| {
        GpuMeasurer::<8>::new(
            workload,
            kernels(&launcher),
            rig(&launcher, 1.0),
            lane,
            cfg,
            Hooks::default(),
        )
    };
    let cpu_lane = Lane::new(
        LaneConfig::preset(LaneKind::Query, DeviceHandle::Cpu),
        Some(Box::new(MockStream)),
    );
    let streamless = Lane::new(LaneConfig::preset(LaneKind::Query, DeviceHandle::Gpu(0)), None);
    let wl = || Workload::build(wcfg()).unwrap();
    assert!(matches!(
        build(wl(), cpu_lane, MeasureConfig::new(0.1)),
        Err(MeasureError::Capability(_))
    ));
    assert!(matches!(
        build(wl(), streamless, MeasureConfig::new(0.1)),
        Err(MeasureError::Capability(_))
    ));
    let other_dim = Workload::build(WorkloadConfig { dim: 16, ..wcfg() }).unwrap();
    assert!(matches!(
        build(other_dim, lane(), MeasureConfig::new(0.1)),
        Err(MeasureError::Config(_))
    ));
    let mut cfg = MeasureConfig::new(0.1);
    cfg.window_ns = 0;
    assert!(matches!(build(wl(), lane(), cfg), Err(MeasureError::Config(_))));
    let mut cfg = MeasureConfig::new(0.1);
    cfg.chunk_budget_ns = 0;
    assert!(matches!(build(wl(), lane(), cfg), Err(MeasureError::Config(_))));
    assert!(matches!(
        build(wl(), lane(), MeasureConfig::new(f64::NAN)),
        Err(MeasureError::Config(_))
    ));
    assert!(build(wl(), lane(), MeasureConfig::new(0.1)).is_ok());
}

#[test]
fn plan_follows_the_pass_time() {
    let plan = |t| Plan::from_pass(t, 50, 10_000_000, 5_000_000);
    assert_eq!(plan(1_000_000.0), Plan { reps: 10, slots_per_launch: 50, group_reps: 5 });
    assert_eq!(plan(5_000_000.0), Plan { reps: 2, slots_per_launch: 50, group_reps: 1 });
    assert_eq!(plan(20_000_000.0), Plan { reps: 1, slots_per_launch: 12, group_reps: 1 });
    assert_eq!(plan(1e12), Plan { reps: 1, slots_per_launch: 1, group_reps: 1 });
    assert_eq!(plan(0.0).reps, MAX_REPS);
    assert_eq!(plan(1e-9).reps, MAX_REPS);
}

#[test]
fn precision_resolution() {
    let with = |axis: Axis, value: Value| combo("2").with(axis, value).unwrap();
    assert_eq!(gpu_dtype(&combo("2")), Ok(DType::F32));
    assert_eq!(gpu_dtype(&with(axes::scoring(), values::fp16())), Ok(DType::F16));
    assert_eq!(gpu_dtype(&with(axes::operands(), values::fp16())), Ok(DType::F16));
    assert_eq!(gpu_dtype(&with(axes::scoring(), values::fp32())), Ok(DType::F32));
    let both_same = with(axes::scoring(), values::fp16()).with(axes::operands(), values::fp16()).unwrap();
    assert_eq!(gpu_dtype(&both_same), Ok(DType::F16));
    let differ = with(axes::scoring(), values::fp16()).with(axes::operands(), values::fp32()).unwrap();
    assert!(gpu_dtype(&differ).unwrap_err().contains("disagree"));
    let unsupported = with(axes::scoring(), Value::new("bf16"));
    assert!(gpu_dtype(&unsupported).unwrap_err().contains("scoring"));
    let int8 = with(axes::operands(), values::int8());
    assert!(gpu_dtype(&int8).unwrap_err().contains("operands"));
}

#[test]
fn probe_scales_to_the_window_with_one_flush_per_probe() {
    let fx = fixture();
    let probe = fx.gpu.time_probe(&combo("4")).unwrap();
    assert_eq!(probe, TimeProbe::Time(1_000_000.0 / 4.0));
    // warm-up, one calibration pass (not counted) and ten repetitions
    assert_eq!(fx.launches(), WARMUP_LAUNCHES + 1 + 10);
    assert_eq!(fx.flushes(), 1);
    assert_eq!(fx.plans(), vec![Plan { reps: 10, slots_per_launch: 50, group_reps: 5 }]);

    let again = fx.gpu.time_probe(&combo("4")).unwrap();
    assert_eq!(again, probe);
    // no new warm-up and no new calibration
    assert_eq!(fx.launches(), WARMUP_LAUNCHES + 1 + 10 + 10);
    assert_eq!(fx.flushes(), 2);
    assert_eq!(fx.plans().len(), 1);
}

#[test]
fn plans_are_kept_per_combination_and_query_count() {
    let fx = fixture();
    fx.gpu.time_probe(&combo("2")).unwrap();
    fx.gpu.time_probe(&combo("3")).unwrap();
    let other = combo("2").with(axes::tile_size(), "64").unwrap();
    fx.gpu.time_probe(&other).unwrap();
    assert_eq!(fx.plans().len(), 3);
    // the warm-up ran once for the instance
    assert_eq!(fx.launches(), WARMUP_LAUNCHES + 3 * 11);
}

#[test]
fn long_passes_are_split_into_gapless_slot_ranges() {
    // one pass takes 20 ms, the chunk budget is 5 ms
    let fx = fixture_with(400_000.0, Arc::new(NeverPending));
    let probe = fx.gpu.time_probe(&combo("2")).unwrap();
    assert_eq!(probe, TimeProbe::Time(20_000_000.0 / 2.0));
    assert_eq!(fx.plans(), vec![Plan { reps: 1, slots_per_launch: 12, group_reps: 1 }]);
    let calls = fx.rig.calls.lock().unwrap().clone();
    let measured: Vec<Range<u32>> = calls[calls.len() - 5..].iter().map(|c| c.2.clone()).collect();
    assert_eq!(measured, vec![0..12, 12..24, 24..36, 36..48, 48..50]);
    assert_eq!(calls[calls.len() - 6].2, 0..50);
    assert!(calls.iter().all(|c| c.0 == DType::F32 && c.1 == 2));
}

#[test]
fn launches_follow_the_ack_rule() {
    let fx = fixture();
    fx.gpu.time_probe(&combo("2")).unwrap();
    // no acknowledgement before the very first launch
    assert_eq!(fx.rig.acks.load(Ordering::SeqCst), fx.launches() - 1);
}

#[test]
fn an_ack_that_is_behind_is_reported() {
    let fx = fixture();
    fx.rig.ack_nothing.store(true, Ordering::SeqCst);
    assert!(matches!(
        fx.gpu.time_probe(&combo("2")),
        Err(MeasureError::Gpu(GpuKernelError::AckBehind { required: 1, acked: 0 }))
    ));
    assert!(!fx.warmed());
    match fx.gpu.deviation(&combo("2")).unwrap() {
        DeviationOutcome::Incompatible { validity } => {
            assert_eq!(validity.kind, ValidityKind::ExecutionFailed);
            assert!(validity.text.contains("not acknowledged"));
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn precision_selects_the_document_dtype() {
    let fx = fixture();
    let fp16 = combo("2").with(axes::scoring(), values::fp16()).unwrap();
    assert!(matches!(fx.gpu.deviation(&fp16).unwrap(), DeviationOutcome::Measured { .. }));
    assert!(fx.rig.calls.lock().unwrap().iter().all(|c| c.0 == DType::F16));
    fx.rig.calls.lock().unwrap().clear();
    fx.gpu.time_probe(&combo("2").with(axes::operands(), values::fp32()).unwrap()).unwrap();
    assert!(fx.rig.calls.lock().unwrap().iter().all(|c| c.0 == DType::F32));
}

#[test]
fn incompatible_combinations_touch_no_device() {
    let fx = fixture();
    let cases = [
        combo("2").with(axes::scoring(), Value::new("bf16")).unwrap(),
        combo("2")
            .with(axes::scoring(), values::fp16())
            .unwrap()
            .with(axes::operands(), values::fp32())
            .unwrap(),
        combo("0"),
        combo("7"),
        combo("x"),
        combo("2").with_seq_lens(vec![0]),
    ];
    for c in &cases {
        match fx.gpu.deviation(c).unwrap() {
            DeviationOutcome::Incompatible { validity } => {
                assert_eq!(validity.kind, ValidityKind::Degenerate);
                assert!(!validity.text.is_empty());
            }
            other => panic!("unexpected {other:?} for {c}"),
        }
        assert!(matches!(fx.gpu.time_probe(c), Err(MeasureError::Config(_))), "{c}");
    }
    assert_eq!(fx.launches(), 0);
    assert_eq!(fx.flushes(), 0);
    assert!(fx.rig.calls.lock().unwrap().is_empty());
    assert!(!fx.warmed());
}

#[test]
fn deviation_of_matching_scores_is_measured() {
    let fx = fixture();
    match fx.gpu.deviation(&combo("3")).unwrap() {
        DeviationOutcome::Measured { deviation, validity } => {
            assert_eq!(deviation, 0.0);
            assert_eq!(validity.kind, ValidityKind::Valid);
        }
        other => panic!("unexpected {other:?}"),
    }
    // warm-up plus one launch over all documents; the warm-up is not repeated
    assert_eq!(fx.launches(), WARMUP_LAUNCHES + 1);
    assert!(fx.warmed());
    fx.gpu.deviation(&combo("3")).unwrap();
    assert_eq!(fx.launches(), WARMUP_LAUNCHES + 2);
    assert_eq!(fx.flushes(), 0);
}

#[test]
fn deviation_judges_the_read_back_scores() {
    let fx = fixture();
    let mut reversed = fx.rig.reference.reference_scores(2).unwrap();
    reversed.reverse();
    *fx.rig.scores.lock().unwrap() = Some(Ok(reversed));
    match fx.gpu.deviation(&combo("2")).unwrap() {
        DeviationOutcome::Incompatible { validity } => {
            assert_eq!(validity.kind, ValidityKind::ExceedsMax);
        }
        other => panic!("unexpected {other:?}"),
    }
    *fx.rig.scores.lock().unwrap() = Some(Ok(vec![1.0; 3]));
    match fx.gpu.deviation(&combo("2")).unwrap() {
        DeviationOutcome::Incompatible { validity } => {
            assert_eq!(validity.kind, ValidityKind::Degenerate);
        }
        other => panic!("unexpected {other:?}"),
    }
    *fx.rig.scores.lock().unwrap() = Some(Err("device lost".to_string()));
    match fx.gpu.deviation(&combo("2")).unwrap() {
        DeviationOutcome::Incompatible { validity } => {
            assert_eq!(validity.kind, ValidityKind::ExecutionFailed);
            assert_eq!(validity.text, "device lost");
        }
        other => panic!("unexpected {other:?}"),
    }
}

#[test]
fn execution_failures_are_outcomes_in_deviation_and_errors_in_probes() {
    let fx = fixture();
    *fx.launcher.fail.lock().unwrap() = Some("boom".to_string());
    match fx.gpu.deviation(&combo("2")).unwrap() {
        DeviationOutcome::Incompatible { validity } => {
            assert_eq!(validity.kind, ValidityKind::ExecutionFailed);
            assert!(validity.text.contains("boom"));
        }
        other => panic!("unexpected {other:?}"),
    }
    assert!(matches!(
        fx.gpu.time_probe(&combo("2")),
        Err(MeasureError::Gpu(GpuKernelError::Launch { .. }))
    ));
    *fx.launcher.fail.lock().unwrap() = None;
    *fx.rig.fail_params.lock().unwrap() = Some("no params".to_string());
    assert_eq!(
        fx.gpu.time_probe(&combo("2")),
        Err(MeasureError::Rig("no params".to_string()))
    );
    assert!(!fx.warmed());
    assert!(fx.plans().is_empty());
}

#[test]
fn a_failing_probe_keeps_the_plan() {
    let fx = fixture();
    fx.gpu.time_probe(&combo("2")).unwrap();
    let plans = fx.plans();
    *fx.rig.fail_flush.lock().unwrap() = Some("flush failed".to_string());
    assert_eq!(
        fx.gpu.time_probe(&combo("2")),
        Err(MeasureError::Rig("flush failed".to_string()))
    );
    *fx.rig.fail_flush.lock().unwrap() = None;
    *fx.launcher.fail.lock().unwrap() = Some("boom".to_string());
    assert!(matches!(fx.gpu.time_probe(&combo("2")), Err(MeasureError::Gpu(_))));
    assert_eq!(fx.plans(), plans);
    *fx.launcher.fail.lock().unwrap() = None;
    assert!(matches!(fx.gpu.time_probe(&combo("2")).unwrap(), TimeProbe::Time(_)));
}

#[test]
fn a_failed_calibration_leaves_no_plan() {
    let fx = fixture();
    fx.gpu.deviation(&combo("2")).unwrap();
    *fx.launcher.fail.lock().unwrap() = Some("boom".to_string());
    assert!(matches!(fx.gpu.time_probe(&combo("2")), Err(MeasureError::Gpu(_))));
    assert!(fx.plans().is_empty());
}

#[test]
fn an_interrupted_warm_up_starts_over() {
    let flag = PendingFlag::new();
    let fx = fixture_with(20_000.0, Arc::new(flag.clone()));
    flag.set(true);
    assert_eq!(fx.gpu.time_probe(&combo("2")).unwrap(), TimeProbe::Interrupted);
    assert!(!fx.warmed());
    assert_eq!(fx.launches(), 1);
    assert_eq!(fx.flushes(), 0);
    flag.set(false);
    assert!(matches!(fx.gpu.time_probe(&combo("2")).unwrap(), TimeProbe::Time(_)));
    assert!(fx.warmed());
    assert_eq!(fx.launches(), 1 + WARMUP_LAUNCHES + 1 + 10);
}

#[test]
fn deviation_is_never_interrupted() {
    let flag = PendingFlag::new();
    flag.set(true);
    let fx = fixture_with(20_000.0, Arc::new(flag));
    assert!(matches!(fx.gpu.deviation(&combo("2")).unwrap(), DeviationOutcome::Measured { .. }));
    assert!(fx.warmed());
    assert_eq!(fx.launches(), WARMUP_LAUNCHES + 1);
}

#[test]
fn interruption_is_checked_before_the_first_and_between_groups() {
    let pending = Arc::new(PendingFrom { calls: AtomicUsize::new(0), from: 2 });
    let fx = fixture_with(20_000.0, pending.clone());
    fx.gpu.deviation(&combo("2")).unwrap();
    assert_eq!(pending.calls.load(Ordering::SeqCst), 0);
    let before = fx.launches();
    assert_eq!(fx.gpu.time_probe(&combo("2")).unwrap(), TimeProbe::Interrupted);
    // calibration plus the first group of five repetitions; no check after the last group
    assert_eq!(fx.launches(), before + 1 + 5);
    assert_eq!(pending.calls.load(Ordering::SeqCst), 2);
    assert_eq!(fx.plans().len(), 1);
}

#[test]
fn an_interruption_before_the_first_group_skips_the_flush() {
    let pending = Arc::new(PendingFrom { calls: AtomicUsize::new(0), from: 1 });
    let fx = fixture_with(20_000.0, pending);
    fx.gpu.deviation(&combo("2")).unwrap();
    let before = fx.launches();
    assert_eq!(fx.gpu.time_probe(&combo("2")).unwrap(), TimeProbe::Interrupted);
    assert_eq!(fx.launches(), before + 1);
    assert_eq!(fx.flushes(), 0);
}

#[test]
fn reference_is_cached_per_query_count() {
    let fx = fixture();
    fx.gpu.deviation(&combo("2")).unwrap();
    let first = fx.gpu.references.get(&fx.gpu.workload, 2).unwrap();
    fx.gpu.deviation(&combo("2")).unwrap();
    let second = fx.gpu.references.get(&fx.gpu.workload, 2).unwrap();
    assert!(Arc::ptr_eq(&first, &second));
    assert_eq!(*first, fx.gpu.workload.reference_scores(2).unwrap());
}

#[test]
fn invalid_top_k_is_a_configuration_error() {
    let launcher = Arc::new(MockLauncher::default());
    let mut cfg = MeasureConfig::new(0.1);
    cfg.top_k = 0;
    let gpu = GpuMeasurer::<8>::new(
        Workload::build(wcfg()).unwrap(),
        kernels(&launcher),
        rig(&launcher, 1.0),
        lane(),
        cfg,
        Hooks::default(),
    )
    .unwrap();
    assert!(matches!(
        gpu.deviation(&combo("2")),
        Err(MeasureError::Bench(BenchError::InvalidConfig { .. }))
    ));
}

#[test]
fn parallel_callers_are_serialized() {
    let fx = Arc::new(fixture());
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let fx = Arc::clone(&fx);
            std::thread::spawn(move || fx.gpu.deviation(&combo("2")).unwrap())
        })
        .collect();
    for h in handles {
        assert!(matches!(h.join().unwrap(), DeviationOutcome::Measured { .. }));
    }
    assert_eq!(fx.launches(), WARMUP_LAUNCHES + 4);
}
