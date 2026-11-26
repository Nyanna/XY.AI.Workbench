use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
const fn assert_send_sync_static<T: Send + Sync + 'static>() {}
const _: () = assert_send_sync_static::<Projection>();
const _: () = assert_send_sync_static::<ProjectionError>();
struct TensorSpec {
    name: &'static str,
    dtype: &'static str,
    shape: Vec<usize>,
    data: Vec<u8>,
}
fn spec(
    name: &'static str,
    dtype: &'static str,
    shape: &[usize],
    data: Vec<u8>,
) -> TensorSpec {
    TensorSpec {
        name,
        dtype,
        shape: shape.to_vec(),
        data,
    }
}
fn f32_bytes(v: &[f32]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}
fn u16_bytes(v: &[u16]) -> Vec<u8> {
    v.iter().flat_map(|x| x.to_le_bytes()).collect()
}
fn safetensors(specs: &[TensorSpec]) -> Vec<u8> {
    let mut entries = Vec::new();
    let mut data = Vec::new();
    for s in specs {
        let begin = data.len();
        data.extend_from_slice(&s.data);
        entries
            .push(
                format!(
                    "\"{}\":{{\"dtype\":\"{}\",\"shape\":{:?},\"data_offsets\":[{},{}]}}",
                    s.name, s.dtype, s.shape, begin, data.len()
                ),
            );
    }
    let header = format!("{{{}}}", entries.join(","));
    raw_file(&header, &data)
}
fn raw_file(header: &str, data: &[u8]) -> Vec<u8> {
    let mut out = (header.len() as u64).to_le_bytes().to_vec();
    out.extend_from_slice(header.as_bytes());
    out.extend_from_slice(data);
    out
}
fn temp_file(bytes: &[u8]) -> PathBuf {
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let path = std::env::temp_dir()
        .join(format!("projection_layer_{}_{}.safetensors", std::process::id(), n));
    std::fs::write(&path, bytes).unwrap();
    path
}
fn layer(name: &str) -> ProjectionLayer {
    ProjectionLayer {
        tensor_name: name.to_owned(),
    }
}
const W: [f32; 6] = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
const B: [f32; 2] = [10.0, 20.0];
fn f32_model(with_bias: bool) -> Vec<u8> {
    let mut specs = vec![spec("proj.weight", "F32", & [2, 3], f32_bytes(& W))];
    if with_bias {
        specs.push(spec("proj.bias", "F32", &[2], f32_bytes(&B)));
    }
    safetensors(&specs)
}
#[test]
fn bias_names() {
    assert_eq!(bias_tensor_name("proj.weight"), "proj.bias");
    assert_eq!(bias_tensor_name("weight"), "bias");
    assert_eq!(bias_tensor_name("proj"), "projbias");
    assert_eq!(bias_tensor_name(""), "bias");
    assert_eq!(bias_tensor_name("weight.weight"), "weight.bias");
}
#[test]
fn f16_conversion_is_exact_for_all_values() {
    for h in 0..=u16::MAX {
        let got = f16_to_f32(h);
        let sign = if h & 0x8000 != 0 { -1.0f64 } else { 1.0 };
        let exp = ((h >> 10) & 0x1f) as i32;
        let man = (h & 0x3ff) as f64;
        if exp == 0x1f {
            if man == 0.0 {
                assert_eq!(got, sign as f32 * f32::INFINITY, "{h:#06x}");
            } else {
                assert!(got.is_nan(), "{h:#06x}");
                assert_eq!(got.is_sign_negative(), h & 0x8000 != 0);
            }
            continue;
        }
        let expected = if exp == 0 {
            sign * man * 2f64.powi(-24)
        } else {
            sign * (1024.0 + man) * 2f64.powi(exp - 25)
        };
        assert_eq!(got as f64, expected, "{h:#06x}");
        assert_eq!(got.is_sign_negative(), h & 0x8000 != 0, "{h:#06x}");
    }
}
#[test]
fn f16_known_values() {
    assert_eq!(f16_to_f32(0x3c00), 1.0);
    assert_eq!(f16_to_f32(0xc000), - 2.0);
    assert_eq!(f16_to_f32(0x7bff), 65504.0);
    assert_eq!(f16_to_f32(0x0001), 2f32.powi(- 24));
    assert!(f16_to_f32(0x8000).is_sign_negative());
    assert_eq!(f16_to_f32(0x8000), 0.0);
    assert_eq!(f16_to_f32(0x7c00), f32::INFINITY);
    assert_eq!(f16_to_f32(0xfc00), f32::NEG_INFINITY);
    assert!(f16_to_f32(0x7e00).is_nan());
}
#[test]
fn bf16_conversion() {
    assert_eq!(bf16_to_f32(0x3f80), 1.0);
    assert_eq!(bf16_to_f32(0xc000), - 2.0);
    assert_eq!(bf16_to_f32(0x7f80), f32::INFINITY);
    assert!(bf16_to_f32(0x7fc0).is_nan());
    assert_eq!(bf16_to_f32(0x0001).to_bits(), 0x0001_0000);
}
#[test]
fn from_bytes_f32_with_bias() {
    let p = Projection::from_bytes(&f32_model(true), &layer("proj.weight"), 3, 2)
        .unwrap();
    assert_eq!(p.input_dim(), 3);
    assert_eq!(p.output_dim(), 2);
    let out = p.apply(&[vec![1.0, 0.0, - 1.0], vec![1.0, 1.0, 1.0]]).unwrap();
    assert_eq!(out, vec![vec![8.0, 18.0], vec![16.0, 35.0]]);
}
#[test]
fn from_bytes_without_bias() {
    let p = Projection::from_bytes(&f32_model(false), &layer("proj.weight"), 3, 2)
        .unwrap();
    assert_eq!(p.bias, None);
    let out = p.apply(&[vec![1.0, 1.0, 1.0]]).unwrap();
    assert_eq!(out, vec![vec![6.0, 15.0]]);
}
#[test]
fn from_bytes_f16_and_bf16() {
    let f16w = u16_bytes(&[0x3c00, 0x4000, 0x4200, 0x4400, 0x4500, 0x4600]);
    let bf16w = u16_bytes(&[0x3f80, 0x4000, 0x4040, 0x4080, 0x40a0, 0x40c0]);
    let f16b = u16_bytes(&[0x4900, 0x4d00]);
    let bf16b = u16_bytes(&[0x4120, 0x41a0]);
    let a = safetensors(
        &[
            spec("proj.weight", "F16", &[2, 3], f16w),
            spec("proj.bias", "F16", &[2], f16b),
        ],
    );
    let b = safetensors(
        &[
            spec("proj.weight", "BF16", &[2, 3], bf16w),
            spec("proj.bias", "BF16", &[2], bf16b),
        ],
    );
    let reference = Projection::from_bytes(&f32_model(true), &layer("proj.weight"), 3, 2)
        .unwrap();
    assert_eq!(
        Projection::from_bytes(& a, & layer("proj.weight"), 3, 2).unwrap(), reference
    );
    assert_eq!(
        Projection::from_bytes(& b, & layer("proj.weight"), 3, 2).unwrap(), reference
    );
}
#[test]
fn mixed_dtypes_between_weight_and_bias() {
    let bytes = safetensors(
        &[
            spec("proj.weight", "F32", &[2, 3], f32_bytes(&W)),
            spec("proj.bias", "BF16", &[2], u16_bytes(&[0x4120, 0x41a0])),
        ],
    );
    let p = Projection::from_bytes(&bytes, &layer("proj.weight"), 3, 2).unwrap();
    assert_eq!(p.bias, Some(vec![10.0, 20.0]));
}
#[test]
fn missing_weight_is_name_mismatch() {
    let err = Projection::from_bytes(&f32_model(true), &layer("other.weight"), 3, 2)
        .unwrap_err();
    assert_eq!(
        err, ProjectionError::WeightNameMismatch { name : "other.weight".into() }
    );
    assert_eq!(err.to_string(), "Gewichtsname passt nicht: other.weight");
}
#[test]
fn metadata_entry_is_not_a_tensor() {
    let header = r#"{"__metadata__":{"format":"pt"}}"#;
    let err = Projection::from_bytes(
            &raw_file(header, &[]),
            &layer("__metadata__"),
            1,
            1,
        )
        .unwrap_err();
    assert!(matches!(err, ProjectionError::WeightNameMismatch { .. }));
}
#[test]
fn wrong_weight_shape() {
    let err = Projection::from_bytes(&f32_model(true), &layer("proj.weight"), 2, 3)
        .unwrap_err();
    assert_eq!(
        err, ProjectionError::IncompatibleShape { tensor : "proj.weight".into(), actual :
        vec![2, 3], expected : vec![3, 2], }
    );
    assert_eq!(
        err.to_string(),
        "inkompatibel: Tensor proj.weight hat Shape [2, 3], erwartet [3, 2]"
    );
}
#[test]
fn shape_is_checked_before_dtype() {
    let bytes = safetensors(&[spec("proj.weight", "F64", &[2, 3], vec![0; 48])]);
    let err = Projection::from_bytes(&bytes, &layer("proj.weight"), 4, 2).unwrap_err();
    assert!(matches!(err, ProjectionError::IncompatibleShape { .. }));
}
#[test]
fn unsupported_weight_dtype() {
    for dtype in ["I64", "F64", "U8"] {
        let bytes = safetensors(&[spec("proj.weight", dtype, &[2, 3], vec![0; 6])]);
        let err = Projection::from_bytes(&bytes, &layer("proj.weight"), 3, 2)
            .unwrap_err();
        assert_eq!(
            err, ProjectionError::IncompatibleDType { tensor : "proj.weight".into(),
            dtype : dtype.into() }
        );
    }
    let err = ProjectionError::IncompatibleDType {
        tensor: "t".into(),
        dtype: "I64".into(),
    };
    assert_eq!(
        err.to_string(), "inkompatibel: Tensor t hat nicht unterstützten Typ I64"
    );
}
#[test]
fn bias_errors_name_the_bias_tensor() {
    let bad_shape = safetensors(
        &[
            spec("proj.weight", "F32", &[2, 3], f32_bytes(&W)),
            spec("proj.bias", "F32", &[3], f32_bytes(&[1.0, 2.0, 3.0])),
        ],
    );
    assert_eq!(
        Projection::from_bytes(& bad_shape, & layer("proj.weight"), 3, 2).unwrap_err(),
        ProjectionError::IncompatibleShape { tensor : "proj.bias".into(), actual :
        vec![3], expected : vec![2], }
    );
    let bad_dtype = safetensors(
        &[
            spec("proj.weight", "F32", &[2, 3], f32_bytes(&W)),
            spec("proj.bias", "I64", &[2], vec![0; 16]),
        ],
    );
    assert_eq!(
        Projection::from_bytes(& bad_dtype, & layer("proj.weight"), 3, 2).unwrap_err(),
        ProjectionError::IncompatibleDType { tensor : "proj.bias".into(), dtype : "I64"
        .into() }
    );
}
#[test]
fn weight_error_wins_over_bias_error() {
    let bytes = safetensors(
        &[
            spec("proj.weight", "I64", &[2, 3], vec![0; 48]),
            spec("proj.bias", "F32", &[5], vec![0; 20]),
        ],
    );
    let err = Projection::from_bytes(&bytes, &layer("proj.weight"), 3, 2).unwrap_err();
    assert!(
        matches!(err, ProjectionError::IncompatibleDType { ref tensor, .. } if tensor ==
        "proj.weight")
    );
}
#[test]
fn unread_tensors_are_ignored() {
    let bytes = safetensors(
        &[
            spec("proj.weight", "F32", &[2, 3], f32_bytes(&W)),
            spec("other", "WEIRD", &[7], vec![1, 2, 3]),
        ],
    );
    assert!(Projection::from_bytes(& bytes, & layer("proj.weight"), 3, 2).is_ok());
}
#[test]
fn zero_dimensions_follow_shape_rules() {
    let bytes = safetensors(
        &[
            spec("proj.weight", "F32", &[2, 0], vec![]),
            spec("proj.bias", "F32", &[2], f32_bytes(&B)),
        ],
    );
    let p = Projection::from_bytes(&bytes, &layer("proj.weight"), 0, 2).unwrap();
    assert_eq!(
        p.apply(& [vec![], vec![]]).unwrap(), vec![vec![10.0, 20.0], vec![10.0, 20.0]]
    );
    let bytes = safetensors(&[spec("proj.weight", "F32", &[0, 3], vec![])]);
    let p = Projection::from_bytes(&bytes, &layer("proj.weight"), 3, 0).unwrap();
    assert_eq!(p.apply(& [vec![1.0, 2.0, 3.0]]).unwrap(), vec![Vec::< f32 >::new()]);
    let err = Projection::from_bytes(&bytes, &layer("proj.weight"), 3, 1).unwrap_err();
    assert!(matches!(err, ProjectionError::IncompatibleShape { .. }));
}
#[test]
fn structural_errors_are_load_errors() {
    let l = layer("proj.weight");
    let load = |bytes: &[u8]| match Projection::from_bytes(bytes, &l, 3, 2) {
        Err(ProjectionError::Load { path: None, .. }) => true,
        _ => false,
    };
    assert!(load(& []));
    assert!(load(& [1, 2, 3, 4, 5, 6, 7]));
    assert!(load(& u64::MAX.to_le_bytes()));
    assert!(load(& 1000u64.to_le_bytes()));
    assert!(load(& raw_file("not json", & [])));
    assert!(load(& raw_file("[1,2]", & [])));
    let entry = |offsets: &str| {
        raw_file(
            &format!(
                r#"{{"proj.weight":{{"dtype":"F32","shape":[2,3],"data_offsets":{offsets}}}}}"#
            ),
            &[0u8; 24],
        )
    };
    assert!(load(& entry("[24,0]")));
    assert!(load(& entry("[0,48]")));
    assert!(load(& entry("[0,20]")));
    assert!(load(& entry("[0,28]")));
    assert!(load(& entry("[0]")));
    assert!(load(& entry("[-1,5]")));
    assert!(Projection::from_bytes(& entry("[0,24]"), & l, 3, 2).is_ok());
    assert!(load(& raw_file(r#"{"proj.weight":{"dtype":"F32"}}"#, & [])));
    assert!(load(& raw_file(r#"{"proj.weight":5}"#, & [])));
}
#[test]
fn huge_shape_does_not_panic_or_allocate() {
    let header = r#"{"proj.weight":{"dtype":"F32","shape":[18446744073709551615,18446744073709551615],"data_offsets":[0,0]}}"#;
    let err = Projection::from_bytes(
            &raw_file(header, &[]),
            &layer("proj.weight"),
            usize::MAX,
            usize::MAX,
        )
        .unwrap_err();
    assert!(matches!(err, ProjectionError::Load { .. }));
}
#[test]
fn arbitrary_truncations_never_panic() {
    let full = f32_model(true);
    for n in 0..full.len() {
        let _ = Projection::from_bytes(&full[..n], &layer("proj.weight"), 3, 2);
    }
    let mut corrupted = full.clone();
    for i in 0..corrupted.len() {
        let saved = corrupted[i];
        corrupted[i] = 0xff;
        let _ = Projection::from_bytes(&corrupted, &layer("proj.weight"), 3, 2);
        corrupted[i] = saved;
    }
}
#[test]
fn apply_validates_dimensions() {
    let p = Projection::from_bytes(&f32_model(true), &layer("proj.weight"), 3, 2)
        .unwrap();
    assert_eq!(p.apply(& []).unwrap(), Vec::< Vec < f32 >>::new());
    let err = p.apply(&[vec![1.0, 2.0, 3.0], vec![1.0, 2.0]]).unwrap_err();
    assert_eq!(err, ProjectionError::InputDimMismatch { expected : 3, actual : 2 });
    assert_eq!(err.to_string(), "Eingabedimension 2 statt 3");
}
#[test]
fn apply_is_deterministic_and_keeps_input() {
    let p = Projection::from_bytes(&f32_model(true), &layer("proj.weight"), 3, 2)
        .unwrap();
    let input = vec![vec![0.1, 0.2, 0.3], vec![- 1.5, 2.5, 0.0]];
    let copy = input.clone();
    let a = p.apply(&input).unwrap();
    let b = p.apply(&input).unwrap();
    assert_eq!(input, copy);
    assert_eq!(a, b);
    let expected = (0.0f32 + 1.0 * 0.1 + 2.0 * 0.2 + 3.0 * 0.3) + 10.0;
    assert_eq!(a[0] [0], expected);
}
#[test]
fn from_file_matches_from_bytes() {
    let bytes = f32_model(true);
    let path = temp_file(&bytes);
    let from_file = Projection::from_file(&path, &layer("proj.weight"), 3, 2).unwrap();
    let from_bytes = Projection::from_bytes(&bytes, &layer("proj.weight"), 3, 2)
        .unwrap();
    assert_eq!(from_file, from_bytes);
    let file = WeightFile::open(&path).unwrap();
    let from_mapped = Projection::from_weight_file(&file, &layer("proj.weight"), 3, 2)
        .unwrap();
    drop(file);
    std::fs::remove_file(&path).unwrap();
    assert_eq!(from_mapped, from_bytes);
}
#[test]
fn file_path_error_mapping() {
    let bytes = safetensors(
        &[
            spec("proj.weight", "I64", &[2, 3], vec![0; 48]),
            spec("proj.bias", "F32", &[2], f32_bytes(&B)),
        ],
    );
    let path = temp_file(&bytes);
    let l = layer("proj.weight");
    assert_eq!(
        Projection::from_file(& path, & l, 3, 2).unwrap_err(),
        ProjectionError::IncompatibleDType { tensor : "proj.weight".into(), dtype : "I64"
        .into() }
    );
    assert!(
        matches!(Projection::from_file(& path, & layer("nope.weight"), 3, 2)
        .unwrap_err(), ProjectionError::WeightNameMismatch { .. })
    );
    assert!(
        matches!(Projection::from_file(& path, & l, 2, 3).unwrap_err(),
        ProjectionError::IncompatibleShape { .. })
    );
    std::fs::remove_file(&path).unwrap();
}
#[test]
fn from_file_load_errors_carry_the_path() {
    let missing = std::env::temp_dir().join("projection_layer_missing_file.safetensors");
    match Projection::from_file(&missing, &layer("proj.weight"), 3, 2).unwrap_err() {
        ProjectionError::Load { path, reason } => {
            assert_eq!(path.as_deref(), Some(missing.as_path()));
            assert!(! reason.is_empty());
        }
        other => panic!("unexpected {other:?}"),
    }
    let bytes = safetensors(
        &[
            spec("proj.weight", "F32", &[2, 3], f32_bytes(&W)),
            spec("other", "F64", &[1], vec![0; 8]),
        ],
    );
    let path = temp_file(&bytes);
    let err = Projection::from_file(&path, &layer("proj.weight"), 3, 2).unwrap_err();
    assert!(matches!(err, ProjectionError::Load { path : Some(_), .. }));
    assert!(err.to_string().starts_with("Load-Error: "));
    std::fs::remove_file(&path).unwrap();
}
#[test]
fn concurrent_apply() {
    let p = Arc::new(
        Projection::from_bytes(&f32_model(true), &layer("proj.weight"), 3, 2).unwrap(),
    );
    let handles: Vec<_> = (0..8)
        .map(|t| {
            let p = Arc::clone(&p);
            std::thread::spawn(move || {
                for i in 0..200 {
                    let x = (t * 1000 + i) as f32;
                    let out = p.apply(&[vec![x, 0.0, 0.0]]).unwrap();
                    assert_eq!(out, vec![vec![x + 10.0, 4.0 * x + 20.0]]);
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
#[test]
fn display_of_load_error_without_path() {
    let err = ProjectionError::Load {
        path: None,
        reason: "boom".into(),
    };
    assert_eq!(err.to_string(), "Load-Error: boom");
}
