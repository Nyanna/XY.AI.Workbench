use super::*;

fn q(dim: u32, tokens: usize) -> Query {
    Query::new(dim, vec![1.0; dim as usize * tokens]).unwrap()
}

fn assert_send_sync<T: Send + Sync>() {}

#[test]
fn types_are_send_sync() {
    assert_send_sync::<SegmentId>();
    assert_send_sync::<ShapeError>();
    assert_send_sync::<SourceError3>();
    assert_send_sync::<Query>();
    assert_send_sync::<QueryBatch>();
    assert_send_sync::<RamSource>();
    assert_send_sync::<Workspace>();
}

#[test]
fn errors_display_and_implement_error() {
    fn is_error<E: std::error::Error + Send + Sync>(_: &E) {}
    let a = ShapeError::NotMultipleOfDim { len: 5, dim: 4 };
    let b = SourceError3::SlotRange {
        segment: SegmentId(3),
        start: 1,
        end: 9,
        slot_count: 4,
    };
    is_error(&a);
    is_error(&b);
    assert!(a.to_string().contains('5'));
    assert!(b.to_string().contains("1..9"));
    assert!(!ShapeError::ZeroDim.to_string().is_empty());
    assert!(!ShapeError::EmptyData.to_string().is_empty());
    assert!(!ShapeError::DimMismatch { expected: 1, got: 2 }
        .to_string()
        .is_empty());
}

#[test]
fn query_validation_order() {
    assert_eq!(Query::new(0, vec![]), Err(ShapeError::ZeroDim));
    assert_eq!(Query::new(0, vec![1.0]), Err(ShapeError::ZeroDim));
    assert_eq!(Query::new(4, vec![]), Err(ShapeError::EmptyData));
    assert_eq!(
        Query::new(4, vec![1.0; 6]),
        Err(ShapeError::NotMultipleOfDim { len: 6, dim: 4 })
    );
}

#[test]
fn query_keeps_data_unchanged() {
    let data = vec![3.0, -1.0, 2.5, 0.0, 9.0, 8.0];
    let query = Query::new(3, data.clone()).unwrap();
    assert_eq!(query.dim(), 3);
    assert_eq!(query.tokens(), 2);
    assert_eq!(query.data(), &data[..]);
    assert_eq!(&query.data()[3..6], &[0.0, 9.0, 8.0]);
}

#[test]
fn query_single_token() {
    let query = q(1, 1);
    assert_eq!(query.tokens(), 1);
}

#[test]
fn batch_new_is_empty() {
    let b = QueryBatch::new(8).unwrap();
    assert!(b.is_empty());
    assert_eq!(b.len(), 0);
    assert_eq!(b.dim(), 8);
    assert_eq!(QueryBatch::new(0), Err(ShapeError::ZeroDim));
}

#[test]
fn batch_push_keeps_order_and_indexes() {
    let mut b = QueryBatch::new(2).unwrap();
    b.push(q(2, 1)).unwrap();
    b.push(q(2, 3)).unwrap();
    assert_eq!(b.len(), 2);
    assert!(!b.is_empty());
    assert_eq!(b.tokens(0), Some(1));
    assert_eq!(b.tokens(1), Some(3));
    assert_eq!(b.data(1).unwrap().len(), 6);
    assert_eq!(b.get(0).unwrap().tokens(), 1);
    let tokens: Vec<usize> = b.iter().map(Query::tokens).collect();
    assert_eq!(tokens, vec![1, 3]);
    let via_ref: Vec<usize> = (&b).into_iter().map(Query::tokens).collect();
    assert_eq!(via_ref, tokens);
}

#[test]
fn batch_out_of_range_is_none() {
    let mut b = QueryBatch::new(2).unwrap();
    assert!(b.get(0).is_none());
    b.push(q(2, 1)).unwrap();
    assert!(b.get(1).is_none());
    assert_eq!(b.tokens(1), None);
    assert_eq!(b.data(1), None);
    assert_eq!(b.tokens(usize::MAX), None);
}

#[test]
fn batch_push_rejects_dim_mismatch_and_stays_unchanged() {
    let mut b = QueryBatch::new(2).unwrap();
    b.push(q(2, 1)).unwrap();
    let before = b.clone();
    assert_eq!(
        b.push(q(3, 1)),
        Err(ShapeError::DimMismatch {
            expected: 2,
            got: 3
        })
    );
    assert_eq!(b, before);
}

#[test]
fn batch_from_queries() {
    let b = QueryBatch::from_queries(2, vec![q(2, 1), q(2, 2)]).unwrap();
    assert_eq!(b.len(), 2);
    assert_eq!(b.tokens(1), Some(2));
    assert_eq!(
        QueryBatch::from_queries(0, vec![]),
        Err(ShapeError::ZeroDim)
    );
    assert!(QueryBatch::from_queries(2, vec![]).unwrap().is_empty());
}

#[test]
fn batch_from_queries_stops_at_first_error() {
    let r = QueryBatch::from_queries(2, vec![q(2, 1), q(4, 1), q(5, 1)]);
    assert_eq!(
        r,
        Err(ShapeError::DimMismatch {
            expected: 2,
            got: 4
        })
    );
}

fn sample_source() -> RamSource {
    let mut s = RamSource::new(2).unwrap();
    s.insert_segment(
        SegmentId(1),
        vec![
            Some(vec![1.0, 2.0, 3.0, 4.0]),
            None,
            Some(vec![]),
            Some(vec![5.0, 6.0]),
        ],
    )
    .unwrap();
    s
}

#[test]
fn ram_source_new_rejects_zero_dim() {
    assert!(matches!(RamSource::new(0), Err(ShapeError::ZeroDim)));
    assert_eq!(RamSource::new(7).unwrap().dim(), 7);
}

#[test]
fn ram_source_counts_and_unknown_segment() {
    let s = sample_source();
    assert_eq!(s.slot_count(SegmentId(1)), 4);
    assert_eq!(s.slot_count(SegmentId(2)), 0);
    assert_eq!(s.slot_offset(SegmentId(2), Slot5(0)), None);
    assert_eq!(s.slot_vectors(SegmentId(2), Slot5(0)), None);
}

#[test]
fn ram_source_offsets_are_prefix_sums_of_bytes() {
    let s = sample_source();
    let seg = SegmentId(1);
    assert_eq!(s.slot_offset(seg, Slot5(0)), Some(0));
    assert_eq!(s.slot_offset(seg, Slot5(1)), Some(16));
    assert_eq!(s.slot_offset(seg, Slot5(2)), Some(16));
    assert_eq!(s.slot_offset(seg, Slot5(3)), Some(16));
    assert_eq!(s.slot_offset(seg, Slot5(4)), None);
    assert_eq!(s.slot_offset(seg, Slot5(u32::MAX)), None);
}

#[test]
fn ram_source_vectors() {
    let s = sample_source();
    let seg = SegmentId(1);
    assert_eq!(s.slot_vectors(seg, Slot5(0)), Some(&[1.0, 2.0, 3.0, 4.0][..]));
    assert_eq!(s.slot_vectors(seg, Slot5(1)), None);
    assert_eq!(s.slot_vectors(seg, Slot5(2)), Some(&[][..]));
    assert_eq!(s.slot_vectors(seg, Slot5(3)), Some(&[5.0, 6.0][..]));
    assert_eq!(s.slot_vectors(seg, Slot5(4)), None);
}

#[test]
fn ram_source_insert_rejects_bad_length_without_effect() {
    let mut s = sample_source();
    let r = s.insert_segment(SegmentId(1), vec![Some(vec![1.0, 2.0]), Some(vec![1.0; 3])]);
    assert_eq!(r, Err(ShapeError::NotMultipleOfDim { len: 3, dim: 2 }));
    assert_eq!(s.slot_count(SegmentId(1)), 4);
    assert_eq!(s.slot_offset(SegmentId(1), Slot5(3)), Some(16));
    let r = s.insert_segment(SegmentId(9), vec![Some(vec![1.0])]);
    assert!(r.is_err());
    assert_eq!(s.slot_count(SegmentId(9)), 0);
}

#[test]
fn ram_source_insert_replaces_segment() {
    let mut s = sample_source();
    s.insert_segment(SegmentId(1), vec![None, Some(vec![7.0, 8.0])])
        .unwrap();
    assert_eq!(s.slot_count(SegmentId(1)), 2);
    assert_eq!(s.slot_offset(SegmentId(1), Slot5(1)), Some(0));
    assert_eq!(s.slot_vectors(SegmentId(1), Slot5(0)), None);
    assert_eq!(s.slot_vectors(SegmentId(1), Slot5(1)), Some(&[7.0, 8.0][..]));
    assert_eq!(s.slot_offset(SegmentId(1), Slot5(2)), None);
}

#[test]
fn ram_source_empty_segment() {
    let mut s = RamSource::new(2).unwrap();
    s.insert_segment(SegmentId(5), vec![]).unwrap();
    assert_eq!(s.slot_count(SegmentId(5)), 0);
    assert_eq!(s.slot_offset(SegmentId(5), Slot5(0)), None);
}

#[test]
fn ram_source_usable_as_trait_object_across_threads() {
    let s: std::sync::Arc<dyn DocSource> = std::sync::Arc::new(sample_source());
    let handles: Vec<_> = (0..4)
        .map(|_| {
            let s = s.clone();
            std::thread::spawn(move || s.slot_offset(SegmentId(1), Slot5(3)))
        })
        .collect();
    for h in handles {
        assert_eq!(h.join().unwrap(), Some(16));
    }
}

#[test]
fn check_slot_range_accepts_valid_ranges() {
    let s = sample_source();
    let seg = SegmentId(1);
    assert_eq!(check_slot_range(&s, seg, 0..4), Ok(()));
    assert_eq!(check_slot_range(&s, seg, 1..3), Ok(()));
    assert_eq!(check_slot_range(&s, seg, 0..0), Ok(()));
    assert_eq!(check_slot_range(&s, seg, 4..4), Ok(()));
}

#[test]
fn check_slot_range_rejects_invalid_ranges() {
    let s = sample_source();
    let seg = SegmentId(1);
    let err = |start, end| {
        Err(SourceError3::SlotRange {
            segment: seg,
            start,
            end,
            slot_count: 4,
        })
    };
    assert_eq!(check_slot_range(&s, seg, 0..5), err(0, 5));
    assert_eq!(check_slot_range(&s, seg, 5..5), err(5, 5));
    #[allow(clippy::reversed_empty_ranges)]
    let reversed = 3..2;
    assert_eq!(check_slot_range(&s, seg, reversed), err(3, 2));
}

#[test]
fn check_slot_range_unknown_segment_only_empty_zero() {
    let s = sample_source();
    let seg = SegmentId(77);
    assert_eq!(check_slot_range(&s, seg, 0..0), Ok(()));
    assert_eq!(
        check_slot_range(&s, seg, 0..1),
        Err(SourceError3::SlotRange {
            segment: seg,
            start: 0,
            end: 1,
            slot_count: 0
        })
    );
    assert!(check_slot_range(&s, seg, 1..1).is_err());
}

#[test]
fn workspace_new_and_accessors() {
    let w = Workspace::new(2, 10);
    assert_eq!((w.start(), w.end(), w.len()), (2, 10, 8));
    assert_eq!(w.range(), 2..10);
    assert!(!w.is_empty());
}

#[test]
fn workspace_reversed_becomes_empty_at_start() {
    let w = Workspace::new(9, 3);
    assert_eq!((w.start(), w.end(), w.len()), (9, 9, 0));
    assert!(w.is_empty());
    assert!(Workspace::new(4, 4).is_empty());
}

#[test]
fn workspace_split_front_basic() {
    let (f, r) = Workspace::new(10, 25).split_front(4);
    assert_eq!(f, Workspace::new(10, 14));
    assert_eq!(r, Workspace::new(14, 25));
}

#[test]
fn workspace_split_front_edges() {
    let w = Workspace::new(3, 8);
    let (f, r) = w.split_front(0);
    assert_eq!((f, r), (Workspace::new(3, 3), w));
    let (f, r) = w.split_front(5);
    assert_eq!((f, r), (w, Workspace::new(8, 8)));
    let (f, r) = w.split_front(u32::MAX);
    assert_eq!((f, r), (w, Workspace::new(8, 8)));
    let (f, r) = Workspace::new(6, 6).split_front(3);
    assert!(f.is_empty() && r.is_empty());
}

#[test]
fn workspace_split_front_no_overflow_at_u32_max() {
    let w = Workspace::new(u32::MAX - 2, u32::MAX);
    let (f, r) = w.split_front(u32::MAX);
    assert_eq!(f, w);
    assert!(r.is_empty());
    let w = Workspace::new(0, u32::MAX);
    let (f, r) = w.split_front(u32::MAX - 1);
    assert_eq!(f.len(), u32::MAX - 1);
    assert_eq!(r.len(), 1);
}

#[test]
fn workspace_repeated_split_is_gapless() {
    let mut rest = Workspace::new(5, 105);
    let mut cursor = 5;
    let mut launches = 0;
    while !rest.is_empty() {
        let (f, r) = rest.split_front(32);
        assert!(f.len() <= 32 && !f.is_empty());
        assert_eq!(f.start(), cursor);
        assert_eq!(r.start(), f.end());
        assert_eq!(f.len() + r.len(), rest.len());
        cursor = f.end();
        rest = r;
        launches += 1;
    }
    assert_eq!(cursor, 105);
    assert_eq!(launches, 4);
}
