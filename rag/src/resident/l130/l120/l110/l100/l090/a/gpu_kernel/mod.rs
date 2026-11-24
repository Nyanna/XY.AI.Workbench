pub mod max_similarity_operation;
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::kernel_register::{
    Kernel, KernelError, KernelFunction, KernelRegister, KernelSource, Variant,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    Event, Lane, LaneError,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::{
    DType, MASK_VALUE,
};
pub const BLOCK_THREADS: u32 = 128;
pub const KERNEL_DENSE_DOT: &str = "dense_dot";
pub const KERNEL_FUSED_MAXSIM: &str = "fused_maxsim";
pub const NO_VALID_TOKEN_SCORE: f32 = MASK_VALUE;
const ENTRY_DENSE_DOT: &str = "xy_dense_dot";
const ENTRY_FUSED_MAXSIM: &str = "xy_fused_maxsim";
const GPU_DTYPES: &[DType] = &[DType::F32, DType::F16];
const PARAMS_SIZE: usize = std::mem::size_of::<KernelParams>();
macro_rules! device_common {
    () => {
        r#"
#define BLOCK_THREADS 128
#define NO_VALID_TOKEN_SCORE (-10000.0f)

struct SegmentEntry {
  unsigned long long base;
  unsigned long long lens_addr;
  unsigned long long stride_bytes;
  unsigned int first_slot;
  unsigned int slot_count;
  unsigned int pad_len;
  unsigned int reserved;
};

struct KernelParams {
  unsigned long long slots_addr;
  unsigned long long segments_addr;
  unsigned long long query_addr;
  unsigned long long out_addr;
  unsigned long long completed_addr;
  unsigned long long block_ctr_addr;
  unsigned int ring_cap;
  unsigned int head;
  unsigned int tail;
  unsigned int segment_count;
  unsigned int query_len;
  unsigned int reserved;
};

static_assert(sizeof(SegmentEntry) == 40, "SegmentEntry layout");
static_assert(sizeof(KernelParams) == 72, "KernelParams layout");

__device__ __forceinline__ const SegmentEntry* find_segment(const KernelParams& p, unsigned int slot) {
  const SegmentEntry* s = (const SegmentEntry*)p.segments_addr;
  for (unsigned int i = 0; i < p.segment_count; ++i) {
    if (slot - s[i].first_slot < s[i].slot_count) return s + i;
  }
  return 0;
}

__device__ __forceinline__ void finish_batch(const KernelParams& p) {
  __shared__ unsigned int is_last;
  __threadfence();
  __syncthreads();
  if (threadIdx.x == 0) {
    unsigned int done = atomicAdd((unsigned int*)p.block_ctr_addr, 1u) + 1u;
    is_last = (done == gridDim.x) ? 1u : 0u;
  }
  __syncthreads();
  if (is_last != 0u && threadIdx.x == 0) {
    __threadfence();
    *(volatile unsigned int*)p.block_ctr_addr = 0u;
    unsigned long long previous;
    asm volatile("atom.add.release.sys.u64 %0, [%1], %2;"
                 : "=l"(previous)
                 : "l"(p.completed_addr), "l"(1ull)
                 : "memory");
  }
}
"#
    };
}
const DENSE_DOT_BODY: &str = concat!(
    device_common!(),
    r#"
extern "C" __global__ void xy_dense_dot(const KernelParams p) {
  __shared__ float sums[4];
  const unsigned int total = p.tail - p.head;
  const unsigned int b = blockIdx.x;
  if (b < total) {
    const unsigned int slot = ((const unsigned int*)p.slots_addr)[(p.head + b) % p.ring_cap];
    const SegmentEntry* seg = find_segment(p, slot);
    const float* queries = (const float*)p.query_addr;
    float* out = (float*)p.out_addr;
    const T* doc = 0;
    if (seg != 0) {
      doc = (const T*)(seg->base + (unsigned long long)(slot - seg->first_slot) * seg->stride_bytes);
    }
    const int lane = threadIdx.x & 31;
    const int warp = threadIdx.x >> 5;
    for (unsigned int q = 0; q < p.query_len; ++q) {
      float acc = 0.0f;
      if (doc != 0) {
        for (int i = threadIdx.x; i < DIM; i += BLOCK_THREADS) {
          acc += queries[(size_t)q * DIM + i] * load_f(doc, (size_t)i);
        }
      }
      acc = warp_reduce_sum(acc);
      if (lane == 0) sums[warp] = acc;
      __syncthreads();
      if (threadIdx.x == 0) {
        out[(size_t)q * total + b] = (sums[0] + sums[1]) + (sums[2] + sums[3]);
      }
      __syncthreads();
    }
  }
  finish_batch(p);
}
"#
);
const FUSED_MAXSIM_BODY: &str = concat!(
    device_common!(),
    r#"
extern "C" __global__ void xy_fused_maxsim(const KernelParams p) {
  __shared__ float maxes[4];
  const unsigned int total = p.tail - p.head;
  const unsigned int b = blockIdx.x;
  if (b < total) {
    const unsigned int slot = ((const unsigned int*)p.slots_addr)[(p.head + b) % p.ring_cap];
    const SegmentEntry* seg = find_segment(p, slot);
    const float* queries = (const float*)p.query_addr;
    const T* doc = 0;
    unsigned int len = 0;
    if (seg != 0) {
      doc = (const T*)(seg->base + (unsigned long long)(slot - seg->first_slot) * seg->stride_bytes);
      len = seg->pad_len;
      if (seg->lens_addr != 0ull) {
        const unsigned int listed = ((const unsigned int*)seg->lens_addr)[slot - seg->first_slot];
        len = listed < len ? listed : len;
      }
    }
    const int lane = threadIdx.x & 31;
    const int warp = threadIdx.x >> 5;
    const float neg_inf = __int_as_float(0xff800000);
    float sum = 0.0f;
    for (unsigned int t = 0; t < p.query_len; ++t) {
      const float* qt = queries + (size_t)t * DIM;
      float best = neg_inf;
      for (unsigned int d = threadIdx.x; d < len; d += BLOCK_THREADS) {
        const T* row = doc + (size_t)d * DIM;
        float acc = 0.0f;
        for (int i = 0; i < DIM; ++i) {
          acc += qt[i] * load_f(row, (size_t)i);
        }
        best = fmaxf(best, acc);
      }
      best = warp_reduce_max(best);
      if (lane == 0) maxes[warp] = best;
      __syncthreads();
      const float m = fmaxf(fmaxf(maxes[0], maxes[1]), fmaxf(maxes[2], maxes[3]));
      sum += (len == 0) ? NO_VALID_TOKEN_SCORE : m;
      __syncthreads();
    }
    if (threadIdx.x == 0) {
      ((float*)p.out_addr)[b] = sum;
    }
  }
  finish_batch(p);
}
"#
);
/// Registers the `dense_dot` and `fused_maxsim` JIT sources; a second call fails with the
/// register's duplicate-name error.
pub fn register_gpu_kernels(register: &KernelRegister) -> Result<(), KernelError> {
    register
        .register_source(KernelSource {
            name: KERNEL_DENSE_DOT,
            body: DENSE_DOT_BODY,
            entry: ENTRY_DENSE_DOT,
            dtypes: GPU_DTYPES,
            precompiled: None,
        })?;
    register
        .register_source(KernelSource {
            name: KERNEL_FUSED_MAXSIM,
            body: FUSED_MAXSIM_BODY,
            entry: ENTRY_FUSED_MAXSIM,
            dtypes: GPU_DTYPES,
            precompiled: None,
        })
}
/// Segment table entry in device memory.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SegmentEntry {
    pub base: u64,
    pub lens_addr: u64,
    pub stride_bytes: u64,
    pub first_slot: u32,
    pub slot_count: u32,
    pub pad_len: u32,
    pub _reserved: u32,
}
impl SegmentEntry {
    pub const BYTES: usize = 40;
    /// Native-endian, in field order; offsets match the `repr(C)` layout.
    pub fn to_bytes(&self) -> [u8; SegmentEntry::BYTES] {
        let mut out = [0u8; SegmentEntry::BYTES];
        out[0..8].copy_from_slice(&self.base.to_ne_bytes());
        out[8..16].copy_from_slice(&self.lens_addr.to_ne_bytes());
        out[16..24].copy_from_slice(&self.stride_bytes.to_ne_bytes());
        out[24..28].copy_from_slice(&self.first_slot.to_ne_bytes());
        out[28..32].copy_from_slice(&self.slot_count.to_ne_bytes());
        out[32..36].copy_from_slice(&self.pad_len.to_ne_bytes());
        out[36..40].copy_from_slice(&self._reserved.to_ne_bytes());
        out
    }
}
const _: () = assert!(SegmentEntry::BYTES == std::mem::size_of::< SegmentEntry > ());
/// Segment table for the kernel: the entries back to back.
pub fn encode_segment_table(entries: &[SegmentEntry]) -> Vec<u8> {
    let mut out = Vec::with_capacity(entries.len() * SegmentEntry::BYTES);
    for e in entries {
        out.extend_from_slice(&e.to_bytes());
    }
    out
}
/// Kernel argument block, passed by value as the only kernel argument.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KernelParams {
    pub slots_addr: u64,
    pub segments_addr: u64,
    pub query_addr: u64,
    pub out_addr: u64,
    pub completed_addr: u64,
    pub block_ctr_addr: u64,
    pub ring_cap: u32,
    pub head: u32,
    pub tail: u32,
    pub segment_count: u32,
    pub query_len: u32,
    pub _reserved: u32,
}
impl KernelParams {
    pub fn slot_total(&self) -> u32 {
        self.tail.wrapping_sub(self.head)
    }
    fn to_bytes(&self) -> [u8; PARAMS_SIZE] {
        let mut out = [0u8; PARAMS_SIZE];
        let wide = [
            self.slots_addr,
            self.segments_addr,
            self.query_addr,
            self.out_addr,
            self.completed_addr,
            self.block_ctr_addr,
        ];
        for (i, v) in wide.iter().enumerate() {
            out[i * 8..i * 8 + 8].copy_from_slice(&v.to_ne_bytes());
        }
        let narrow = [
            self.ring_cap,
            self.head,
            self.tail,
            self.segment_count,
            self.query_len,
            self._reserved,
        ];
        for (i, v) in narrow.iter().enumerate() {
            let at = 48 + i * 4;
            out[at..at + 4].copy_from_slice(&v.to_ne_bytes());
        }
        out
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchConfig {
    pub grid: u32,
    pub block: u32,
    pub dynamic_shared: u32,
}
/// Implemented by the GPU binding; enqueues the launch in the lane's stream order without
/// waiting for completion. `Err` carries the driver's cause text.
pub trait GpuLauncher: Send + Sync {
    fn launch(
        &self,
        function: &Arc<dyn KernelFunction>,
        lane: &Lane,
        cfg: &LaunchConfig,
        params: &[u8],
    ) -> Result<(), String>;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GpuKernelError {
    UnsupportedDType(DType),
    InvalidParams(String),
    AckBehind { required: u64, acked: u64 },
    Kernel(KernelError),
    Launch { kernel: &'static str, grid: u32, block: u32, cause: String },
    Lane(LaneError),
}
impl fmt::Display for GpuKernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GpuKernelError::UnsupportedDType(d) => {
                write!(f, "unsupported document dtype for gpu scoring: {d:?}")
            }
            GpuKernelError::InvalidParams(m) => {
                write!(f, "invalid kernel parameters: {m}")
            }
            GpuKernelError::AckBehind { required, acked } => {
                write!(
                    f,
                    "previous batch not acknowledged: required {required}, acked {acked}"
                )
            }
            GpuKernelError::Kernel(e) => write!(f, "kernel unavailable: {e}"),
            GpuKernelError::Launch { kernel, grid, block, cause } => {
                write!(
                    f, "launch of {kernel} failed (grid {grid}, block {block}): {cause}"
                )
            }
            GpuKernelError::Lane(e) => {
                write!(f, "recording completion event failed: {e}")
            }
        }
    }
}
impl std::error::Error for GpuKernelError {}
/// Launcher for embedding dimension `DIM`; one instance serves one doorbell and one lane.
pub struct GpuKernels<const DIM: usize> {
    register: Arc<KernelRegister>,
    launcher: Arc<dyn GpuLauncher>,
    launched: Mutex<u64>,
}
static_assertions::assert_impl_all!(GpuKernels < 8 >: Send, Sync);
fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}
impl<const DIM: usize> GpuKernels<DIM> {
    const DIM_VALID: () = assert!(DIM >= 1 && DIM <= u32::MAX as usize);
    pub fn new(register: Arc<KernelRegister>, launcher: Arc<dyn GpuLauncher>) -> Self {
        let () = Self::DIM_VALID;
        GpuKernels {
            register,
            launcher,
            launched: Mutex::new(0),
        }
    }
    /// Number of successfully launched batches (= number of the last batch).
    pub fn launched(&self) -> u64 {
        *lock(&self.launched)
    }
    pub fn dense_dot(
        &self,
        lane: &mut Lane,
        doc_dtype: DType,
        p: &KernelParams,
        acked: u64,
    ) -> Result<Event, GpuKernelError> {
        self.run(KERNEL_DENSE_DOT, lane, doc_dtype, p, acked)
    }
    pub fn fused_max_sim(
        &self,
        lane: &mut Lane,
        doc_dtype: DType,
        p: &KernelParams,
        acked: u64,
    ) -> Result<Event, GpuKernelError> {
        self.run(KERNEL_FUSED_MAXSIM, lane, doc_dtype, p, acked)
    }
    fn run(
        &self,
        kernel: &'static str,
        lane: &mut Lane,
        doc_dtype: DType,
        p: &KernelParams,
        acked: u64,
    ) -> Result<Event, GpuKernelError> {
        if !GPU_DTYPES.contains(&doc_dtype) {
            return Err(GpuKernelError::UnsupportedDType(doc_dtype));
        }
        Self::validate(p, lane.params().len())?;
        let mut launched = lock(&self.launched);
        let n = *launched;
        if n >= 1 && acked < n {
            return Err(GpuKernelError::AckBehind {
                required: n,
                acked,
            });
        }
        let function = self.function(kernel, doc_dtype)?;
        lane.params_mut()[..PARAMS_SIZE].copy_from_slice(&p.to_bytes());
        let cfg = LaunchConfig {
            grid: p.slot_total().max(1),
            block: BLOCK_THREADS,
            dynamic_shared: 0,
        };
        self.launcher
            .launch(&function, lane, &cfg, &lane.params()[..PARAMS_SIZE])
            .map_err(|cause| GpuKernelError::Launch {
                kernel,
                grid: cfg.grid,
                block: cfg.block,
                cause,
            })?;
        *launched = n + 1;
        lane.record_event().map_err(GpuKernelError::Lane)
    }
    fn validate(p: &KernelParams, param_block_len: usize) -> Result<(), GpuKernelError> {
        let invalid = |m: &str| Err(GpuKernelError::InvalidParams(m.to_string()));
        let total = p.slot_total();
        if p.query_len == 0 {
            return invalid("query_len must be at least 1");
        }
        if p.ring_cap == 0 {
            return invalid("ring_cap must be greater than 0");
        }
        if total > p.ring_cap {
            return invalid("slot count exceeds ring_cap");
        }
        if total > i32::MAX as u32 {
            return invalid("slot count exceeds i32::MAX");
        }
        if total > 0 && p.segment_count == 0 {
            return invalid("segment_count must be at least 1 when slots are scored");
        }
        if PARAMS_SIZE > param_block_len {
            return invalid("lane parameter block is smaller than the kernel parameters");
        }
        Ok(())
    }
    fn function(
        &self,
        name: &'static str,
        dtype: DType,
    ) -> Result<Arc<dyn KernelFunction>, GpuKernelError> {
        let variant = Variant {
            dim: Some(DIM as u32),
            tile: None,
        };
        match self.register.request(name, dtype, variant) {
            Ok(Kernel::Gpu(k)) => Ok(Arc::clone(&k.function)),
            Ok(Kernel::Library(_)) => {
                Err(
                    GpuKernelError::Kernel(
                        KernelError::Unavailable(
                            format!(
                                "{name}: resolved to a library kernel, not a gpu kernel"
                            ),
                        ),
                    ),
                )
            }
            Err(e) => Err(GpuKernelError::Kernel(e)),
        }
    }
}
#[cfg(test)]
#[path = "test/gpu_kernel.rs"]
mod tests;
