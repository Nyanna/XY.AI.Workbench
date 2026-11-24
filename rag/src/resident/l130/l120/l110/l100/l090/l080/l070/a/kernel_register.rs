use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Condvar, Mutex};
use std::time::Instant;
use log::{info, warn};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::a::resource_lifecycle::{
    LifecycleError, MemoryClass, ResourceKey, ResourceLoader, ResourceRegistry,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::a::library_adapter::{
    LibKernelEntry, LibraryKernelSource,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::{
    device_lane_event::DeviceHandle, layout::DType, target_env::ComputeCapability,
};
pub const RESOURCE_KEY: &str = "kernel_register";
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArchLevel {
    Sm61,
    Sm70,
    Sm75,
    Sm80,
    Sm86,
    Sm89,
}
impl ArchLevel {
    pub fn capability(self) -> ComputeCapability {
        let (major, minor) = match self {
            ArchLevel::Sm61 => (6, 1),
            ArchLevel::Sm70 => (7, 0),
            ArchLevel::Sm75 => (7, 5),
            ArchLevel::Sm80 => (8, 0),
            ArchLevel::Sm86 => (8, 6),
            ArchLevel::Sm89 => (8, 9),
        };
        ComputeCapability::new(major, minor)
    }
    pub fn jit_arch(self) -> String {
        self.capability().jit_arch()
    }
}
pub fn arch_level_for(cc: ComputeCapability) -> ArchLevel {
    match (cc.major, cc.minor) {
        (6, 1) => ArchLevel::Sm61,
        (7, 0) => ArchLevel::Sm70,
        (7, 5) => ArchLevel::Sm75,
        (8, 0) => ArchLevel::Sm80,
        (8, 6) => ArchLevel::Sm86,
        (8, 9) => ArchLevel::Sm89,
        _ => ArchLevel::Sm75,
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Variant {
    pub dim: Option<u32>,
    pub tile: Option<u32>,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KernelKey {
    pub name: String,
    pub dtype: DType,
    pub arch: ArchLevel,
    pub variant: Variant,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KernelError {
    Unavailable(String),
    UnknownKernel(String),
    CompileError { log: String },
    ModuleLoadError(String),
    MissingFunction { kernel: String, symbol: String },
}
impl fmt::Display for KernelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KernelError::Unavailable(t) => write!(f, "kernel unavailable: {t}"),
            KernelError::UnknownKernel(n) => write!(f, "unknown kernel: {n}"),
            KernelError::CompileError { log } => {
                write!(f, "kernel compilation failed: {log}")
            }
            KernelError::ModuleLoadError(t) => {
                write!(f, "kernel module load failed: {t}")
            }
            KernelError::MissingFunction { kernel, symbol } => {
                write!(f, "kernel {kernel}: symbol {symbol} missing in module")
            }
        }
    }
}
impl std::error::Error for KernelError {}
#[derive(Debug, Clone)]
pub struct KernelSource {
    pub name: &'static str,
    pub body: &'static str,
    pub entry: &'static str,
    pub dtypes: &'static [DType],
    pub precompiled: Option<&'static [(ArchLevel, &'static [u8])]>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeKind {
    Ptx,
    Cubin,
}
#[derive(Debug, Clone)]
pub struct CodeImage {
    pub kind: CodeKind,
    pub bytes: Vec<u8>,
}
#[derive(Debug, Clone)]
pub struct CompileRequest {
    pub file_name: String,
    pub source: String,
    pub arch: String,
    pub kind: CodeKind,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompilerError {
    Unavailable(String),
    Failed { log: String },
}
pub trait KernelCompiler: Send + Sync {
    fn compile(&self, req: &CompileRequest) -> Result<CodeImage, CompilerError>;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadFailure {
    PtxRejected(String),
    MissingSymbol(String),
    Other(String),
}
pub trait KernelFunction: Send + Sync + fmt::Debug {
    fn symbol(&self) -> &str;
    fn as_any(&self) -> &dyn Any;
}
pub trait ModuleLoader: Send + Sync {
    fn device_capability(&self) -> Option<ComputeCapability>;
    fn load(
        &self,
        image: &CodeImage,
        symbol: &str,
    ) -> Result<Arc<dyn KernelFunction>, LoadFailure>;
}
#[derive(Clone, Debug)]
pub struct GpuKernel {
    pub key: KernelKey,
    pub function: Arc<dyn KernelFunction>,
    pub image_kind: CodeKind,
}
/// Score kernel on host memory.
pub trait CpuScoreKernel: Send + Sync {
    /// `rows`: n*dim contiguous f32 values; `out.len() == n`; f32 accumulation.
    fn score(&self, query: &[f32], rows: &[f32], dim: usize, out: &mut [f32]);
}
#[derive(Clone)]
pub enum Kernel {
    Gpu(Arc<GpuKernel>),
    Library(LibKernelEntry),
    Cpu(Arc<dyn CpuScoreKernel>),
}
/// Source of kernels for one sub-engine. The core obtains every kernel through it.
pub trait KernelProvider: Send + Sync {
    fn request(
        &self,
        name: &str,
        dtype: DType,
        variant: Variant,
    ) -> Result<Kernel, KernelError>;
}
impl fmt::Debug for Kernel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Kernel::Gpu(k) => f.debug_tuple("Gpu").field(k).finish(),
            Kernel::Library(e) => f.debug_tuple("Library").field(e).finish(),
            Kernel::Cpu(_) => f.write_str("Cpu"),
        }
    }
}
type BuildResult = Result<Arc<GpuKernel>, KernelError>;
struct Flight {
    result: Mutex<Option<BuildResult>>,
    ready: Condvar,
}
impl Flight {
    fn new() -> Flight {
        Flight {
            result: Mutex::new(None),
            ready: Condvar::new(),
        }
    }
    fn complete(&self, result: BuildResult) {
        *lock(&self.result) = Some(result);
        self.ready.notify_all();
    }
    fn wait(&self) -> BuildResult {
        let mut guard = lock(&self.result);
        loop {
            if let Some(r) = guard.as_ref() {
                return r.clone();
            }
            guard = self.ready.wait(guard).unwrap_or_else(|e| e.into_inner());
        }
    }
}
enum Slot {
    Ready(Arc<GpuKernel>),
    Pending(Arc<Flight>),
}
enum Role {
    Lead(Arc<Flight>),
    Wait(Arc<Flight>),
}
fn dtype_name(dtype: DType) -> &'static str {
    match dtype {
        DType::F32 => "f32",
        DType::F16 => "f16",
        DType::BF16 => "bf16",
        DType::U8 => "u8",
        DType::U32 => "u32",
        DType::I64 => "i64",
    }
}
fn prelude(dtype: DType, variant: Variant) -> String {
    let (include, ty, load) = match dtype {
        DType::F32 => ("", "float", "p[i]"),
        DType::F16 => ("#include <cuda_fp16.h>\n", "__half", "__half2float(p[i])"),
        DType::U8 => ("", "unsigned char", "(float)p[i]"),
        DType::U32 => ("", "unsigned int", "(float)p[i]"),
        DType::I64 => ("", "long long", "(float)p[i]"),
        DType::BF16 => ("", "float", "p[i]"),
    };
    let mut s = String::new();
    s.push_str(include);
    s.push_str(&format!("typedef {ty} T;\n"));
    s.push_str(
        &format!(
            "__device__ __forceinline__ float load_f(const T* p, size_t i) {{ return {load}; }}\n"
        ),
    );
    s.push_str(
        "__device__ __forceinline__ float warp_reduce_sum(float v) {\n\
         \x20 for (int o = 16; o > 0; o >>= 1) v += __shfl_xor_sync(0xffffffffu, v, o);\n\
         \x20 return v;\n}\n\
         __device__ __forceinline__ float warp_reduce_max(float v) {\n\
         \x20 for (int o = 16; o > 0; o >>= 1) v = fmaxf(v, __shfl_xor_sync(0xffffffffu, v, o));\n\
         \x20 return v;\n}\n",
    );
    if let Some(d) = variant.dim {
        s.push_str(&format!("#define DIM {d}\n"));
    }
    if let Some(t) = variant.tile {
        s.push_str(&format!("#define TILE {t}\n"));
    }
    s
}
fn file_name(key: &KernelKey) -> String {
    let mut s = format!(
        "{}_{}_{}", key.name, dtype_name(key.dtype), key.arch.jit_arch()
    );
    if let Some(d) = key.variant.dim {
        s.push_str(&format!("_d{d}"));
    }
    if let Some(t) = key.variant.tile {
        s.push_str(&format!("_t{t}"));
    }
    s.push_str(".cu");
    s
}
/// Lifecycle key of a kernel of the given name and element type.
pub fn kernel_resource_key(name: &str, dtype: DType) -> ResourceKey {
    format!("kernel:{name}:{dtype:?}")
}
pub struct KernelRegister {
    device: DeviceHandle,
    capability: Option<ComputeCapability>,
    compiler: Arc<dyn KernelCompiler>,
    loader: Arc<dyn ModuleLoader>,
    library: LibraryKernelSource,
    sources: Mutex<HashMap<&'static str, KernelSource>>,
    slots: Mutex<HashMap<KernelKey, Slot>>,
}
static_assertions::assert_impl_all!(KernelRegister : Send, Sync);
impl KernelRegister {
    pub fn new(
        device: DeviceHandle,
        compiler: Arc<dyn KernelCompiler>,
        loader: Arc<dyn ModuleLoader>,
        library: LibraryKernelSource,
    ) -> KernelRegister {
        let capability = match device {
            DeviceHandle::Gpu(_) => loader.device_capability(),
            DeviceHandle::Cpu => None,
        };
        KernelRegister {
            device,
            capability,
            compiler,
            loader,
            library,
            sources: Mutex::new(HashMap::new()),
            slots: Mutex::new(HashMap::new()),
        }
    }
    pub fn register_source(&self, src: KernelSource) -> Result<(), KernelError> {
        let mut sources = lock(&self.sources);
        if sources.contains_key(src.name) {
            return Err(
                KernelError::Unavailable(
                    format!("kernel source '{}' is already registered", src.name),
                ),
            );
        }
        sources.insert(src.name, src);
        Ok(())
    }
    pub fn device(&self) -> DeviceHandle {
        self.device
    }
    pub fn capability(&self) -> Option<ComputeCapability> {
        self.capability
    }
    pub fn target_arch(&self) -> Option<ArchLevel> {
        self.capability.map(arch_level_for)
    }
    pub fn request(
        &self,
        name: &str,
        dtype: DType,
        variant: Variant,
    ) -> Result<Kernel, KernelError> {
        let arch = self
            .target_arch()
            .ok_or_else(|| KernelError::Unavailable(
                "no compute capability available".to_string(),
            ));
        match arch {
            Ok(arch) => {
                self.get(
                    &KernelKey {
                        name: name.to_string(),
                        dtype,
                        arch,
                        variant,
                    },
                )
            }
            Err(e) => {
                if lock(&self.sources).contains_key(name) {
                    return Err(e);
                }
                self.library_kernel(name)
            }
        }
    }
    pub fn get(&self, key: &KernelKey) -> Result<Kernel, KernelError> {
        let src = lock(&self.sources).get(key.name.as_str()).cloned();
        let Some(src) = src else {
            return self.library_kernel(&key.name);
        };
        if key.dtype == DType::BF16 || !src.dtypes.contains(&key.dtype) {
            return Err(
                KernelError::Unavailable(
                    format!(
                        "kernel '{}' does not support dtype {}", key.name, dtype_name(key
                        .dtype)
                    ),
                ),
            );
        }
        if self.capability.is_none() {
            return Err(
                KernelError::Unavailable("no compute capability available".to_string()),
            );
        }
        let role = {
            let mut slots = lock(&self.slots);
            match slots.get(key) {
                Some(Slot::Ready(k)) => return Ok(Kernel::Gpu(k.clone())),
                Some(Slot::Pending(f)) => Role::Wait(f.clone()),
                None => {
                    let f = Arc::new(Flight::new());
                    slots.insert(key.clone(), Slot::Pending(f.clone()));
                    Role::Lead(f)
                }
            }
        };
        match role {
            Role::Wait(f) => f.wait().map(Kernel::Gpu),
            Role::Lead(f) => {
                let started = Instant::now();
                let result = self.build(key, &src);
                {
                    let mut slots = lock(&self.slots);
                    match &result {
                        Ok(k) => {
                            slots.insert(key.clone(), Slot::Ready(k.clone()));
                        }
                        Err(_) => {
                            slots.remove(key);
                        }
                    }
                }
                if let Ok(k) = &result {
                    info!(
                        "kernel '{}' dtype={} arch={} variant={:?} ready ({:?}) in {} ms",
                        key.name, dtype_name(key.dtype), key.arch.jit_arch(), key
                        .variant, k.image_kind, started.elapsed().as_millis()
                    );
                }
                f.complete(result.clone());
                result.map(Kernel::Gpu)
            }
        }
    }
    pub fn len(&self) -> usize {
        lock(&self.slots).values().filter(|s| matches!(s, Slot::Ready(_))).count()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn clear(&self) -> usize {
        let mut slots = lock(&self.slots);
        let before = slots.len();
        slots.retain(|_, s| matches!(s, Slot::Pending(_)));
        before - slots.len()
    }
    fn library_kernel(&self, name: &str) -> Result<Kernel, KernelError> {
        self.library
            .get(name)
            .map(|e| Kernel::Library(e.clone()))
            .map_err(|_| KernelError::UnknownKernel(name.to_string()))
    }
    fn compile_guarded(&self, req: &CompileRequest) -> Result<CodeImage, CompilerError> {
        catch_unwind(AssertUnwindSafe(|| self.compiler.compile(req)))
            .unwrap_or_else(|p| {
                Err(CompilerError::Failed {
                    log: format!(
                        "compiler panicked: {}", panic_payload_text(&* p)
                        .unwrap_or("panic with non-text payload")
                    ),
                })
            })
    }
    fn load_guarded(
        &self,
        image: &CodeImage,
        symbol: &str,
    ) -> Result<Arc<dyn KernelFunction>, LoadFailure> {
        catch_unwind(AssertUnwindSafe(|| self.loader.load(image, symbol)))
            .unwrap_or_else(|p| {
                Err(
                    LoadFailure::Other(
                        format!(
                            "loader panicked: {}", panic_payload_text(&* p)
                            .unwrap_or("panic with non-text payload")
                        ),
                    ),
                )
            })
    }
    fn finish_load(
        &self,
        key: &KernelKey,
        src: &KernelSource,
        result: Result<Arc<dyn KernelFunction>, LoadFailure>,
        kind: CodeKind,
    ) -> BuildResult {
        match result {
            Ok(function) => {
                Ok(
                    Arc::new(GpuKernel {
                        key: key.clone(),
                        function,
                        image_kind: kind,
                    }),
                )
            }
            Err(LoadFailure::MissingSymbol(_)) => {
                Err(KernelError::MissingFunction {
                    kernel: key.name.clone(),
                    symbol: src.entry.to_string(),
                })
            }
            Err(LoadFailure::PtxRejected(t)) | Err(LoadFailure::Other(t)) => {
                Err(KernelError::ModuleLoadError(t))
            }
        }
    }
    fn build(&self, key: &KernelKey, src: &KernelSource) -> BuildResult {
        let capability = self
            .capability
            .ok_or_else(|| {
                KernelError::Unavailable("no compute capability available".to_string())
            })?;
        let source = format!("{}{}", prelude(key.dtype, key.variant), src.body);
        let name = file_name(key);
        let ptx_req = CompileRequest {
            file_name: name.clone(),
            source: source.clone(),
            arch: key.arch.jit_arch(),
            kind: CodeKind::Ptx,
        };
        match self.compile_guarded(&ptx_req) {
            Ok(image) => {
                match self.load_guarded(&image, src.entry) {
                    Err(LoadFailure::PtxRejected(reason)) => {
                        warn!(
                            "kernel '{}' PTX rejected by driver ({reason}); retrying with cubin for {}",
                            key.name, capability.jit_arch()
                        );
                        let cubin_req = CompileRequest {
                            file_name: name,
                            source,
                            arch: capability.jit_arch(),
                            kind: CodeKind::Cubin,
                        };
                        let image = match self.compile_guarded(&cubin_req) {
                            Ok(i) => i,
                            Err(CompilerError::Unavailable(t)) => {
                                return Err(KernelError::ModuleLoadError(t));
                            }
                            Err(CompilerError::Failed { log }) => {
                                return Err(KernelError::ModuleLoadError(log));
                            }
                        };
                        let loaded = self.load_guarded(&image, src.entry);
                        self.finish_load(key, src, loaded, CodeKind::Cubin)
                    }
                    other => self.finish_load(key, src, other, CodeKind::Ptx),
                }
            }
            Err(CompilerError::Failed { log }) => Err(KernelError::CompileError { log }),
            Err(CompilerError::Unavailable(text)) => {
                let bytes = src
                    .precompiled
                    .and_then(|set| set.iter().find(|(a, _)| *a == key.arch))
                    .map(|(_, b)| *b);
                match bytes {
                    Some(bytes) => {
                        warn!(
                            "kernel '{}': compiler unavailable ({text}); using precompiled image",
                            key.name
                        );
                        let image = CodeImage {
                            kind: CodeKind::Cubin,
                            bytes: bytes.to_vec(),
                        };
                        let loaded = self.load_guarded(&image, src.entry);
                        self.finish_load(key, src, loaded, CodeKind::Cubin)
                    }
                    None => Err(KernelError::ModuleLoadError(text)),
                }
            }
        }
    }
}
impl KernelProvider for KernelRegister {
    fn request(
        &self,
        name: &str,
        dtype: DType,
        variant: Variant,
    ) -> Result<Kernel, KernelError> {
        KernelRegister::request(self, name, dtype, variant)
    }
}
struct FactoryLoader {
    factory: Arc<dyn Fn() -> Result<KernelRegister, String> + Send + Sync>,
}
impl ResourceLoader<KernelRegister> for FactoryLoader {
    fn load(&self) -> Result<KernelRegister, String> {
        (self.factory)()
    }
}
pub fn register_resource(
    registry: &ResourceRegistry<KernelRegister>,
    factory: Arc<dyn Fn() -> Result<KernelRegister, String> + Send + Sync>,
) -> Result<(), LifecycleError> {
    registry
        .register(
            RESOURCE_KEY.to_string(),
            MemoryClass::Vram,
            Arc::new(FactoryLoader { factory }),
        )
}
#[cfg(test)]
#[path = "test/kernel_register.rs"]
mod tests;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::sync_util::lock;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::sync_util::panic_payload_text;
