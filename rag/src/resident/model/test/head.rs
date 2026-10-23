use super::*;

fn checkpoint(with_bias: bool) -> Vec<u8> {
    let w: Vec<u8> = [1.0f32, 0.0, 0.0, 1.0, 1.0, 1.0].iter().flat_map(|v| v.to_le_bytes()).collect();
    let b: Vec<u8> = [0.5f32, 0.0, -1.0].iter().flat_map(|v| v.to_le_bytes()).collect();
    let mut tensors = vec![("linear.weight", TensorView::new(Dtype::F32, vec![3, 2], &w).unwrap())];
    if with_bias {
        tensors.push(("linear.bias", TensorView::new(Dtype::F32, vec![3], &b).unwrap()));
    }
    safetensors::serialize(tensors, None).unwrap()
}

fn spec(bias: bool) -> HeadSpec {
    HeadSpec { weight_name: "linear.weight".into(), in_dim: 2, out_dim: 3, bias }
}

#[test]
fn projects_with_and_without_bias() {
    let plain = ProjectionHead::from_safetensors(&checkpoint(false), &spec(false)).unwrap();
    assert_eq!(plain.apply(&[2.0, 3.0]), vec![2.0, 3.0, 5.0]);
    let biased = ProjectionHead::from_safetensors(&checkpoint(true), &spec(true)).unwrap();
    assert_eq!(biased.apply(&[2.0, 3.0]), vec![2.5, 3.0, 4.0]);
}

#[test]
fn missing_or_misshaped_tensors_are_reported() {
    let mut s = spec(false);
    s.weight_name = "dense.weight".into();
    assert!(matches!(
        ProjectionHead::from_safetensors(&checkpoint(false), &s),
        Err(ModelError::WeightNameMismatch { .. })
    ));
    let mut s = spec(false);
    s.out_dim = 4;
    assert!(matches!(
        ProjectionHead::from_safetensors(&checkpoint(false), &s),
        Err(ModelError::Incompatible { .. })
    ));
}
