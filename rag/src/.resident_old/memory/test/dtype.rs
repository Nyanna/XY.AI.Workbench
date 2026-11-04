use super::*;

#[test]
fn sizes() {
    assert_eq!(DType::U8.size_bytes(), 1);
    assert_eq!(DType::F16.size_bytes(), 2);
    assert_eq!(DType::Bf16.size_bytes(), 2);
    assert_eq!(DType::F32.size_bytes(), 4);
    assert_eq!(DType::U32.size_bytes(), 4);
    assert_eq!(DType::I64.size_bytes(), 8);
}

#[test]
fn safetensors_convert_and_reject() {
    assert_eq!(DType::try_from(safetensors::Dtype::F16), Ok(DType::F16));
    assert!(DType::try_from(safetensors::Dtype::F64).is_err());
}

#[cfg(feature = "candle")]
#[test]
fn candle_roundtrip_and_reject() {
    for d in [DType::F32, DType::F16, DType::Bf16, DType::U8, DType::U32, DType::I64] {
        let c = candle_core::DType::try_from(d).unwrap();
        assert_eq!(DType::try_from(c), Ok(d));
    }
    assert!(DType::try_from(candle_core::DType::F64).is_err());
}
