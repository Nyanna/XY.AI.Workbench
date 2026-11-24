use super::super::fixture::*;
use super::*;
fn ids(list: &[EntryId]) -> Vec<u64> {
    list.iter().map(|id| id.0).collect()
}
#[test]
fn rows_are_decoded_for_every_supported_dtype() {
    let values = [1.5f32, -2.0, 0.25];
    let f32_bytes: Vec<u8> = values.iter().flat_map(|v| v.to_le_bytes()).collect();
    let f16_bytes: Vec<u8> = values
        .iter()
        .flat_map(|v| half::f16::from_f32(*v).to_le_bytes())
        .collect();
    let bf16_bytes: Vec<u8> = values
        .iter()
        .flat_map(|v| half::bf16::from_f32(*v).to_le_bytes())
        .collect();
    assert_eq!(decode_rows(DType::F32, & f32_bytes), values);
    assert_eq!(decode_rows(DType::F16, & f16_bytes), values);
    assert_eq!(decode_rows(DType::BF16, & bf16_bytes), values);
}
#[test]
fn vectors_are_validated_against_the_slot() {
    let f = Fixture::new();
    let inner = &f.manager.inner;
    assert_eq!(inner.validate_vector(0), Ok(0));
    assert_eq!(inner.validate_vector(16), Ok(1));
    assert_eq!(inner.validate_vector(32), Ok(2));
    assert_eq!(
        inner.validate_vector(15), Err(GpuManagerError::InvalidVector { len : 15,
        row_bytes : 16 })
    );
    assert_eq!(
        inner.validate_vector(48), Err(GpuManagerError::TooManyTokens { tokens : 3, max :
        MAX_TOKENS })
    );
}
#[test]
fn free_slots_of_a_published_segment_are_filled_first() {
    let f = Fixture::new();
    f.resident(3);
    f.put(4, 2, 4.0);
    let report = f.manager.ensure(&set(&[4])).unwrap();
    assert_eq!(ids(& report.loaded), vec![4]);
    assert_eq!(report.segments_created, 0);
    assert!(f.queue.is_empty());
    assert_eq!(f.manager.pending_updates(), 1);
    f.apply();
    assert_eq!(f.manager.global_slot_of(EntryId(4)), Some(3));
}
#[test]
fn full_segments_make_room_for_a_new_one() {
    let f = Fixture::new();
    f.resident(SLOTS_PER_SEGMENT as u64);
    f.put(9, 2, 9.0);
    let report = f.manager.ensure(&set(&[9])).unwrap();
    assert_eq!(report.segments_created, 1);
    assert_eq!(f.queue.len(), 1);
    f.publish_queued();
    f.apply();
    assert_eq!(f.manager.global_slot_of(EntryId(9)), Some(SLOTS_PER_SEGMENT));
}
#[test]
fn equal_call_sequences_give_equal_slots() {
    let slots = |f: &Fixture| -> Vec<Option<u32>> {
        (1..=12).map(|id| f.manager.global_slot_of(EntryId(id))).collect()
    };
    let first = Fixture::new();
    first.resident(12);
    let second = Fixture::new();
    second.resident(12);
    assert_eq!(slots(& first), slots(& second));
    let expected: Vec<Option<u32>> = (0..12).map(Some).collect();
    assert_eq!(slots(& first), expected);
}
#[test]
fn reported_segments_are_closed_until_published() {
    let f = Fixture::new();
    f.put(1, 2, 1.0);
    f.put(2, 2, 2.0);
    f.manager.ensure(&set(&[1])).unwrap();
    f.manager.ensure(&set(&[2])).unwrap();
    let queued = f.queue.drain();
    assert_eq!(queued.len(), 2);
    assert_eq!(queued[0].bindings.len(), 1);
    assert_eq!(queued[1].bindings.len(), 1);
}
