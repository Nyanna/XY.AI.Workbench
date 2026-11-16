use std::fmt;
/// Row-major, dense. `data.len() == rows * cols`.
#[derive(Debug, Clone, PartialEq)]
pub struct FloatBatch {
    rows: usize,
    cols: usize,
    data: Vec<f32>,
}
impl FloatBatch {
    pub fn new(rows: usize, cols: usize, data: Vec<f32>) -> Result<Self, ComputeError> {
        let len = rows.checked_mul(cols).ok_or(ComputeError::Overflow)?;
        if data.len() != len {
            return Err(
                ComputeError::Mismatch(
                    format!(
                        "batch [{rows}, {cols}] needs {len} values, got {}", data.len()
                    ),
                ),
            );
        }
        Ok(Self { rows, cols, data })
    }
    pub fn rows(&self) -> usize {
        self.rows
    }
    pub fn cols(&self) -> usize {
        self.cols
    }
    pub fn as_slice(&self) -> &[f32] {
        &self.data
    }
    pub fn into_vec(self) -> Vec<f32> {
        self.data
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BatchKernel {
    /// `[m,k] x [k,n] -> [m,n]`
    MatMul,
    /// Same shape, element-wise.
    Add,
    /// Same shape, element-wise.
    Mul,
    /// Softmax per row.
    SoftmaxLastDim,
}
impl BatchKernel {
    pub fn arity(&self) -> usize {
        match self {
            BatchKernel::MatMul | BatchKernel::Add | BatchKernel::Mul => 2,
            BatchKernel::SoftmaxLastDim => 1,
        }
    }
    /// Kernel name used by the library adapter.
    pub fn name(&self) -> &'static str {
        match self {
            BatchKernel::MatMul => "matmul",
            BatchKernel::Add => "add",
            BatchKernel::Mul => "mul",
            BatchKernel::SoftmaxLastDim => "softmax_last_dim",
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackendKind {
    Cpu,
    Gpu,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComputeError {
    /// Wrong operand count or incompatible shapes.
    Mismatch(String),
    Overflow,
    /// GPU acceleration is not available.
    GpuUnavailable(String),
    /// Error reported by the GPU library or the adapter.
    Backend(String),
}
impl fmt::Display for ComputeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ComputeError::Mismatch(m) => write!(f, "operand mismatch: {m}"),
            ComputeError::Overflow => write!(f, "dimension computation overflows"),
            ComputeError::GpuUnavailable(m) => write!(f, "gpu unavailable: {m}"),
            ComputeError::Backend(m) => write!(f, "backend error: {m}"),
        }
    }
}
impl std::error::Error for ComputeError {}
/// Swappable float-batch to float-batch compute interface.
pub trait BatchCompute: Send + Sync {
    fn kind(&self) -> BackendKind;
    fn run(
        &self,
        kernel: BatchKernel,
        inputs: &[&FloatBatch],
    ) -> Result<FloatBatch, ComputeError>;
}
/// Validates operands and returns the result shape.
fn check_operands(
    kernel: BatchKernel,
    inputs: &[&FloatBatch],
) -> Result<(usize, usize), ComputeError> {
    if inputs.len() != kernel.arity() {
        return Err(
            ComputeError::Mismatch(
                format!(
                    "{}: expected {} operands, got {}", kernel.name(), kernel.arity(),
                    inputs.len()
                ),
            ),
        );
    }
    let a = inputs[0];
    let shape = match kernel {
        BatchKernel::MatMul => {
            let b = inputs[1];
            if a.cols != b.rows {
                return Err(
                    ComputeError::Mismatch(
                        format!(
                            "matmul: [{}, {}] x [{}, {}]", a.rows, a.cols, b.rows, b.cols
                        ),
                    ),
                );
            }
            (a.rows, b.cols)
        }
        BatchKernel::Add | BatchKernel::Mul => {
            let b = inputs[1];
            if (a.rows, a.cols) != (b.rows, b.cols) {
                return Err(
                    ComputeError::Mismatch(
                        format!(
                            "{}: [{}, {}] vs [{}, {}]", kernel.name(), a.rows, a.cols, b
                            .rows, b.cols
                        ),
                    ),
                );
            }
            (a.rows, a.cols)
        }
        BatchKernel::SoftmaxLastDim => (a.rows, a.cols),
    };
    shape.0.checked_mul(shape.1).ok_or(ComputeError::Overflow)?;
    Ok(shape)
}
fn alloc_zeros(len: usize) -> Result<Vec<f32>, ComputeError> {
    let mut v: Vec<f32> = Vec::new();
    v.try_reserve_exact(len).map_err(|_| ComputeError::Overflow)?;
    v.resize(len, 0.0);
    Ok(v)
}
/// Result for empty outputs and for `MatMul` with an empty inner dimension.
fn trivial_result(
    kernel: BatchKernel,
    inputs: &[&FloatBatch],
    (rows, cols): (usize, usize),
) -> Result<Option<FloatBatch>, ComputeError> {
    let len = rows.checked_mul(cols).ok_or(ComputeError::Overflow)?;
    if len == 0 {
        return Ok(
            Some(FloatBatch {
                rows,
                cols,
                data: Vec::new(),
            }),
        );
    }
    if kernel == BatchKernel::MatMul && inputs[0].cols == 0 {
        return Ok(
            Some(FloatBatch {
                rows,
                cols,
                data: alloc_zeros(len)?,
            }),
        );
    }
    Ok(None)
}
/// CPU reference implementation (default and fallback).
#[derive(Debug, Clone, Copy, Default)]
pub struct CpuBatchCompute;
impl CpuBatchCompute {
    pub fn new() -> Self {
        CpuBatchCompute
    }
}
fn cpu_matmul(a: &FloatBatch, b: &FloatBatch) -> Result<Vec<f32>, ComputeError> {
    let (k, n) = (a.cols, b.cols);
    let mut out = alloc_zeros(a.rows * n)?;
    for (a_row, out_row) in a.data.chunks_exact(k).zip(out.chunks_exact_mut(n)) {
        for (&av, b_row) in a_row.iter().zip(b.data.chunks_exact(n)) {
            for (o, &bv) in out_row.iter_mut().zip(b_row) {
                *o += av * bv;
            }
        }
    }
    Ok(out)
}
fn cpu_softmax(a: &FloatBatch) -> Result<Vec<f32>, ComputeError> {
    let mut out = alloc_zeros(a.data.len())?;
    for (row, out_row) in a.data.chunks_exact(a.cols).zip(out.chunks_exact_mut(a.cols)) {
        let max = row.iter().copied().fold(f32::NEG_INFINITY, f32::max);
        let mut sum = 0.0f32;
        for (o, &x) in out_row.iter_mut().zip(row) {
            *o = (x - max).exp();
            sum += *o;
        }
        for o in out_row.iter_mut() {
            *o /= sum;
        }
    }
    Ok(out)
}
impl BatchCompute for CpuBatchCompute {
    fn kind(&self) -> BackendKind {
        BackendKind::Cpu
    }
    fn run(
        &self,
        kernel: BatchKernel,
        inputs: &[&FloatBatch],
    ) -> Result<FloatBatch, ComputeError> {
        let shape = check_operands(kernel, inputs)?;
        if let Some(done) = trivial_result(kernel, inputs, shape)? {
            return Ok(done);
        }
        let a = inputs[0];
        let data = match kernel {
            BatchKernel::MatMul => cpu_matmul(a, inputs[1])?,
            BatchKernel::Add => {
                a.data.iter().zip(&inputs[1].data).map(|(x, y)| x + y).collect()
            }
            BatchKernel::Mul => {
                a.data.iter().zip(&inputs[1].data).map(|(x, y)| x * y).collect()
            }
            BatchKernel::SoftmaxLastDim => cpu_softmax(a)?,
        };
        Ok(FloatBatch {
            rows: shape.0,
            cols: shape.1,
            data,
        })
    }
}
mod gpu {
    use super::{
        check_operands, trivial_result, BackendKind, BatchCompute, BatchKernel,
        ComputeError, FloatBatch,
    };
    use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::library_adapter::{
        library_kernel_source, AdapterError, LibTensor, LibraryKernelSource,
    };
    use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::target_env::{
        jit_target_for, ComputeCapability,
    };
    use candle_core::{CudaDevice, Device, Tensor};
    use std::panic::{catch_unwind, AssertUnwindSafe};
    /// GPU implementation running the adapter kernels on CUDA device 0.
    pub struct GpuBatchCompute {
        device: Device,
        source: LibraryKernelSource,
    }
    fn unavailable(what: impl std::fmt::Display) -> ComputeError {
        ComputeError::GpuUnavailable(what.to_string())
    }
    fn backend(what: impl std::fmt::Display) -> ComputeError {
        ComputeError::Backend(what.to_string())
    }
    impl From<AdapterError> for ComputeError {
        fn from(e: AdapterError) -> Self {
            backend(e)
        }
    }
    impl GpuBatchCompute {
        /// Fails with `GpuUnavailable` when no compatible GPU can be used.
        pub fn new() -> Result<Self, ComputeError> {
            match catch_unwind(AssertUnwindSafe(Self::try_new)) {
                Ok(r) => r,
                Err(_) => Err(unavailable("panic during gpu initialisation")),
            }
        }
        fn try_new() -> Result<Self, ComputeError> {
            let cuda = CudaDevice::new_with_stream(0).map_err(unavailable)?;
            if let Ok((major, minor)) = cuda.cuda_stream().context().compute_capability()
            {
                let cc = ComputeCapability::new(
                    u32::try_from(major).unwrap_or(0),
                    u32::try_from(minor).unwrap_or(0),
                );
                jit_target_for(cc).map_err(unavailable)?;
            }
            let this = Self {
                device: Device::Cuda(cuda),
                source: library_kernel_source(),
            };
            this.init_test()?;
            Ok(this)
        }
        fn init_test(&self) -> Result<(), ComputeError> {
            let a = FloatBatch::new(1, 1, vec![1.5])?;
            let b = FloatBatch::new(1, 1, vec![2.25])?;
            let out = self
                .exec(BatchKernel::Add, &[&a, &b], (1, 1))
                .map_err(unavailable)?;
            if out.as_slice() != [3.75f32] {
                return Err(unavailable("init test returned a wrong value"));
            }
            Ok(())
        }
        fn upload(&self, batch: &FloatBatch) -> Result<LibTensor, ComputeError> {
            Tensor::from_vec(batch.data.clone(), (batch.rows, batch.cols), &self.device)
                .map(LibTensor::from_lib)
                .map_err(backend)
        }
        fn exec(
            &self,
            kernel: BatchKernel,
            inputs: &[&FloatBatch],
            (rows, cols): (usize, usize),
        ) -> Result<FloatBatch, ComputeError> {
            let tensors = inputs
                .iter()
                .map(|b| self.upload(b))
                .collect::<Result<Vec<_>, _>>()?;
            let entry = self.source.get(kernel.name())?;
            let mut outputs = (entry.run)(&tensors)?;
            if outputs.len() != 1 {
                return Err(
                    backend(
                        format!(
                            "{}: expected 1 result, got {}", kernel.name(), outputs.len()
                        ),
                    ),
                );
            }
            let out = outputs.remove(0);
            if out.dims() != [rows, cols] {
                return Err(
                    backend(
                        format!(
                            "{}: result shape {:?}, expected [{rows}, {cols}]", kernel
                            .name(), out.dims()
                        ),
                    ),
                );
            }
            let bytes = out.to_host_bytes()?;
            let size = std::mem::size_of::<f32>();
            if bytes.len() % size != 0 || bytes.len() / size != rows * cols {
                return Err(
                    backend(
                        format!(
                            "{}: result holds {} bytes, expected {} f32 values", kernel
                            .name(), bytes.len(), rows * cols
                        ),
                    ),
                );
            }
            let data = bytes
                .chunks_exact(size)
                .map(|c| f32::from_ne_bytes([c[0], c[1], c[2], c[3]]))
                .collect();
            FloatBatch::new(rows, cols, data)
        }
    }
    impl BatchCompute for GpuBatchCompute {
        fn kind(&self) -> BackendKind {
            BackendKind::Gpu
        }
        fn run(
            &self,
            kernel: BatchKernel,
            inputs: &[&FloatBatch],
        ) -> Result<FloatBatch, ComputeError> {
            let shape = check_operands(kernel, inputs)?;
            if let Some(done) = trivial_result(kernel, inputs, shape)? {
                return Ok(done);
            }
            match catch_unwind(AssertUnwindSafe(|| self.exec(kernel, inputs, shape))) {
                Ok(r) => r,
                Err(_) => Err(backend("panic inside the gpu library")),
            }
        }
    }
}
pub use gpu::GpuBatchCompute;
/// Returns the GPU implementation when available, otherwise the CPU one. Never fails.
pub fn create_default() -> Box<dyn BatchCompute> {
    match GpuBatchCompute::new() {
        Ok(gpu) => Box::new(gpu),
        Err(_) => Box::new(CpuBatchCompute::new()),
    }
}
#[cfg(test)]
#[path = "test/batch_compute.rs"]
mod tests;
