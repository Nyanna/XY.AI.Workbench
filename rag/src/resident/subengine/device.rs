//! Device/Lane, execution context, loaded encoder, GPU memory arenas.
use crate::resident::backend::ComputeBackend;
/// Real CUDA backend; the single Candle/CUDA adapter point for the subengine. Construction fails if no compatible GPU is available,
/// which lets `create_backend` fall back to `CpuBackend`.
pub struct CudaBackend;
impl CudaBackend {
    pub fn try_new() -> Result<Self, crate::resident::error::IndexError> {
        Err(crate::resident::error::IndexError::TransferFailed {
            reason: "CUDA backend not yet implemented".to_string(),
        })
    }
}
impl ComputeBackend for CudaBackend {
    fn run_batch(&self, _input: &[f32]) -> Vec<f32> {
        unimplemented!("real CUDA kernel dispatch is not implemented yet")
    }
}
use std::time::{Duration, Instant};
use crate::resident::memory::DeviceKind;
use crate::resident::subengine::{
    CpuLane, DeviceError, ExecContext, LaneClass, LaneConfig,
};
/// Identifies the compute device a subengine runs on; lanes are opened from it.
#[derive(Clone)]
pub struct DeviceHandle {
    kind: DeviceKind,
    ordinal: usize,
    init_cost: Duration,
    #[cfg(feature = "cuda")]
    ctx: Option<std::sync::Arc<cudarc::driver::CudaContext>>,
}
impl std::fmt::Debug for DeviceHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DeviceHandle")
            .field("kind", &self.kind)
            .field("ordinal", &self.ordinal)
            .finish()
    }
}
impl DeviceHandle {
    pub fn cpu() -> Self {
        Self {
            kind: DeviceKind::Cpu,
            ordinal: 0,
            init_cost: Duration::ZERO,
            #[cfg(feature = "cuda")]
            ctx: None,
        }
    }
    /// Retains the primary context of `ordinal`. The first call is expensive (0.2-2 s, more when
    /// the driver is cold), so callers should run it off the latency path.
    #[cfg(feature = "cuda")]
    pub fn cuda(ordinal: usize) -> Result<Self, DeviceError> {
        if std::env::var_os("CUDA_MODULE_LOADING").is_none() {
            std::env::set_var("CUDA_MODULE_LOADING", "LAZY");
        }
        let started = Instant::now();
        let ctx = cudarc::driver::CudaContext::new(ordinal)
            .map_err(|e| DeviceError::Init {
                reason: e.to_string(),
            })?;
        let init_cost = started.elapsed();
        tracing::info!(
            ordinal, init_ms = init_cost.as_millis() as u64, "cuda context retained"
        );
        Ok(Self {
            kind: DeviceKind::Gpu,
            ordinal,
            init_cost,
            ctx: Some(ctx),
        })
    }
    #[cfg(not(feature = "cuda"))]
    pub fn cuda(_ordinal: usize) -> Result<Self, DeviceError> {
        Err(DeviceError::CudaUnavailable)
    }
    pub fn kind(&self) -> DeviceKind {
        self.kind
    }
    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
    /// Time spent creating the context, reported to the lazy-init accounting.
    pub fn init_cost(&self) -> Duration {
        self.init_cost
    }
    /// Forces driver-side lazy work (first allocation, stream creation) so it does not land on the
    /// first request.
    pub fn warmup(&self) -> Result<Duration, DeviceError> {
        let started = Instant::now();
        #[cfg(feature = "cuda")]
        if let Some(ctx) = &self.ctx {
            let init = |e: cudarc::driver::DriverError| DeviceError::Init {
                reason: e.to_string(),
            };
            let stream = ctx.new_stream().map_err(init)?;
            let _probe = stream.alloc_zeros::<u8>(1).map_err(init)?;
            stream.synchronize().map_err(init)?;
        }
        Ok(started.elapsed())
    }
    pub fn open_lane(
        &self,
        class: LaneClass,
        cfg: LaneConfig,
    ) -> Result<Box<dyn ExecContext>, DeviceError> {
        #[cfg(feature = "cuda")]
        if let Some(ctx) = &self.ctx {
            return Ok(Box::new(CudaLane::new(ctx, class, cfg)?));
        }
        Ok(Box::new(CpuLane::new(class, cfg)?))
    }
}
#[cfg(feature = "cuda")]
pub use cuda_lane::CudaLane;
#[cfg(feature = "cuda")]
mod cuda_lane {
    use std::sync::Arc;
    use cudarc::driver::{result, CudaContext, CudaStream};
    use super::*;
    use crate::resident::subengine::{EventHandle, LaneJob, ParamBlock, ScratchArena};
    use crate::resident::subengine::exec::EventRepr;
    /// Prioritised non-blocking stream; all device work of one lane is ordered on it.
    pub struct CudaLane {
        class: LaneClass,
        priority: i32,
        ctx: Arc<CudaContext>,
        stream: Arc<CudaStream>,
        params: ParamBlock,
        scratch: ScratchArena,
        span: tracing::Span,
    }
    fn event_err(e: cudarc::driver::DriverError) -> DeviceError {
        DeviceError::EventRecord {
            reason: e.to_string(),
        }
    }
    impl CudaLane {
        pub(super) fn new(
            ctx: &Arc<CudaContext>,
            class: LaneClass,
            cfg: LaneConfig,
        ) -> Result<Self, DeviceError> {
            let (a, b) = result::stream::get_priority_range()
                .map_err(|_| DeviceError::PriorityUnsupported)?;
            let priority = class.priority().clamp(a.min(b), a.max(b));
            let stream = ctx
                .new_stream_with_priority(priority)
                .map_err(|e| DeviceError::StreamCreate {
                    reason: e.to_string(),
                })?;
            let span = tracing::info_span!(
                "lane", lane = ? class, stream_prio = priority
            );
            Ok(Self {
                class,
                priority,
                ctx: ctx.clone(),
                stream,
                params: ParamBlock::new(cfg.param_capacity),
                scratch: ScratchArena::new(cfg.scratch),
                span,
            })
        }
        pub fn priority(&self) -> i32 {
            self.priority
        }
        pub(crate) fn stream(&self) -> &Arc<CudaStream> {
            &self.stream
        }
    }
    impl ExecContext for CudaLane {
        fn class(&self) -> LaneClass {
            self.class
        }
        fn device(&self) -> DeviceKind {
            DeviceKind::Gpu
        }
        fn submit(&self, job: LaneJob) -> Result<(), DeviceError> {
            self.ctx.bind_to_thread().map_err(event_err)?;
            self.span.in_scope(job)
        }
        fn record_event(&self) -> Result<EventHandle, DeviceError> {
            let event = self.stream.record_event(None).map_err(event_err)?;
            tracing::trace!(parent : & self.span, "event recorded");
            Ok(EventHandle::from_repr(EventRepr::Cuda(event)))
        }
        fn wait(&self, event: &EventHandle) -> Result<(), DeviceError> {
            match event.repr() {
                EventRepr::Cuda(e) => self.stream.wait(e).map_err(event_err),
                EventRepr::Cpu(_) => event.synchronize(),
            }
        }
        fn synchronize(&self) -> Result<(), DeviceError> {
            self.stream.synchronize().map_err(event_err)
        }
        fn param_block(&mut self) -> &mut ParamBlock {
            &mut self.params
        }
        fn scratch(&self) -> &ScratchArena {
            &self.scratch
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::resident::memory::{ArenaId, Region};
    fn cfg() -> LaneConfig {
        LaneConfig::new(Region {
            arena: ArenaId(0),
            offset: 0,
            len: 1024,
            gen: 0,
        })
    }
    #[test]
    fn cpu_handle_opens_cpu_lanes() {
        let dev = DeviceHandle::cpu();
        assert_eq!(dev.kind(), DeviceKind::Cpu);
        assert_eq!(dev.init_cost(), Duration::ZERO);
        let lane = dev.open_lane(LaneClass::Query, cfg()).unwrap();
        assert_eq!(lane.device(), DeviceKind::Cpu);
        assert_eq!(lane.class(), LaneClass::Query);
        lane.synchronize().unwrap();
        dev.warmup().unwrap();
    }
    #[cfg(not(feature = "cuda"))]
    #[test]
    fn cuda_handle_unavailable_without_feature() {
        assert!(matches!(DeviceHandle::cuda(0), Err(DeviceError::CudaUnavailable)));
    }
    #[cfg(feature = "cuda")]
    #[test]
    fn project_cudarc_is_the_one_candle_uses() {
        use std::any::TypeId;
        assert_eq!(
            TypeId::of::< candle_core::cuda_backend::cudarc::driver::CudaContext > (),
            TypeId::of::< cudarc::driver::CudaContext > ()
        );
    }
    #[cfg(feature = "cuda")]
    #[test]
    #[ignore = "requires a CUDA device"]
    fn cuda_lanes_order_work_through_events() {
        let dev = DeviceHandle::cuda(0).unwrap();
        dev.warmup().unwrap();
        let producer = dev.open_lane(LaneClass::Copy, cfg()).unwrap();
        let consumer = dev.open_lane(LaneClass::Query, cfg()).unwrap();
        let event = producer.record_event().unwrap();
        consumer.wait(&event).unwrap();
        consumer.synchronize().unwrap();
        assert!(event.is_complete());
        assert_eq!(consumer.device(), DeviceKind::Gpu);
    }
}
