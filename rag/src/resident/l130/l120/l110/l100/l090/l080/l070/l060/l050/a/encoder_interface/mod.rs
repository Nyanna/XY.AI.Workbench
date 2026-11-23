pub mod projection_layer;
pub mod transformer_model;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{mpsc, Arc, Mutex, MutexGuard};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::a::pooling_norm::PoolNorm;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::memory_access::TensorRef;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::model_descriptor::{
    Normalization, Pooling,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::device_lane_event::{
    Event, EventSignal, Lane,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::layout::DType;
use self::projection_layer::Projection;
use self::transformer_model::TransformerModel;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EncoderError {
    Incompatible { what: &'static str, expected: String, actual: String },
    Device(String),
    Memory(String),
    Processing(String),
}
impl fmt::Display for EncoderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncoderError::Incompatible { what, expected, actual } => {
                write!(
                    f, "inkompatibel: {what}: erwartet {expected}, tatsächlich {actual}"
                )
            }
            EncoderError::Device(s) => write!(f, "Device-Error: {s}"),
            EncoderError::Memory(s) => write!(f, "Memory-Error: {s}"),
            EncoderError::Processing(s) => write!(f, "Verarbeitungsfehler: {s}"),
        }
    }
}
impl std::error::Error for EncoderError {}
pub type TaskOutcome = Result<(), EncoderError>;
pub type TaskSender = mpsc::SyncSender<TaskOutcome>;
pub type TaskReceiver = mpsc::Receiver<TaskOutcome>;
pub fn oneshot() -> (TaskSender, TaskReceiver) {
    mpsc::sync_channel(1)
}
type ErrorSlot = Arc<Mutex<Option<EncoderError>>>;
fn lock<T>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}
#[derive(Clone)]
pub struct CompletionEvent {
    event: Event,
    error: ErrorSlot,
}
impl CompletionEvent {
    pub fn event(&self) -> &Event {
        &self.event
    }
    pub fn is_complete(&self) -> Result<bool, EncoderError> {
        self.event.is_complete().map_err(|e| EncoderError::Device(e.to_string()))
    }
    pub fn wait(&self) -> Result<(), EncoderError> {
        self.event.wait().map_err(|e| EncoderError::Device(e.to_string()))?;
        match lock(&self.error).take() {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }
}
pub struct Encoder {
    model: Arc<TransformerModel>,
    projection: Option<Arc<Projection>>,
    pooling: Pooling,
    pool_norm: PoolNorm,
    storage_dtype: DType,
}
fn incompatible(
    what: &'static str,
    expected: impl Into<String>,
    actual: impl Into<String>,
) -> EncoderError {
    EncoderError::Incompatible {
        what,
        expected: expected.into(),
        actual: actual.into(),
    }
}
impl Encoder {
    pub fn new(
        model: Arc<TransformerModel>,
        projection: Option<Arc<Projection>>,
        pooling: Pooling,
        normalization: Normalization,
        storage_dtype: DType,
    ) -> Result<Encoder, EncoderError> {
        if !matches!(storage_dtype, DType::F32 | DType::F16 | DType::BF16) {
            return Err(
                incompatible(
                    "storage_dtype",
                    "F32, F16 or BF16",
                    storage_dtype.to_safetensors(),
                ),
            );
        }
        if let Some(p) = &projection {
            if p.input_dim() != model.dimension() {
                return Err(
                    incompatible(
                        "projection input_dim",
                        model.dimension().to_string(),
                        p.input_dim().to_string(),
                    ),
                );
            }
        }
        let dim = projection
            .as_ref()
            .map_or_else(|| model.dimension(), |p| p.output_dim());
        let pool_norm = PoolNorm::new(pooling, normalization, dim)
            .map_err(|e| incompatible("pooling", "non-zero dimension", e.to_string()))?;
        Ok(Encoder {
            model,
            projection,
            pooling,
            pool_norm,
            storage_dtype,
        })
    }
    pub fn output_dim(&self) -> usize {
        self.projection
            .as_ref()
            .map_or_else(|| self.model.dimension(), |p| p.output_dim())
    }
    pub fn storage_dtype(&self) -> DType {
        self.storage_dtype
    }
    pub fn encode_async(
        &self,
        lane: &Lane,
        input: &TensorRef,
        output: &TensorRef,
        rows: usize,
        seq_len: usize,
        senders: Vec<TaskSender>,
    ) -> Result<CompletionEvent, EncoderError> {
        let _ = lane;
        check_tensor("input", input, &[2, rows, seq_len], DType::U32)?;
        let out_shape: Vec<usize> = match self.pooling {
            Pooling::Mean | Pooling::Cls => vec![rows, self.output_dim()],
            Pooling::PerToken => vec![rows, seq_len, self.output_dim()],
        };
        check_tensor("output", output, &out_shape, self.storage_dtype)?;
        let (event, signal) = Event::cpu();
        let error: ErrorSlot = Arc::new(Mutex::new(None));
        let job = Job {
            model: Arc::clone(&self.model),
            projection: self.projection.clone(),
            pool_norm: self.pool_norm,
            dtype: self.storage_dtype,
            input: input.clone(),
            output: output.clone(),
            rows,
            seq_len,
        };
        run_job(job, senders, Arc::clone(&error), signal);
        Ok(CompletionEvent { event, error })
    }
}
fn check_tensor(
    what: &'static str,
    t: &TensorRef,
    shape: &[usize],
    dtype: DType,
) -> Result<(), EncoderError> {
    let layout = t.layout();
    if layout.shape() != shape {
        return Err(
            incompatible(what, format!("{shape:?}"), format!("{:?}", layout.shape())),
        );
    }
    if layout.dtype() != dtype {
        return Err(
            incompatible(what, dtype.to_safetensors(), layout.dtype().to_safetensors()),
        );
    }
    if !layout.is_contiguous() {
        return Err(
            incompatible(what, "contiguous", format!("strides {:?}", layout.strides())),
        );
    }
    Ok(())
}
struct Job {
    model: Arc<TransformerModel>,
    projection: Option<Arc<Projection>>,
    pool_norm: PoolNorm,
    dtype: DType,
    input: TensorRef,
    output: TensorRef,
    rows: usize,
    seq_len: usize,
}
struct SignalOnDrop(EventSignal);
impl Drop for SignalOnDrop {
    fn drop(&mut self) {
        self.0.signal();
    }
}
fn run_job(job: Job, senders: Vec<TaskSender>, error: ErrorSlot, signal: EventSignal) {
    let _signal = SignalOnDrop(signal);
    let result = match catch_unwind(AssertUnwindSafe(|| execute(&job))) {
        Ok(r) => r,
        Err(payload) => Err(panic_text(payload.as_ref())),
    };
    let outcome: TaskOutcome = match result {
        Ok(()) => Ok(()),
        Err(text) => {
            log::warn!("Embedding-Batch fehlgeschlagen: {text}");
            let e = EncoderError::Processing(text);
            *lock(&error) = Some(e.clone());
            Err(e)
        }
    };
    for sender in senders {
        let _ = sender.send(outcome.clone());
    }
}
fn panic_text(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        format!("panic: {s}")
    } else if let Some(s) = payload.downcast_ref::<String>() {
        format!("panic: {s}")
    } else {
        "panic".to_string()
    }
}
fn execute(job: &Job) -> Result<(), String> {
    let cells = job
        .rows
        .checked_mul(job.seq_len)
        .ok_or_else(|| "rows * seq_len overflows".to_string())?;
    let words = cells
        .checked_mul(2)
        .ok_or_else(|| "2 * rows * seq_len overflows".to_string())?;
    let byte_len = job.input.layout().byte_len();
    if byte_len != words * 4 {
        return Err(format!("input has {byte_len} bytes, expected {}", words * 4));
    }
    let mut raw = vec![0u8; byte_len];
    job.input
        .region()
        .read(job.input.offset_bytes(), &mut raw)
        .map_err(|e| e.to_string())?;
    let values: Vec<u32> = raw
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect();
    let (ids, mask) = values.split_at(cells);
    let dim = job.model.dimension();
    if dim == 0 {
        return Err("model dimension is zero".to_string());
    }
    let hidden = job
        .model
        .encode(ids, mask, job.rows, job.seq_len)
        .map_err(|e| e.to_string())?;
    if hidden.len() != cells * dim {
        return Err(
            format!("encoder returned {} values, expected {}", hidden.len(), cells * dim),
        );
    }
    let hidden = match &job.projection {
        Some(p) => {
            let tokens: Vec<Vec<f32>> = hidden
                .chunks_exact(dim)
                .map(<[f32]>::to_vec)
                .collect();
            p.apply(&tokens).map_err(|e| e.to_string())?.into_iter().flatten().collect()
        }
        None => hidden,
    };
    let mask_u8: Vec<u8> = mask.iter().map(|&m| u8::from(m != 0)).collect();
    let pooled = job
        .pool_norm
        .apply(&hidden, &mask_u8, job.rows, job.seq_len)
        .map_err(|e| e.to_string())?;
    let expected = job.output.layout().elem_count();
    if pooled.vectors.len() != expected {
        return Err(
            format!(
                "result has {} values, output holds {expected}", pooled.vectors.len()
            ),
        );
    }
    let bytes = to_storage_bytes(&pooled.vectors, job.dtype)?;
    if bytes.is_empty() {
        return Ok(());
    }
    job.output
        .region()
        .write(job.output.offset_bytes(), &bytes)
        .map_err(|e| e.to_string())
}
fn to_storage_bytes(values: &[f32], dtype: DType) -> Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(values.len() * dtype.size_in_bytes());
    match dtype {
        DType::F32 => values.iter().for_each(|v| out.extend_from_slice(&v.to_le_bytes())),
        DType::F16 => {
            values
                .iter()
                .for_each(|&v| out.extend_from_slice(&f32_to_f16_bits(v).to_le_bytes()))
        }
        DType::BF16 => {
            values
                .iter()
                .for_each(|&v| out.extend_from_slice(&f32_to_bf16_bits(v).to_le_bytes()))
        }
        other => {
            return Err(format!("unsupported storage type {}", other.to_safetensors()));
        }
    }
    Ok(out)
}
fn f32_to_bf16_bits(v: f32) -> u16 {
    let bits = v.to_bits();
    if v.is_nan() {
        return ((bits >> 16) as u16) | 0x0040;
    }
    let rounded = bits.wrapping_add(0x7FFF + ((bits >> 16) & 1));
    (rounded >> 16) as u16
}
fn f32_to_f16_bits(v: f32) -> u16 {
    let bits = v.to_bits();
    let sign = ((bits >> 16) & 0x8000) as u16;
    let exp = ((bits >> 23) & 0xFF) as i32;
    let man = bits & 0x007F_FFFF;
    if exp == 0xFF {
        return if man == 0 { sign | 0x7C00 } else { sign | 0x7E00 | (man >> 13) as u16 };
    }
    let e = exp - 127 + 15;
    if e >= 31 {
        return sign | 0x7C00;
    }
    if e <= 0 {
        if e < -10 {
            return sign;
        }
        let full = man | 0x0080_0000;
        let shift = (14 - e) as u32;
        let mut half = full >> shift;
        let rem = full & ((1u32 << shift) - 1);
        let tie = 1u32 << (shift - 1);
        if rem > tie || (rem == tie && half & 1 == 1) {
            half += 1;
        }
        return sign | half as u16;
    }
    let mut half = ((e as u32) << 10) | (man >> 13);
    let rem = man & 0x1FFF;
    if rem > 0x1000 || (rem == 0x1000 && half & 1 == 1) {
        half += 1;
    }
    sign | half as u16
}
#[cfg(test)]
#[path = "test/encoder_interface.rs"]
mod tests;
