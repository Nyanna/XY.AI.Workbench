//! Static table of element types with scoring kernels per device kind.
use super::super::l010::capability_model::{axes, values, CapabilitySet, Value};
use super::super::l010::device_lane_event::DeviceKind;
use super::super::l010::layout::DType;
/// Element types with a scoring kernel for the device kind, in fixed order.
pub fn scoring_dtypes(kind: DeviceKind) -> &'static [DType] {
    match kind {
        DeviceKind::Cpu => &[DType::F32],
        DeviceKind::Gpu => &[DType::F32, DType::F16],
        #[allow(unreachable_patterns)]
        _ => &[],
    }
}
/// True iff `dtype` is in `scoring_dtypes(kind)`.
pub fn has_scoring_kernel(kind: DeviceKind, dtype: DType) -> bool {
    scoring_dtypes(kind).contains(&dtype)
}
/// Capability value name of the type; `None` for types that are never compute types.
pub fn dtype_value(dtype: DType) -> Option<Value> {
    match dtype {
        DType::F32 => Some(values::fp32()),
        DType::F16 => Some(values::fp16()),
        _ => None,
    }
}
/// Operands and scoring axes with the values of all supported types.
pub fn precision_capabilities(kind: DeviceKind) -> CapabilitySet {
    let mut set = CapabilitySet::new();
    for value in scoring_dtypes(kind).iter().filter_map(|d| dtype_value(*d)) {
        set.add(axes::operands(), value.clone());
        set.add(axes::scoring(), value);
    }
    set
}
#[cfg(test)]
#[path = "test/supported_precisions.rs"]
mod tests;
