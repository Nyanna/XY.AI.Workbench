use super::*;

#[test]
fn dot_basic_and_empty() {
    assert_eq!(dot(&[], &[]), 0.0);
    let a: Vec<f32> = (1..=11).map(|x| x as f32).collect();
    let expected: f32 = a.iter().map(|x| x * x).sum();
    assert_eq!(dot(&a, &a), expected);
}

#[test]
#[should_panic]
fn dot_len_mismatch_panics() {
    dot(&[1.0], &[1.0, 2.0]);
}

#[test]
fn max_sim_values_and_errors() {
    let q = [1.0, 0.0, 0.0, 1.0];
    let d = [1.0, 0.0, 0.5, 0.5];
    assert_eq!(max_sim(&q, &d, 2).unwrap(), 1.5);
    assert_eq!(max_sim(&q, &[], 2).unwrap(), 2.0 * MASK_VALUE);
    assert_eq!(max_sim(&[], &[], 2).unwrap(), 0.0);
    assert_eq!(max_sim(&q, &d, 0), Err(CpuComputeError::InvalidDim));
    assert_eq!(
        max_sim(&[1.0, 2.0, 3.0], &d, 2),
        Err(CpuComputeError::Shape { what: ShapeSource::Query(0), len: 3, dim: 2 })
    );
    assert_eq!(
        max_sim(&q, &[1.0], 2),
        Err(CpuComputeError::Shape { what: ShapeSource::Doc, len: 1, dim: 2 })
    );
}

#[test]
fn l2_normalize_cases() {
    let mut v = [3.0, 4.0];
    l2_normalize(&mut v);
    assert!((v[0] - 0.6).abs() < 1e-6 && (v[1] - 0.8).abs() < 1e-6);
    let mut z = [0.0, 0.0];
    l2_normalize(&mut z);
    assert_eq!(z, [0.0, 0.0]);
    let mut e: [f32; 0] = [];
    l2_normalize(&mut e);
}

#[test]
fn scan_matches_max_sim() {
    let q1: &[f32] = &[1.0, 0.0, 0.0, 1.0];
    let q2: &[f32] = &[];
    let s0: &[f32] = &[1.0, 0.0, 0.5, 0.5];
    let s2: &[f32] = &[0.0, 2.0];
    let slots = [Some(s0), None, Some(s2)];
    let out = scan_slots(&[q1, q2], &slots, 0..3, 2).unwrap();
    assert_eq!(out.len(), 6);
    for (i, s) in [Some(s0), None, Some(s2)].iter().enumerate() {
        assert_eq!(out[i], max_sim(q1, s.unwrap_or(&[]), 2).unwrap());
        assert_eq!(out[3 + i], 0.0);
    }
    let sub = scan_slots(&[q1], &slots, 1..3, 2).unwrap();
    assert_eq!(sub, vec![out[1], out[2]]);
}

#[test]
fn scan_errors() {
    let q: &[f32] = &[1.0, 2.0];
    let bad: &[f32] = &[1.0, 2.0, 3.0];
    let slots = [Some(q), Some(bad)];
    assert_eq!(scan_slots(&[q], &slots, 0..2, 0), Err(CpuComputeError::InvalidDim));
    assert_eq!(
        scan_slots(&[q], &slots, 0..3, 2),
        Err(CpuComputeError::RangeOutOfBounds { start: 0, end: 3, slots: 2 })
    );
    assert_eq!(
        scan_slots(&[bad], &slots, 0..2, 2),
        Err(CpuComputeError::Shape { what: ShapeSource::Query(0), len: 3, dim: 2 })
    );
    assert_eq!(
        scan_slots(&[q], &slots, 0..2, 2),
        Err(CpuComputeError::Shape { what: ShapeSource::Slot(1), len: 3, dim: 2 })
    );
    assert_eq!(scan_slots(&[], &slots, 1..1, 2), Ok(vec![]));
}
