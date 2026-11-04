use super::*;

#[test]
fn dtype_conversion_sizes() {
    assert_eq!(encode_bytes(&[1.0, 2.0], DType::F32).unwrap().len(), 8);
    assert_eq!(encode_bytes(&[1.0, 2.0], DType::F16).unwrap().len(), 4);
    assert!(encode_bytes(&[1.0], DType::U8).is_err());
}

#[test]
fn f16_roundtrip_is_within_tolerance() {
    let bytes = encode_bytes(&[0.1234567], DType::F16).unwrap();
    let back = f16::from_le_bytes([bytes[0], bytes[1]]).to_f32();
    assert!((back - 0.1234567).abs() < 1e-3);
}
