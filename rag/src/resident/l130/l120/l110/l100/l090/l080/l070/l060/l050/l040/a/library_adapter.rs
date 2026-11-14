use std::fmt;
use std::sync::{Arc, OnceLock};
use candle_core::{CudaDevice, Tensor};
use super::super::l030::a::region_handle::RegionHandle;
use super::super::l030::l020::l010::layout::{DType, Layout};
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    UnsupportedLibDType(String),
    NotElementMultiple { bytes: u64, elem_size: usize },
    Overflow,
    OutOfBounds { offset: u64, len: u64, block_len: u64 },
    LayoutExceedsRegion { required: u64, region_len: u64 },
    NotContiguous,
    Misaligned { addr: u64, align: usize },
    Mismatch(String),
    UnknownKernel(String),
    Library(String),
}
impl fmt::Display for AdapterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdapterError::UnsupportedLibDType(d) => {
                write!(f, "library dtype {d} has no engine equivalent")
            }
            AdapterError::NotElementMultiple { bytes, elem_size } => {
                write!(f, "{bytes} bytes is not a multiple of element size {elem_size}")
            }
            AdapterError::Overflow => write!(f, "size computation overflows"),
            AdapterError::OutOfBounds { offset, len, block_len } => {
                write!(f, "region [{offset}, +{len}) exceeds block of {block_len} bytes")
            }
            AdapterError::LayoutExceedsRegion { required, region_len } => {
                write!(f, "layout needs {required} bytes but region holds {region_len}")
            }
            AdapterError::NotContiguous => write!(f, "layout is not contiguous"),
            AdapterError::Misaligned { addr, align } => {
                write!(f, "device address {addr:#x} is not aligned to {align} bytes")
            }
            AdapterError::Mismatch(m) => write!(f, "operand mismatch: {m}"),
            AdapterError::UnknownKernel(n) => write!(f, "unknown kernel '{n}'"),
            AdapterError::Library(m) => write!(f, "library error: {m}"),
        }
    }
}
impl std::error::Error for AdapterError {}
fn lib_err(e: candle_core::Error) -> AdapterError {
    AdapterError::Library(e.to_string())
}
pub fn to_lib_dtype(d: DType) -> candle_core::DType {
    match d {
        DType::F32 => candle_core::DType::F32,
        DType::F16 => candle_core::DType::F16,
        DType::BF16 => candle_core::DType::BF16,
        DType::U8 => candle_core::DType::U8,
        DType::U32 => candle_core::DType::U32,
        DType::I64 => candle_core::DType::I64,
    }
}
pub fn from_lib_dtype(d: candle_core::DType) -> Result<DType, AdapterError> {
    match d {
        candle_core::DType::F32 => Ok(DType::F32),
        candle_core::DType::F16 => Ok(DType::F16),
        candle_core::DType::BF16 => Ok(DType::BF16),
        candle_core::DType::U8 => Ok(DType::U8),
        candle_core::DType::U32 => Ok(DType::U32),
        candle_core::DType::I64 => Ok(DType::I64),
        other => Err(AdapterError::UnsupportedLibDType(format!("{other:?}"))),
    }
}
pub fn elems_to_bytes(dtype: DType, elems: u64) -> Result<u64, AdapterError> {
    elems.checked_mul(dtype.size_in_bytes() as u64).ok_or(AdapterError::Overflow)
}
pub fn bytes_to_elems(dtype: DType, bytes: u64) -> Result<u64, AdapterError> {
    let elem_size = dtype.size_in_bytes();
    if bytes % elem_size as u64 != 0 {
        return Err(AdapterError::NotElementMultiple {
            bytes,
            elem_size,
        });
    }
    Ok(bytes / elem_size as u64)
}
#[derive(Debug, Clone)]
pub struct DeviceBlock {
    pub device: CudaDevice,
    pub base_addr: u64,
    pub len_bytes: u64,
}
#[derive(Debug, Clone)]
pub struct LibTensor {
    tensor: Tensor,
    borrowed: bool,
}
impl LibTensor {
    pub fn lib(&self) -> &Tensor {
        &self.tensor
    }
    pub fn into_lib(self) -> Tensor {
        self.tensor
    }
    pub fn from_lib(t: Tensor) -> LibTensor {
        LibTensor {
            tensor: t,
            borrowed: false,
        }
    }
    pub fn dtype(&self) -> Result<DType, AdapterError> {
        from_lib_dtype(self.tensor.dtype())
    }
    pub fn dims(&self) -> &[usize] {
        self.tensor.dims()
    }
    pub fn is_borrowed(&self) -> bool {
        self.borrowed
    }
    pub fn to_owned_copy(&self) -> Result<LibTensor, AdapterError> {
        let src = if self.tensor.is_contiguous() {
            self.tensor.clone()
        } else {
            self.tensor.contiguous().map_err(lib_err)?
        };
        let dst = Tensor::zeros(src.dims(), src.dtype(), src.device()).map_err(lib_err)?;
        copy_into(&dst, &src)?;
        Ok(LibTensor::from_lib(dst))
    }
    pub fn write_from(&self, src: &LibTensor) -> Result<(), AdapterError> {
        if self.tensor.dtype() != src.tensor.dtype() {
            return Err(
                AdapterError::Mismatch(
                    format!(
                        "dtype {:?} vs {:?}", self.tensor.dtype(), src.tensor.dtype()
                    ),
                ),
            );
        }
        if self.tensor.dims() != src.tensor.dims() {
            return Err(
                AdapterError::Mismatch(
                    format!("shape {:?} vs {:?}", self.tensor.dims(), src.tensor.dims()),
                ),
            );
        }
        if !self.tensor.device().same_device(src.tensor.device()) {
            return Err(
                AdapterError::Mismatch("operands live on different devices".into()),
            );
        }
        if !self.tensor.is_contiguous() {
            return Err(AdapterError::Mismatch("destination is not contiguous".into()));
        }
        copy_into(&self.tensor, &src.tensor)
    }
    pub fn to_host_bytes(&self) -> Result<Vec<u8>, AdapterError> {
        if self.tensor.elem_count() == 0 {
            return Ok(Vec::new());
        }
        let flat = self.tensor.flatten_all().map_err(lib_err)?;
        macro_rules! bytes_of {
            ($t:ty) => {
                flat.to_vec1::<$t > ().map_err(lib_err) ? .into_iter().flat_map(| x | x
                .to_ne_bytes()).collect()
            };
        }
        let bytes: Vec<u8> = match self.tensor.dtype() {
            candle_core::DType::U8 => flat.to_vec1::<u8>().map_err(lib_err)?,
            candle_core::DType::U32 => bytes_of!(u32),
            candle_core::DType::I64 => bytes_of!(i64),
            candle_core::DType::F32 => bytes_of!(f32),
            candle_core::DType::F16 => bytes_of!(half::f16),
            candle_core::DType::BF16 => bytes_of!(half::bf16),
            other => return Err(AdapterError::UnsupportedLibDType(format!("{other:?}"))),
        };
        Ok(bytes)
    }
}
fn copy_into(dst: &Tensor, src: &Tensor) -> Result<(), AdapterError> {
    if dst.elem_count() == 0 {
        return Ok(());
    }
    let d = dst.flatten_all().map_err(lib_err)?;
    let s = src.flatten_all().map_err(lib_err)?;
    d.slice_set(&s, 0, 0).map_err(lib_err)
}
fn checked_elem_count(layout: &Layout) -> Result<u64, AdapterError> {
    if layout.shape().contains(&0) {
        return Ok(0);
    }
    layout
        .shape()
        .iter()
        .try_fold(1u64, |acc, &d| acc.checked_mul(d as u64))
        .ok_or(AdapterError::Overflow)
}
fn validate_wrap(
    block_len: u64,
    base_addr: u64,
    region: &RegionHandle,
    layout: &Layout,
) -> Result<(), AdapterError> {
    let offset = region.offset();
    let len = region.len();
    let end = offset.checked_add(len).ok_or(AdapterError::Overflow)?;
    if end > block_len {
        return Err(AdapterError::OutOfBounds {
            offset,
            len,
            block_len,
        });
    }
    let elems = checked_elem_count(layout)?;
    if elems == 0 {
        return Ok(());
    }
    if !layout.is_contiguous() {
        return Err(AdapterError::NotContiguous);
    }
    let required = elems_to_bytes(layout.dtype(), elems)?;
    if required > len {
        return Err(AdapterError::LayoutExceedsRegion {
            required,
            region_len: len,
        });
    }
    let addr = base_addr.checked_add(offset).ok_or(AdapterError::Overflow)?;
    let align = layout.dtype().size_in_bytes();
    if addr % align as u64 != 0 {
        return Err(AdapterError::Misaligned {
            addr,
            align,
        });
    }
    Ok(())
}
/// # Safety
/// The block and its context must outlive the tensor, every view and every reference to it.
/// The caller has already verified the generation and validity of the handle.
pub unsafe fn wrap_region(
    block: &DeviceBlock,
    region: &RegionHandle,
    layout: &Layout,
) -> Result<LibTensor, AdapterError> {
    validate_wrap(block.len_bytes, block.base_addr, region, layout)?;
    let addr = block.base_addr + region.offset();
    let tensor = wrap_device_memory(&block.device, addr, layout)?;
    Ok(LibTensor {
        tensor,
        borrowed: true,
    })
}
unsafe fn wrap_device_memory(
    device: &CudaDevice,
    addr: u64,
    layout: &Layout,
) -> Result<Tensor, AdapterError> {
    use candle_core::cuda_backend::CudaStorageSlice;
    use candle_core::op::BackpropOp;
    use candle_core::{CudaStorage, Shape, Storage};
    let stream = device.cuda_stream();
    let ptr = addr as cudarc::driver::sys::CUdeviceptr;
    let n = layout.elem_count();
    let slice = match layout.dtype() {
        DType::F32 => CudaStorageSlice::F32(stream.upgrade_device_ptr::<f32>(ptr, n)),
        DType::F16 => {
            CudaStorageSlice::F16(stream.upgrade_device_ptr::<half::f16>(ptr, n))
        }
        DType::BF16 => {
            CudaStorageSlice::BF16(stream.upgrade_device_ptr::<half::bf16>(ptr, n))
        }
        DType::U8 => CudaStorageSlice::U8(stream.upgrade_device_ptr::<u8>(ptr, n)),
        DType::U32 => CudaStorageSlice::U32(stream.upgrade_device_ptr::<u32>(ptr, n)),
        DType::I64 => CudaStorageSlice::I64(stream.upgrade_device_ptr::<i64>(ptr, n)),
    };
    let storage = Storage::Cuda(CudaStorage {
        slice,
        device: device.clone(),
    });
    let shape = Shape::from(layout.shape().to_vec());
    let tensor = Tensor::from_storage(storage, shape, BackpropOp::none(), false);
    std::mem::forget(tensor.clone());
    Ok(tensor)
}
pub type LibKernelFn = Arc<
    dyn Fn(&[LibTensor]) -> Result<Vec<LibTensor>, AdapterError> + Send + Sync,
>;
#[derive(Clone)]
pub struct LibKernelEntry {
    pub name: &'static str,
    pub arity: usize,
    pub run: LibKernelFn,
}
impl fmt::Debug for LibKernelEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LibKernelEntry")
            .field("name", &self.name)
            .field("arity", &self.arity)
            .finish()
    }
}
#[derive(Clone, Debug)]
pub struct LibraryKernelSource {
    entries: Vec<LibKernelEntry>,
}
impl LibraryKernelSource {
    pub fn source_name(&self) -> &'static str {
        "candle"
    }
    pub fn kernels(&self) -> &[LibKernelEntry] {
        &self.entries
    }
    pub fn get(&self, name: &str) -> Result<&LibKernelEntry, AdapterError> {
        self.entries
            .iter()
            .find(|e| e.name == name)
            .ok_or_else(|| AdapterError::UnknownKernel(name.to_string()))
    }
}
pub fn library_kernel_source() -> LibraryKernelSource {
    static SOURCE: OnceLock<LibraryKernelSource> = OnceLock::new();
    SOURCE.get_or_init(build_source).clone()
}
fn build_source() -> LibraryKernelSource {
    fn entry(
        name: &'static str,
        arity: usize,
        f: fn(&[LibTensor]) -> Result<LibTensor, AdapterError>,
    ) -> LibKernelEntry {
        let run: LibKernelFn = Arc::new(move |args: &[LibTensor]| {
            if args.len() != arity {
                return Err(
                    AdapterError::Mismatch(
                        format!("{name}: expected {arity} operands, got {}", args.len()),
                    ),
                );
            }
            f(args).map(|t| vec![t])
        });
        LibKernelEntry { name, arity, run }
    }
    LibraryKernelSource {
        entries: vec![
            entry("matmul", 2, kernel_matmul), entry("add", 2, kernel_add), entry("mul",
            2, kernel_mul), entry("softmax_last_dim", 1, kernel_softmax_last_dim),
        ],
    }
}
fn is_float(d: candle_core::DType) -> bool {
    matches!(
        d, candle_core::DType::F32 | candle_core::DType::F16 | candle_core::DType::BF16
    )
}
fn check_supported(name: &str, t: &LibTensor) -> Result<(), AdapterError> {
    from_lib_dtype(t.tensor.dtype())
        .map(|_| ())
        .map_err(|e| AdapterError::Mismatch(format!("{name}: {e}")))
}
fn check_pair(name: &str, a: &LibTensor, b: &LibTensor) -> Result<(), AdapterError> {
    check_supported(name, a)?;
    if a.tensor.dtype() != b.tensor.dtype() {
        return Err(
            AdapterError::Mismatch(
                format!("{name}: dtype {:?} vs {:?}", a.tensor.dtype(), b.tensor.dtype()),
            ),
        );
    }
    if !a.tensor.device().same_device(b.tensor.device()) {
        return Err(
            AdapterError::Mismatch(format!("{name}: operands live on different devices")),
        );
    }
    Ok(())
}
fn check_same_shape(
    name: &str,
    a: &LibTensor,
    b: &LibTensor,
) -> Result<(), AdapterError> {
    check_pair(name, a, b)?;
    if a.tensor.dims() != b.tensor.dims() {
        return Err(
            AdapterError::Mismatch(
                format!("{name}: shape {:?} vs {:?}", a.tensor.dims(), b.tensor.dims()),
            ),
        );
    }
    Ok(())
}
fn kernel_matmul(args: &[LibTensor]) -> Result<LibTensor, AdapterError> {
    let (a, b) = (&args[0], &args[1]);
    check_pair("matmul", a, b)?;
    if !is_float(a.tensor.dtype()) {
        return Err(
            AdapterError::Mismatch(
                format!("matmul: unsupported dtype {:?}", a.tensor.dtype()),
            ),
        );
    }
    let (da, db) = (a.tensor.dims(), b.tensor.dims());
    let rank = da.len();
    if rank < 2 || db.len() != rank || da[..rank - 2] != db[..rank - 2]
        || da[rank - 1] != db[rank - 2]
    {
        return Err(AdapterError::Mismatch(format!("matmul: shapes {da:?} x {db:?}")));
    }
    a.tensor.matmul(&b.tensor).map(LibTensor::from_lib).map_err(lib_err)
}
fn kernel_add(args: &[LibTensor]) -> Result<LibTensor, AdapterError> {
    let (a, b) = (&args[0], &args[1]);
    check_same_shape("add", a, b)?;
    (&a.tensor + &b.tensor).map(LibTensor::from_lib).map_err(lib_err)
}
fn kernel_mul(args: &[LibTensor]) -> Result<LibTensor, AdapterError> {
    let (a, b) = (&args[0], &args[1]);
    check_same_shape("mul", a, b)?;
    (&a.tensor * &b.tensor).map(LibTensor::from_lib).map_err(lib_err)
}
fn kernel_softmax_last_dim(args: &[LibTensor]) -> Result<LibTensor, AdapterError> {
    let x = &args[0];
    check_supported("softmax_last_dim", x)?;
    if !is_float(x.tensor.dtype()) || x.tensor.rank() == 0 {
        return Err(
            AdapterError::Mismatch(
                format!(
                    "softmax_last_dim: dtype {:?}, rank {}", x.tensor.dtype(), x.tensor
                    .rank()
                ),
            ),
        );
    }
    if x.tensor.elem_count() == 0 {
        return x.to_owned_copy();
    }
    let contiguous = x.tensor.contiguous().map_err(lib_err)?;
    candle_nn::ops::softmax_last_dim(&contiguous)
        .map(LibTensor::from_lib)
        .map_err(lib_err)
}
#[cfg(test)]
#[path = "test/library_adapter.rs"]
mod tests;
