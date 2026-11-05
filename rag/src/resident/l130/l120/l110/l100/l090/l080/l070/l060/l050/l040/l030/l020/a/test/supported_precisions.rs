use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::l010::capability_model::{
    cpu_reference, gpu_reference,
};

const ALL: [DType; 6] = [
    DType::F32,
    DType::F16,
    DType::BF16,
    DType::U8,
    DType::U32,
    DType::I64,
];
const KINDS: [DeviceKind; 2] = [DeviceKind::Cpu, DeviceKind::Gpu];

#[test]
fn dtype_tables() {
    assert_eq!(scoring_dtypes(DeviceKind::Cpu), &[DType::F32]);
    assert_eq!(scoring_dtypes(DeviceKind::Gpu), &[DType::F32, DType::F16]);
}

#[test]
fn kernel_presence() {
    for k in KINDS {
        assert!(!has_scoring_kernel(k, DType::BF16));
        assert!(!has_scoring_kernel(k, DType::U8));
        assert!(!has_scoring_kernel(k, DType::U32));
        assert!(!has_scoring_kernel(k, DType::I64));
        assert!(has_scoring_kernel(k, DType::F32));
    }
    assert!(!has_scoring_kernel(DeviceKind::Cpu, DType::F16));
    assert!(has_scoring_kernel(DeviceKind::Gpu, DType::F16));
}

#[test]
fn dtype_values() {
    assert_eq!(dtype_value(DType::F32), Some(values::fp32()));
    assert_eq!(dtype_value(DType::F16), Some(values::fp16()));
    for d in [DType::BF16, DType::U8, DType::U32, DType::I64] {
        assert!(dtype_value(d).is_none());
    }
    for k in KINDS {
        for d in ALL {
            if scoring_dtypes(k).contains(&d) {
                assert!(dtype_value(d).is_some());
            }
            if dtype_value(d).is_none() {
                assert!(!has_scoring_kernel(k, d));
            }
        }
    }
}

#[test]
fn capabilities_exact() {
    let cpu = CapabilitySet::new()
        .with(axes::operands(), values::fp32())
        .with(axes::scoring(), values::fp32());
    assert_eq!(precision_capabilities(DeviceKind::Cpu), cpu);
    let gpu = CapabilitySet::new()
        .with(axes::operands(), values::fp32())
        .with(axes::operands(), values::fp16())
        .with(axes::scoring(), values::fp32())
        .with(axes::scoring(), values::fp16());
    assert_eq!(precision_capabilities(DeviceKind::Gpu), gpu);
}

#[test]
fn within_references() {
    assert!(precision_capabilities(DeviceKind::Cpu).is_within(&cpu_reference()));
    assert!(precision_capabilities(DeviceKind::Gpu).is_within(&gpu_reference()));
}

#[test]
fn only_two_axes_and_repeatable() {
    for k in KINDS {
        let c = precision_capabilities(k);
        assert_eq!(c.axes().count(), 2);
        assert_eq!(c.values(&axes::operands()), c.values(&axes::scoring()));
        assert_eq!(c, precision_capabilities(k));
    }
}
