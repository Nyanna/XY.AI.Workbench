use std::fmt;

pub const GPU_NAME: &str = "NVIDIA GeForce GTX 1660 SUPER";
pub const DRIVER_VERSION: &str = "610.57.04";
pub const CUDA_RELEASE: &str = "13.3";
pub const CUDA_VERSION: &str = "13.3.73";
pub const CUDA_BUILD: &str = "38244171_0";
pub const CARGO_MANIFEST_PATH: &str = "/home/user/xyan/xy.ai.workbench/rag/Cargo.toml";

/// Compute capability (major, minor).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ComputeCapability {
    pub major: u32,
    pub minor: u32,
}

pub const REFERENCE_CC: ComputeCapability = ComputeCapability { major: 7, minor: 5 };
/// First CC with acquire-semantics loads.
pub const MIN_ACQUIRE_CC: ComputeCapability = ComputeCapability { major: 7, minor: 0 };

/// Tensor-Core path offered? Always false.
pub const TENSOR_CORE_PATH: bool = false;
/// BF16 as compute path offered? Always false.
pub const BF16_COMPUTE: bool = false;

pub const WEIGHTS_FORMAT: &str = "safetensors";
pub const INDEX_FORMAT: &str = "parquet";
pub const TENSOR_LIBRARY: &str = "candle";

impl ComputeCapability {
    pub const fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    /// JIT target string, e.g. (7,5) -> "sm_75".
    pub fn jit_arch(&self) -> String {
        format!("sm_{}{}", self.major, self.minor)
    }

    pub fn supports_acquire_loads(&self) -> bool {
        *self >= MIN_ACQUIRE_CC
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetEnvError {
    /// Detected CC is below the minimum (7.0).
    UnsupportedCapability(ComputeCapability),
    /// A Tensor-Core or BF16-compute path was requested.
    PathNotOffered(&'static str),
}

impl fmt::Display for TargetEnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TargetEnvError::UnsupportedCapability(cc) => write!(
                f,
                "unsupported compute capability {}.{}: minimum is {}.{}",
                cc.major, cc.minor, MIN_ACQUIRE_CC.major, MIN_ACQUIRE_CC.minor
            ),
            TargetEnvError::PathNotOffered(name) => {
                write!(f, "path not offered: {}", name)
            }
        }
    }
}

impl std::error::Error for TargetEnvError {}

/// Validates a detected CC and returns the JIT arch string for it.
pub fn jit_target_for(detected: ComputeCapability) -> Result<String, TargetEnvError> {
    if detected < MIN_ACQUIRE_CC {
        Err(TargetEnvError::UnsupportedCapability(detected))
    } else {
        Ok(detected.jit_arch())
    }
}

pub fn require_tensor_core_path() -> Result<(), TargetEnvError> {
    Err(TargetEnvError::PathNotOffered("tensor_core"))
}

pub fn require_bf16_compute() -> Result<(), TargetEnvError> {
    Err(TargetEnvError::PathNotOffered("bf16_compute"))
}

#[cfg(test)]
#[path = "test/target_env.rs"]
mod tests;
