use super::*;

fn hit(offset: u64, score: f32) -> KernelHit {
    KernelHit { offset, score }
}

#[test]
fn keeps_best_k_in_descending_order() {
    let mut t = TopK::new(3);
    for (i, s) in [0.1, 0.9, 0.5, 0.7, 0.3].into_iter().enumerate() {
        t.push(hit(i as u64, s));
    }
    let got: Vec<u64> = t.into_sorted().iter().map(|h| h.offset).collect();
    assert_eq!(got, vec![1, 3, 2]);
}

#[test]
fn ties_prefer_lower_offset() {
    let mut t = TopK::new(2);
    for off in [5, 3, 9, 1] {
        t.push(hit(off, 1.0));
    }
    let got: Vec<u64> = t.into_sorted().iter().map(|h| h.offset).collect();
    assert_eq!(got, vec![1, 3]);
}

#[test]
fn nan_and_zero_k_are_ignored() {
    let mut t = TopK::new(2);
    t.push(hit(0, f32::NAN));
    t.push(hit(1, 0.5));
    assert_eq!(t.len(), 1);
    let mut z = TopK::new(0);
    z.push(hit(0, 1.0));
    assert!(z.is_empty());
}

#[test]
fn merge_over_slices_equals_global_selection() {
    let all: Vec<KernelHit> = (0..50u64).map(|i| hit(i, ((i * 37) % 23) as f32)).collect();
    let global = merge([all.clone()], 7);
    let parts = all.chunks(8).map(|c| merge([c.to_vec()], 7));
    assert_eq!(merge(parts, 7), global);
}
