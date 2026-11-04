use super::*;

fn set() -> StarvedSet {
    StarvedSet::new(Duration::from_millis(100))
}

#[test]
fn mark_is_visible_until_ttl() {
    let s = set();
    s.mark_at(ResourceKind::Encoder, 10);
    assert!(s.snapshot_at(50).contains(ResourceKind::Encoder));
    assert!(!s.snapshot_at(50).contains(ResourceKind::Scorer));
    assert!(s.snapshot_at(111).is_empty());
}

#[test]
fn pick_clears_only_its_kind() {
    let s = set();
    s.mark_at(ResourceKind::Encoder, 0);
    s.mark_at(ResourceKind::Scorer, 0);
    s.clear(ResourceKind::Encoder);
    let snap = s.snapshot_at(1);
    assert!(!snap.contains(ResourceKind::Encoder));
    assert!(snap.contains(ResourceKind::Scorer));
}

#[test]
fn remark_extends_lifetime() {
    let s = set();
    s.mark_at(ResourceKind::Index, 0);
    s.mark_at(ResourceKind::Index, 90);
    assert!(s.snapshot_at(150).contains(ResourceKind::Index));
}

#[test]
fn clear_all_consumes_snapshot() {
    let s = set();
    s.mark_at(ResourceKind::Encoder, 0);
    s.mark_at(ResourceKind::Tokenizer, 0);
    let snap = s.snapshot_at(1);
    s.clear_all(snap);
    assert!(s.snapshot_at(1).is_empty());
}
