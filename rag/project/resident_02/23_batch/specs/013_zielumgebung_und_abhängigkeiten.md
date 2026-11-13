# Spec: Zielumgebung und Abhängigkeiten (build order 13)

## 1. Task, restrictions and boundaries

**Task.** Fix the target environment and the dependency and format decisions as a small, pure-Rust module. The module offers constants, a compute-capability helper and error types that other modules use to stay consistent with the target. It contains no GPU code and no I/O.

**In scope**
- Target hardware: Nvidia GeForce GTX 1660 SUPER, Compute Capability 7.5 (Turing), driver 610.57.04. This card has no Tensor Cores.
- CUDA toolchain: release 13.3, version 13.3.73, build 38244171_0.
- GPU kernels are JIT-compiled at runtime for the architecture actually present. On the reference card that is SM 7.5.
- No Tensor-Core path and no BF16 compute path are offered.
- Turing is sufficient for loads with acquire semantics, which are available from CC 7.0.
- Dependencies are already vetted and entered in `/home/user/xyan/xy.ai.workbench/rag/Cargo.toml`. The external tensor/NN library is **Candle**.
- Model weights use **safetensors**. Persistent indexes use **Parquet** with an **Arrow schema**.

**Out of scope and forbidden**
- Do not add, remove or re-version dependencies. Do not edit `Cargo.toml`.
- Do not implement kernels, a tensor-core path, a BF16 compute path, an AOT per-arch cubin set, or a fallback for other architectures.
- Do not read or write safetensors or Parquet files. Other modules do that.
- Do not call the CUDA driver or runtime. Do not query hardware.

## 2. Context

This module has no dependencies and is a foundation for modules that run kernels, load weights or persist indexes. Those modules rely on the following facts:
- The GPU is Turing (CC 7.5) with FP32 and FP16 CUDA cores only. There is no BF16 compute and no Tensor-Core path.
- Kernels are compiled at runtime for the detected architecture.
- Acquire loads are usable.
- Candle is the only tensor/NN library.
- Weights are safetensors. Indexes are Parquet with an Arrow schema.

## 3. Interface

```rust
//! module: target_env
use std::fmt;

pub const GPU_NAME: &str = "NVIDIA GeForce GTX 1660 SUPER";
pub const DRIVER_VERSION: &str = "610.57.04";
pub const CUDA_RELEASE: &str = "13.3";
pub const CUDA_VERSION: &str = "13.3.73";
pub const CUDA_BUILD: &str = "38244171_0";
pub const CARGO_MANIFEST_PATH: &str = "/home/user/xyan/xy.ai.workbench/rag/Cargo.toml";

/// Compute capability (major, minor).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComputeCapability { pub major: u32, pub minor: u32 }

pub const REFERENCE_CC: ComputeCapability = ComputeCapability { major: 7, minor: 5 };
/// First CC with acquire-semantics loads.
pub const MIN_ACQUIRE_CC: ComputeCapability = ComputeCapability { major: 7, minor: 0 };

/// Tensor-Core path offered? Always false.
pub const TENSOR_CORE_PATH: bool = false;
/// BF16 as compute path offered? Always false.
pub const BF16_COMPUTE: bool = false;

pub const WEIGHTS_FORMAT: &str = "safetensors";
pub const INDEX_FORMAT: &str = "parquet";   // columnar, Arrow schema
pub const TENSOR_LIBRARY: &str = "candle";

impl ComputeCapability {
    pub const fn new(major: u32, minor: u32) -> Self;
    /// JIT target string, e.g. (7,5) -> "sm_75".
    pub fn jit_arch(&self) -> String;
    /// true iff self >= MIN_ACQUIRE_CC.
    pub fn supports_acquire_loads(&self) -> bool;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetEnvError {
    /// Detected CC is below the minimum (7.0).
    UnsupportedCapability(ComputeCapability),
    /// A Tensor-Core or BF16-compute path was requested.
    PathNotOffered(&'static str),
}
impl fmt::Display for TargetEnvError { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result; }
impl std::error::Error for TargetEnvError {}

/// Validates a detected CC and returns the JIT arch string for it.
pub fn jit_target_for(detected: ComputeCapability) -> Result<String, TargetEnvError>;

/// Always Err(PathNotOffered("tensor_core")).
pub fn require_tensor_core_path() -> Result<(), TargetEnvError>;
/// Always Err(PathNotOffered("bf16_compute")).
pub fn require_bf16_compute() -> Result<(), TargetEnvError>;
```

`ComputeCapability`, `TargetEnvError` and the constants are `Send + Sync + 'static`.

## 4. Behavior and invariants

- `ComputeCapability::new(7, 5) == REFERENCE_CC`.
- `jit_arch` returns `"sm_{major}{minor}"`. `(7,5)` gives `"sm_75"`, and `(8,6)` gives `"sm_86"`.
- `supports_acquire_loads` is true for `(7,0)`, `(7,5)` and `(8,0)`. It is false for `(6,1)`.
- `jit_target_for` behaves as follows:
  - It returns `Ok("sm_75")` for `(7,5)`.
  - It returns `Ok` with the detected arch string for any CC >= 7.0. It never substitutes SM 7.5 for the detected architecture.
  - It returns `Err(UnsupportedCapability(cc))` for CC < 7.0.
- `require_tensor_core_path()` and `require_bf16_compute()` always return `Err(PathNotOffered(..))` with the names given above. `TENSOR_CORE_PATH` and `BF16_COMPUTE` are `false`.
- All functions are pure, deterministic, allocation-light and thread-safe. They have no global state and never panic.
- `Display` output is non-empty and includes the CC as `major.minor` for `UnsupportedCapability`, and the path name for `PathNotOffered`.
- Constants hold exactly the values above (verbatim strings).
- The module does not enforce the dependency set. The Cargo manifest at `CARGO_MANIFEST_PATH` is authoritative, and implementers of other modules use only crates already listed there.

## 5. Relied-on elements

None. This module has no dependencies.