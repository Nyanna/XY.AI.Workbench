# Specification: Capability-Erkennung (build order 104)

## 1. Task, restrictions and boundaries
Implement in Rust the detection of a device's capabilities. Detection is by device kind (CPU or GPU). It produces a `CapabilitySet` that covers:
- precisions
- INT8
- fusion
- numeric axes
- thread counts

The set is then reduced to the kernels that exist.

Restrictions:
- Use the `log` crate. Use only `std` for CPU feature detection (`is_x86_feature_detected!` under `cfg(any(target_arch = "x86", target_arch = "x86_64"))`).
- Do not link any GPU runtime. GPU facts come through the `GpuProbe` trait, which the GPU binding implements.
- No `unsafe`, no global state, no I/O besides logging.
- Not part of this module:
  - Loading or compiling kernels.
  - Scheduling.
  - Reading configuration files. The `Konfiguration` module is not specified yet, so this module defines its own plain input struct, `CapabilityConfig`.
  - Profile names.
  - A Tensor-Core capability. There is no Tensor-Core path, so Tensor Cores are only logged.

## 2. Context
- Backends and the facade call `detect_capabilities` once per device. The result is the capability set that the device offers.
- The result uses the Capability-Modell (`CapabilitySet`, `axes::*`, `values::*`). Its reference sets (`cpu_reference()`, `gpu_reference()`) are upper bounds for the precision, INT8 and fusion axes.
- "Unterstützte Precisions" is the source of truth for which dtypes have scoring kernels (`precision_capabilities`).
- The Kernel-Register has no listing API. The built-in kernel catalog is therefore defined here (section 4).
- The reference GPU is a GTX 1660 SUPER (CC 7.5, TU116). It has no Tensor Cores.

## 3. Interface
```rust
use std::sync::Arc;
use capability::{CapabilitySet, Axis, Value, axes, values};   // Capability-Modell
use device_lane_event::{DeviceHandle, DeviceKind};
use target_env::ComputeCapability;                            // {major, minor}, Copy, Ord, ComputeCapability::new(u32,u32)

/// Supplied by the GPU binding. Bound to one device context. Both methods are cheap.
pub trait GpuProbe: Send + Sync {
    /// None if the device context is not available.
    fn compute_capability(&self) -> Option<ComputeCapability>;
    /// Marketing or chip name, e.g. "NVIDIA GeForce GTX 1660 SUPER". None if unknown.
    fn device_name(&self) -> Option<String>;
}

/// Input from the configuration. None = use the default.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CapabilityConfig {
    pub batch_sizes: Option<Vec<u32>>,   // default [1, 8, 32]
    pub tile_sizes: Option<Vec<u32>>,    // default [16, 32, 64, 128, 256, 512]
    pub max_seq_len: Option<u32>,        // default 512
    pub kernels: Option<CapabilitySet>,  // given kernel set; None = built-in catalog
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CpuFeatures { pub avx2: bool, pub f16c: bool, pub fma: bool, pub vnni: bool }

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetectedCapabilities {
    pub capabilities: CapabilitySet,   // final, reduced
    pub max_seq_len: u32,              // effective maximum sequence length
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DetectError {
    NoGpuSupport,              // device is a GPU, but no GpuProbe was given
    NoDeviceContext,           // probe exists but compute_capability() is None
    UnsupportedDevice(String), // unknown future DeviceKind
}
impl std::fmt::Display for DetectError {}
impl std::error::Error for DetectError {}

/// Central entry point. Dispatches on device.kind(). `gpu` is only read for GPU devices.
pub fn detect_capabilities(
    device: DeviceHandle,
    gpu: Option<&dyn GpuProbe>,
    cfg: &CapabilityConfig,
) -> Result<DetectedCapabilities, DetectError>;

// Building blocks (pub, pure unless noted, for tests):
pub fn detect_cpu_features() -> CpuFeatures;                       // queries the system
pub fn logical_cores() -> usize;                                   // available_parallelism, fallback 1
pub fn cpu_capabilities(f: CpuFeatures, logical_cores: usize) -> CapabilitySet;
pub fn gpu_capabilities(cc: ComputeCapability) -> CapabilitySet;
pub fn has_tensor_cores(cc: ComputeCapability, device_name: Option<&str>) -> bool;
pub fn thread_counts(logical_cores: usize) -> Vec<u32>;
pub fn builtin_catalog(kind: DeviceKind) -> CapabilitySet;
pub fn reduce_to_kernels(caps: CapabilitySet, kernels: &CapabilitySet, subengine: &str) -> CapabilitySet;
```
All items are `Send + Sync`. There is no interior mutability.

## 4. Behavior and invariants

**Flow of `detect_capabilities`**
1. Match `device.kind()`:
   - `Cpu` → `cpu_capabilities(detect_cpu_features(), logical_cores())`.
   - `Gpu` → GPU detection (below).
   - Any other kind → `Err(UnsupportedDevice)`. The match needs a wildcard arm because `DeviceKind` is `#[non_exhaustive]`.
2. Add the numeric axes.
3. Choose the kernel set: `cfg.kernels` if `Some`, else `builtin_catalog(kind)`.
4. Call `reduce_to_kernels`.
5. Log the result at `info!`. The log line must contain the subengine (`"cpu"` or `"gpu"`), the axis count and the total value count, for example `capabilities subengine=gpu axes=7 values=18`.

**CPU (`cpu_capabilities`)**
- `operands` always has `fp32`. It also has `fp16_storage_fp32_compute` iff `f16c`.
- `scoring` always has `fp32`. It has `int8` iff `avx2 || vnni`. It has `int8_vnni` iff `vnni`.
- `thread_count` has the values from `thread_counts(logical_cores)`.
- There are no `accumulator`, `fusion`, `batch_size` or `tile_size` values here. `fma` is only reported in `CpuFeatures`, and no capability depends on it.
- `detect_cpu_features`:
  - On x86 and x86_64, `avx2`, `f16c` and `fma` come from `is_x86_feature_detected!`.
  - `vnni` is true iff `avx512vnni` or `avxvnni` is detected. `avxvnni` counts only if the toolchain supports querying it.
  - On any other architecture, all four are false.
- `thread_counts(n)`:
  - Let `n' = max(n, 1)`.
  - The result is the ascending, duplicate-free list of all powers of two ≤ `n'`, plus `n'`.
  - Examples: 1→[1], 6→[1,2,4,6], 8→[1,2,4,8], 0→[1].
  - Each value becomes a `Name` via `Name::from_number`.

**GPU**
- With `gpu == None`, return `Err(NoGpuSupport)`.
- If `compute_capability()` returns `None`, return `Err(NoDeviceContext)`. No other errors occur.
- `gpu_capabilities(cc)`:
  - `operands`, `accumulator` and `scoring` always have `fp32`. They also have `fp16` iff `cc >= 5.3`.
  - `scoring` also has `int8_dp4a` iff `cc >= 6.1`.
  - `fusion` is always `{on, off}`.
  - The result is within `gpu_reference()`.
- `has_tensor_cores(cc, name)`:
  - It is true iff `cc.major >= 7` and the name does not match an exclusion.
  - The exclusion match is case-insensitive. A name is excluded if it contains `TU116`, `TU117`, `GTX 16` or `GTX16`. This covers TU116/TU117 even though they report SM 7.x.
  - A missing name means no exclusion.
  - `has_tensor_cores(7.5, "NVIDIA GeForce GTX 1660 SUPER") == false`.
  - Detection only calls `info!("tensor cores: yes/no", cc, name)`. It never adds a capability, so no Tensor-Core value or axis appears.
- No `thread_count` axis is produced for a GPU.

**Numeric axes (both device kinds)**
- `max_seq_len` = `cfg.max_seq_len` or 512. It is returned in `DetectedCapabilities` and is not a capability axis.
- `batch_size` = the values of `cfg.batch_sizes` (default 1, 8, 32), keeping only values > 0.
- `tile_size` = the values of `cfg.tile_sizes` (default the powers of two 16 to 512), keeping only values with `0 < t <= max_seq_len`.
- Dropped values are logged at `warn!`.
- Duplicates are merged by set semantics.
- An axis left with no values is not created. This can only happen with invalid or empty config, and it is not an error.
- Values are `Name::from_number(v as u64)`.
- Config tile values are not forced to be powers of two. Only the stated filters apply.

**Built-in catalog (`builtin_catalog`)**
- The catalog is `precision_capabilities(kind)` (axes `operands` and `scoring`) plus these extras:
  - Cpu: `operands` + `fp16_storage_fp32_compute`; `scoring` + `int8` and `int8_vnni`.
  - Gpu: `scoring` + `int8_dp4a`; `accumulator` {fp32, fp16}; `fusion` {on, off}.
- The catalog has no numeric axes and no `thread_count`.
- Consequence: on the CPU, `fp16_storage_fp32_compute` is kept even though the precision table has no CPU F16 scoring kernel.

**Reduction (`reduce_to_kernels(caps, kernels, subengine)`)**
- The "catalog axes" are the axes of `kernels`.
- For each catalog axis, `caps[axis]` is replaced by `caps[axis] ∩ kernels[axis]`.
- If the intersection is empty, the axis is removed completely.
- Axes of `caps` that are not in `kernels` (unknown axes, e.g. numeric ones with the built-in catalog) stay unchanged.
- Axes that exist only in `kernels` are never added.
- Every removed value or axis is logged at `info!` or `warn!` with the subengine, axis and value.
- A given kernel set (`cfg.kernels`) fully replaces the built-in catalog, including for numeric axes if it names them.
- Reduction never adds anything. The result is a subset of `caps`.

**Invariants**
- On the reference GPU (CC 7.5), the result has no Tensor-Core entry. It contains `fp32`, `fp16`, `int8_dp4a`, `on` and `off` (with the default catalog).
- The CPU always contains `operands:fp32` and `scoring:fp32`. This holds even for `CpuFeatures::default()` and with 0 cores. It can only be removed if the given `cfg.kernels` lacks it.
- The result is a valid `CapabilitySet`, with no empty axis.
- The function is deterministic for fixed inputs, pure apart from logging and system queries, and thread-safe. Concurrent calls are independent.
- Nothing panics.

## 5. Relied-on elements
- **Capability-Modell**:
  - `CapabilitySet::{new, add(&mut self, axis: impl Into<Axis>, value: impl Into<Value>) -> bool, with, contains, values(&Axis) -> Option<&BTreeSet<Value>>, axes(), iter(), is_empty, is_within}`. An axis exists only with at least one value.
  - `Name::from_number(u64)`.
  - `axes::{operands, accumulator, scoring, fusion, batch_size, tile_size, thread_count}`.
  - `values::{fp32, fp16, fp16_storage_fp32_compute, int8, int8_dp4a, int8_vnni, on, off}`.
  - `gpu_reference()` and `cpu_reference()` as upper bounds.
- **Unterstützte Precisions**: `precision_capabilities(kind: DeviceKind) -> CapabilitySet`.
  - Cpu = {operands:{fp32}, scoring:{fp32}}.
  - Gpu = {operands:{fp32,fp16}, scoring:{fp32,fp16}}.
  - Unknown kinds give an empty set.
- **Device und Lane und Event**: `DeviceHandle {Cpu, Gpu(u32)}` with `kind() -> DeviceKind`. `DeviceKind {Cpu, Gpu}` is `#[non_exhaustive]`.
- **Kernel-Register**: `KernelRegister::capability() -> Option<ComputeCapability>` and `ModuleLoader::device_capability()` can serve as the source for `GpuProbe::compute_capability`. This module does not call the Kernel-Register directly.
- **target_env**: `ComputeCapability { major, minor }` (`Copy`, `Ord`) and `ComputeCapability::new(u32, u32)`.