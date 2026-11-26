//! Normalizes pooled vectors once, converts them to the output dtype and writes them gap-free.
pub mod segment_writer;
use std::fmt;
use log::warn;
use self::segment_writer::Reservation;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::kernel_register::{
    GpuKernel, Kernel, KernelError, KernelRegister, KernelSource, Variant,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::library_adapter::LibTensor;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::kernel_layout::Addr;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::pooling_norm::{
    PoolNorm, PoolNormError,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::memory_access::{
    AccessError, AccessRegion,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::model_descriptor::Normalization;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::{
    device_lane_event::{Event, EventError, Lane, LaneError},
    float_codec::{f32_to_bf16_bits, f32_to_f16_bits},
    layout::DType,
};
pub const WRITE_KERNEL_NAME: &str = "embedding_write";
const WRITE_KERNEL_ENTRY: &str = "embedding_write";
const WRITE_KERNEL_DTYPES: &[DType] = &[DType::F32];
const WRITE_KERNEL_BODY: &str = r#"
__device__ __forceinline__ unsigned int eo_f16_bits(float f) {
    unsigned int x = __float_as_uint(f);
    unsigned int sign = (x >> 16) & 0x8000u;
    unsigned int mag = x & 0x7FFFFFFFu;
    if (mag > 0x7F800000u) return sign | 0x7E00u;
    if (mag >= 0x477FF000u) return sign | 0x7C00u;
    if (mag >= 0x38800000u) {
        unsigned int r = mag - 0x38000000u;
        r = (r + 0xFFFu + ((r >> 13) & 1u)) >> 13;
        return sign | r;
    }
    unsigned int e = mag >> 23;
    if (e < 102u) return sign;
    unsigned int m = (mag & 0x7FFFFFu) | 0x800000u;
    unsigned int s = 126u - e;
    unsigned int q = m >> s;
    unsigned int rem = m & ((1u << s) - 1u);
    unsigned int half = 1u << (s - 1u);
    if (rem > half || (rem == half && (q & 1u))) q++;
    return sign | q;
}

__device__ __forceinline__ unsigned int eo_bf16_bits(float f) {
    unsigned int x = __float_as_uint(f);
    if ((x & 0x7FFFFFFFu) > 0x7F800000u) return ((x >> 16) & 0x8000u) | 0x7FC0u;
    return (x + 0x7FFFu + ((x >> 16) & 1u)) >> 16;
}

extern "C" __global__ void embedding_write(
    const T* src, unsigned char* dst, unsigned long long n,
    unsigned int normalize, unsigned int out_dtype) {
    unsigned long long vec = blockIdx.x;
    if (vec >= n || out_dtype > 2u) return;
    unsigned int lane = threadIdx.x & 31u;
    const T* row = src + vec * DIM;
    float scale = 1.0f;
    if (normalize != 0u) {
        float acc = 0.0f;
        for (int i = (int)lane; i < DIM; i += 32) {
            float x = load_f(row, (size_t)i);
            acc += x * x;
        }
        acc = warp_reduce_sum(acc);
        float nrm = sqrtf(acc);
        scale = isnan(nrm) ? 1.0f : fmaxf(nrm, 1e-12f);
    }
    unsigned int width = out_dtype == 0u ? 4u : 2u;
    unsigned char* out = dst + vec * DIM * width;
    for (int i = (int)lane; i < DIM; i += 32) {
        float x = load_f(row, (size_t)i);
        if (normalize != 0u) x = x / scale;
        unsigned int bits = out_dtype == 0u ? __float_as_uint(x)
                          : (out_dtype == 1u ? eo_f16_bits(x) : eo_bf16_bits(x));
        unsigned char* p = out + (size_t)i * width;
        p[0] = (unsigned char)(bits & 0xFFu);
        p[1] = (unsigned char)((bits >> 8) & 0xFFu);
        if (width == 4u) {
            p[2] = (unsigned char)((bits >> 16) & 0xFFu);
            p[3] = (unsigned char)((bits >> 24) & 0xFFu);
        }
    }
}
"#;
#[derive(Debug, Clone, PartialEq)]
pub enum OutputError {
    Incompatible(DType),
    Memory(AccessError),
    PoolNorm(PoolNormError),
    InvalidResult(String),
    SlotTooSmall { required: u64, available: u64 },
    Kernel(KernelError),
    Lane(LaneError),
}
impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputError::Incompatible(d) => {
                write!(f, "inkompatibel: Output-Datentyp {d:?}")
            }
            OutputError::Memory(e) => write!(f, "output write failed: {e}"),
            OutputError::PoolNorm(e) => write!(f, "normalization failed: {e}"),
            OutputError::InvalidResult(t) => write!(f, "invalid result tensor: {t}"),
            OutputError::SlotTooSmall { required, available } => {
                write!(
                    f, "slot too small: {required} bytes required, {available} available"
                )
            }
            OutputError::Kernel(e) => write!(f, "write kernel unavailable: {e}"),
            OutputError::Lane(e) => write!(f, "lane error: {e}"),
        }
    }
}
impl std::error::Error for OutputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            OutputError::Memory(e) => Some(e),
            OutputError::PoolNorm(e) => Some(e),
            OutputError::Kernel(e) => Some(e),
            OutputError::Lane(e) => Some(e),
            _ => None,
        }
    }
}
impl From<AccessError> for OutputError {
    fn from(e: AccessError) -> Self {
        OutputError::Memory(e)
    }
}
impl From<PoolNormError> for OutputError {
    fn from(e: PoolNormError) -> Self {
        OutputError::PoolNorm(e)
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WriteReport {
    pub num_vectors: usize,
    pub bytes_written: usize,
    pub degenerate_count: usize,
}
pub fn is_supported_output(dtype: DType) -> bool {
    matches!(dtype, DType::F32 | DType::F16 | DType::BF16)
}
/// Little-endian bytes of `vectors` in `dtype`.
pub fn encode_f32_slice(vectors: &[f32], dtype: DType) -> Result<Vec<u8>, OutputError> {
    if !is_supported_output(dtype) {
        return Err(OutputError::Incompatible(dtype));
    }
    let mut out = Vec::with_capacity(vectors.len() * dtype.size_in_bytes());
    match dtype {
        DType::F32 => {
            for v in vectors {
                out.extend_from_slice(&v.to_le_bytes());
            }
        }
        DType::F16 => {
            for v in vectors {
                out.extend_from_slice(&f32_to_f16_bits(*v).to_le_bytes());
            }
        }
        _ => {
            for v in vectors {
                out.extend_from_slice(&f32_to_bf16_bits(*v).to_le_bytes());
            }
        }
    }
    Ok(out)
}
/// Host path: normalizes `vectors` in place, converts and writes them with one `AccessRegion::write`.
pub fn write_embeddings(
    region: &AccessRegion,
    offset_bytes: usize,
    vectors: &mut [f32],
    norm: &PoolNorm,
    out_dtype: DType,
) -> Result<WriteReport, OutputError> {
    if !is_supported_output(out_dtype) {
        return Err(OutputError::Incompatible(out_dtype));
    }
    let degenerate_count = norm.normalize(vectors)?;
    let buf = encode_f32_slice(vectors, out_dtype)?;
    if !buf.is_empty() {
        region.write(offset_bytes, &buf)?;
    }
    if degenerate_count > 0 {
        warn!("{degenerate_count} empty embeddings stored as zero vectors");
    }
    Ok(WriteReport {
        num_vectors: vectors.len() / norm.dim,
        bytes_written: buf.len(),
        degenerate_count,
    })
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WriteKernelArgs {
    pub num_vectors: u64,
    pub dim: u32,
    pub normalize: bool,
    pub out_dtype: DType,
    pub dst: Addr,
}
impl WriteKernelArgs {
    /// Kernel encoding of `out_dtype`: 0 = F32, 1 = F16, 2 = BF16.
    pub fn out_dtype_code(&self) -> u32 {
        match self.out_dtype {
            DType::F16 => 1,
            DType::BF16 => 2,
            _ => 0,
        }
    }
}
/// Implemented by the GPU binding.
pub trait WriteKernelLauncher: Send + Sync {
    /// Reads the device pointer of `result` and enqueues `kernel` on the stream of `lane` with
    /// `num_vectors` blocks of 32 threads. Does not wait for completion.
    fn launch(
        &self,
        lane: &Lane,
        kernel: &GpuKernel,
        args: &WriteKernelArgs,
        result: &LibTensor,
    ) -> Result<(), String>;
}
pub fn write_kernel_source() -> KernelSource {
    KernelSource {
        name: WRITE_KERNEL_NAME,
        body: WRITE_KERNEL_BODY,
        entry: WRITE_KERNEL_ENTRY,
        dtypes: WRITE_KERNEL_DTYPES,
        precompiled: None,
    }
}
pub fn register_write_kernel(reg: &KernelRegister) -> Result<(), KernelError> {
    reg.register_source(write_kernel_source())
}
/// Keeps the result tensor alive until the write event completed.
#[derive(Debug)]
pub struct GpuWriteTicket {
    #[allow(dead_code)]
    result: LibTensor,
    event: Event,
}
static_assertions::assert_impl_all!(GpuWriteTicket : Send, Sync);
impl GpuWriteTicket {
    pub fn event(&self) -> &Event {
        &self.event
    }
    pub fn is_complete(&self) -> Result<bool, EventError> {
        self.event.is_complete()
    }
    pub fn wait(&self) -> Result<(), EventError> {
        self.event.wait()
    }
}
impl Drop for GpuWriteTicket {
    fn drop(&mut self) {
        if !matches!(self.event.is_complete(), Ok(true)) {
            let _ = self.event.wait();
        }
    }
}
/// GPU path: enqueues the write kernel for `result` into `slot` and returns the ticket carrying
/// the event; no bytes are written on the host and nothing blocks on the GPU.
pub fn write_embeddings_gpu(
    register: &KernelRegister,
    launcher: &dyn WriteKernelLauncher,
    lane: &Lane,
    result: &LibTensor,
    norm: &PoolNorm,
    out_dtype: DType,
    slot: &Reservation,
) -> Result<GpuWriteTicket, OutputError> {
    if !is_supported_output(out_dtype) {
        return Err(OutputError::Incompatible(out_dtype));
    }
    if norm.dim == 0 {
        return Err(OutputError::PoolNorm(PoolNormError::ZeroDimension));
    }
    match result.dtype() {
        Ok(DType::F32) => {}
        Ok(other) => {
            return Err(
                OutputError::InvalidResult(format!("result dtype {other:?} is not F32")),
            );
        }
        Err(e) => return Err(OutputError::InvalidResult(e.to_string())),
    }
    let dims = result.dims();
    if dims.len() != 2 || dims[1] != norm.dim {
        return Err(
            OutputError::InvalidResult(
                format!("result dims {dims:?} do not match [n, {}]", norm.dim),
            ),
        );
    }
    let n = dims[0];
    let available = slot.target.end.saturating_sub(slot.target.start);
    let required = (n as u64)
        .checked_mul(norm.dim as u64)
        .and_then(|e| e.checked_mul(out_dtype.size_in_bytes() as u64));
    match required {
        Some(required) if required <= available => {}
        Some(required) => {
            return Err(OutputError::SlotTooSmall {
                required,
                available,
            });
        }
        None => {
            return Err(OutputError::SlotTooSmall {
                required: u64::MAX,
                available,
            });
        }
    }
    let dim = u32::try_from(norm.dim)
        .map_err(|_| OutputError::InvalidResult(
            format!("dim {} exceeds u32", norm.dim),
        ))?;
    let kernel = register
        .request(
            WRITE_KERNEL_NAME,
            DType::F32,
            Variant {
                dim: Some(dim),
                tile: None,
            },
        )
        .map_err(OutputError::Kernel)?;
    let kernel = match kernel {
        Kernel::Gpu(k) => k,
        Kernel::Library(_) | Kernel::Cpu(_) => {
            return Err(
                OutputError::Kernel(
                    KernelError::UnknownKernel(WRITE_KERNEL_NAME.to_string()),
                ),
            );
        }
    };
    if n > 0 {
        let args = WriteKernelArgs {
            num_vectors: n as u64,
            dim,
            normalize: norm.normalization == Normalization::L2,
            out_dtype,
            dst: slot.addr,
        };
        launcher
            .launch(lane, &kernel, &args, result)
            .map_err(|text| OutputError::Memory(AccessError::Backend(text)))?;
    }
    let event = lane.record_event().map_err(OutputError::Lane)?;
    Ok(GpuWriteTicket {
        result: result.clone(),
        event,
    })
}
#[cfg(test)]
#[path = "test/embedding_output.rs"]
mod tests;
