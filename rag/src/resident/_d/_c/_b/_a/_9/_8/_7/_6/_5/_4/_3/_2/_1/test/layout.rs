use super::*;

const ALL: [DType; 6] = [
    DType::F32,
    DType::F16,
    DType::BF16,
    DType::U8,
    DType::U32,
    DType::I64,
];

#[test]
fn sizes_and_float() {
    let sizes: Vec<usize> = ALL.iter().map(|d| d.size_in_bytes()).collect();
    assert_eq!(sizes, vec![4, 2, 2, 1, 4, 8]);
    assert!(DType::BF16.is_float());
    assert!(!DType::I64.is_float());
}

#[test]
fn safetensors_roundtrip_and_errors() {
    for d in ALL {
        assert_eq!(DType::from_safetensors(d.to_safetensors(), "x"), Ok(d));
    }
    for bad in ["F64", "I32", "I16", "I8", "U16", "U64", "BOOL", "F8_E4M3", "", "f32"] {
        let err = DType::from_safetensors(bad, "m.safetensors:w").unwrap_err();
        assert_eq!(
            err,
            LayoutError::UnsupportedDType {
                dtype: bad.to_owned(),
                source: "m.safetensors:w".to_owned()
            }
        );
    }
    let msg = DType::from_safetensors("F64", "a").unwrap_err().to_string();
    assert_eq!(msg, "nicht unterstützter Datentyp: F64 (Quelle: a)");
}

#[test]
fn mask_value() {
    assert_eq!(MASK_VALUE, -10000.0);
    assert!(MASK_VALUE.is_finite() && MASK_VALUE.abs() < 65504.0);
}

#[test]
fn contiguous_strides_examples() {
    assert_eq!(Layout::contiguous(&[2, 3, 4], DType::F32).strides(), &[48, 16, 4]);
    assert_eq!(Layout::contiguous(&[2, 0, 4], DType::F32).strides(), &[16, 16, 4]);
}

#[test]
fn scalar() {
    let l = Layout::contiguous(&[], DType::I64);
    assert_eq!(l.rank(), 0);
    assert_eq!(l.elem_count(), 1);
    assert_eq!(l.byte_len(), 8);
    assert!(l.is_contiguous());
}

#[test]
fn new_rank_mismatch() {
    assert_eq!(
        Layout::new(vec![1, 2], vec![4], DType::F32),
        Err(LayoutError::RankMismatch {
            shape_rank: 2,
            strides_rank: 1
        })
    );
}

#[test]
fn counts() {
    let l = Layout::contiguous(&[2, 3], DType::F16);
    assert_eq!(l.elem_count(), 6);
    assert_eq!(l.byte_len(), 12);
    let z = Layout::contiguous(&[2, 0], DType::F16);
    assert_eq!(z.elem_count(), 0);
    assert!(z.is_contiguous());
}

#[test]
fn transpose_behavior() {
    let l = Layout::contiguous(&[2, 3], DType::F32);
    let t = l.transpose(0, 1).unwrap();
    assert_eq!(t.shape(), &[3, 2]);
    assert_eq!(t.strides(), &[4, 12]);
    assert!(!t.is_contiguous());
    assert_eq!(t.transpose(0, 1).unwrap(), l);
    assert_eq!(l.transpose(1, 1).unwrap(), l);
    assert_eq!(
        l.transpose(0, 2),
        Err(LayoutError::DimOutOfRange { dim: 2, rank: 2 })
    );
}

#[test]
fn narrow_behavior() {
    let l = Layout::contiguous(&[4, 3], DType::F32);
    let n = l.narrow(0, 1, 2).unwrap();
    assert_eq!(n.shape(), &[2, 3]);
    assert_eq!(n.strides(), l.strides());
    assert!(n.is_contiguous());
    let m = l.narrow(1, 0, 2).unwrap();
    assert!(!m.is_contiguous());
    assert_eq!(l.narrow(0, 4, 0).unwrap().shape(), &[0, 3]);
    assert!(matches!(
        l.narrow(0, 3, 2),
        Err(LayoutError::NarrowOutOfRange { .. })
    ));
    assert!(matches!(
        l.narrow(0, usize::MAX, 2),
        Err(LayoutError::NarrowOutOfRange { .. })
    ));
    assert!(matches!(
        l.narrow(2, 0, 1),
        Err(LayoutError::DimOutOfRange { .. })
    ));
}

#[test]
fn narrow_offset() {
    let l = Layout::contiguous(&[4, 3], DType::F32);
    assert_eq!(l.narrow_offset_bytes(0, 2), Ok(24));
    assert_eq!(l.narrow_offset_bytes(0, 4), Ok(48));
    assert_eq!(
        l.narrow_offset_bytes(0, 5),
        Err(LayoutError::NarrowOutOfRange {
            dim: 0,
            start: 5,
            len: 0,
            size: 4
        })
    );
    assert_eq!(
        l.narrow_offset_bytes(2, 0),
        Err(LayoutError::DimOutOfRange { dim: 2, rank: 2 })
    );
}

#[test]
fn send_sync() {
    fn check<T: Send + Sync>() {}
    check::<DType>();
    check::<Layout>();
}
