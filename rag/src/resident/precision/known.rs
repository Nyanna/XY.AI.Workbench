//! Well-known axes and values for the target hardware (GTX 1660, sm_75).
//! bf16 is deliberately absent: it has no native path before sm_80.

use std::sync::LazyLock;

use super::axis::{AxisId, ValueId};
use super::capability::CapabilitySet;

macro_rules! known_ids {
    ($ty:ident: $($name:ident = $s:literal),+ $(,)?) => {
        $(pub static $name: LazyLock<$ty> = LazyLock::new(|| $ty::new($s));)+
    };
}

known_ids!(AxisId:
    OPERANDS = "operands",
    ACCUMULATOR = "accumulator",
    SCORING = "scoring",
    FUSION = "fusion",
    BATCH = "batch",
    TILE = "tile",
    THREADS = "threads",
);

known_ids!(ValueId:
    FP32 = "fp32",
    FP16 = "fp16",
    FP16_TO_FP32 = "fp16->fp32",
    INT8 = "int8",
    // TODO: verify dp4a usage and bf16 behaviour on sm_75 hardware.
    INT8_DP4A = "int8-dp4a",
    ON = "on",
    OFF = "off",
);

/// Upper bound of what the GPU can offer; the real set is measured at runtime.
/// Numeric axes (`batch`, `tile`) carry measured values and are not listed.
pub fn gpu_sm75() -> CapabilitySet {
    let mut c = CapabilitySet::new();
    for v in [&FP32, &FP16] {
        c.insert(&*OPERANDS, &**v);
        c.insert(&*ACCUMULATOR, &**v);
    }
    for v in [&FP32, &FP16, &INT8_DP4A] {
        c.insert(&*SCORING, &**v);
    }
    for v in [&ON, &OFF] {
        c.insert(&*FUSION, &**v);
    }
    c
}

/// Numeric axes (`threads`, `tile`) are not listed.
pub fn cpu() -> CapabilitySet {
    let mut c = CapabilitySet::new();
    for v in [&FP32, &FP16_TO_FP32] {
        c.insert(&*OPERANDS, &**v);
    }
    for v in [&FP32, &INT8] {
        c.insert(&*SCORING, &**v);
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gpu_offers_no_bf16() {
        let gpu = gpu_sm75();
        assert!(gpu.contains(&OPERANDS, &FP16));
        assert!(gpu.contains(&SCORING, &INT8_DP4A));
        assert!(!gpu.contains(&OPERANDS, &ValueId::new("bf16")));
        assert!(!gpu.contains(&ACCUMULATOR, &ValueId::new("bf16")));
    }

    #[test]
    fn gpu_satisfies_cpu_independent_requirements() {
        let req = CapabilitySet::new().with(&*OPERANDS, &*FP16).with(&*FUSION, &*ON);
        assert!(gpu_sm75().satisfies(&req));
        assert!(!cpu().satisfies(&req));
    }
}
