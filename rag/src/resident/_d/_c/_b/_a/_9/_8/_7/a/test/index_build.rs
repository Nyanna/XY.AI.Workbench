use super::*;

use std::sync::Arc;

use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::segment_arena::SegmentArenaConfig;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::allocator::Allocator as Alloc;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::bookkeeping::Bookkeeping;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::arenas::{
    ArenaCapacities, ArenaKind, ArenaSet, Budget,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::DeviceKind;

fn arena(capacity: u64) -> SegmentArena<Arc<Alloc>> {
    let mut caps = ArenaCapacities::default();
    caps.set(ArenaKind::IndexSegments, capacity);
    let set = Arc::new(ArenaSet::new(caps));
    let budget = Budget {
        free: u64::MAX / 2,
        total: u64::MAX / 2,
        generation_reserve: 0,
    };
    let seg = set.ensure(ArenaKind::IndexSegments, &budget).unwrap();
    let bk = Arc::new(Bookkeeping::new(set));
    let alloc = Arc::new(Alloc::new(seg, DeviceKind::Gpu, bk));
    SegmentArena::new(alloc, SegmentArenaConfig::default()).unwrap()
}

#[test]
fn begin_errors_leave_nothing_allocated() {
    let a = arena(8192);
    assert_eq!(
        IndexBuild::begin(&a, 0).err(),
        Some(IndexBuildError::Arena(SegmentArenaError::ZeroLength))
    );
    assert!(matches!(
        IndexBuild::begin(&a, u64::MAX),
        Err(IndexBuildError::Arena(SegmentArenaError::Exhausted { .. }))
    ));
    assert_eq!(a.stats().live_segments, 0);
}

#[test]
fn spans_are_aligned_disjoint_and_accounted() {
    let a = arena(8192);
    let mut b = IndexBuild::begin(&a, 1000).unwrap();
    let seg = *b.segment();
    assert_eq!(b.remaining(), seg.len);
    let s1 = b.reserve(3, 1).unwrap();
    let s2 = b.reserve(16, 64).unwrap();
    let s3 = b.reserve(0, 32).unwrap();
    assert_eq!(s1.offset, seg.offset);
    assert_eq!(s2.offset % 64, 0);
    assert!(s1.end() <= s2.offset && s2.end() <= s3.offset);
    assert_eq!(s3.len, 0);
    assert!(s3.end() <= seg.offset + seg.len);
    assert_eq!(b.written() + b.remaining(), seg.len);
}

#[test]
fn failed_reserve_keeps_cursor() {
    let a = arena(8192);
    let mut b = IndexBuild::begin(&a, 512).unwrap();
    b.reserve(10, 1).unwrap();
    let w = b.written();
    let rem = b.remaining();
    assert_eq!(
        b.reserve(1, 3),
        Err(IndexBuildError::Arena(SegmentArenaError::InvalidAlignment))
    );
    assert_eq!(
        b.reserve(1, 0),
        Err(IndexBuildError::Arena(SegmentArenaError::InvalidAlignment))
    );
    assert_eq!(
        b.reserve(rem + 1, 1),
        Err(IndexBuildError::Arena(SegmentArenaError::Exhausted {
            requested: rem + 1,
            available: rem
        }))
    );
    assert!(b.reserve(u64::MAX, 1).is_err());
    assert_eq!(b.written(), w);
    assert_eq!(b.remaining(), rem);
}

#[test]
fn finish_keeps_segment_live_and_builds_handle() {
    let a = arena(8192);
    let mut b = IndexBuild::begin(&a, 512).unwrap();
    b.reserve(100, 8).unwrap();
    let built = b.finish();
    assert_eq!(built.written, 100);
    assert_eq!(a.stats().live_segments, 1);
    let h = built.handle(Generation(7)).unwrap();
    assert_eq!(h.arena(), ArenaKind::IndexSegments);
    assert_eq!(h.offset(), built.segment.offset);
    assert_eq!(h.len(), 100);
    assert_eq!(h.generation(), Generation(7));
    a.release(&built.segment, Readers::None).unwrap();
    assert_eq!(a.stats().live_segments, 0);
}

#[test]
fn empty_handle_and_overflow() {
    let a = arena(8192);
    let built = IndexBuild::begin(&a, 256).unwrap().finish();
    assert_eq!(built.written, 0);
    assert!(built.handle(Generation(1)).unwrap().is_empty());
    let mut bad = built;
    bad.segment.offset = u64::MAX;
    bad.written = 2;
    assert_eq!(
        bad.handle(Generation(1)),
        Err(IndexBuildError::Handle(RegionError::Overflow))
    );
}

#[test]
fn abort_releases_and_double_release_is_stale() {
    let a = arena(8192);
    let b = IndexBuild::begin(&a, 512).unwrap();
    let seg = *b.segment();
    b.abort(Readers::None).unwrap();
    assert_eq!(a.stats().live_segments, 0);
    assert_eq!(
        a.release(&seg, Readers::None),
        Err(SegmentArenaError::StaleRegion)
    );
}

#[test]
fn builds_are_disjoint_and_error_displays() {
    let a = arena(8192);
    let b1 = IndexBuild::begin(&a, 512).unwrap();
    let b2 = IndexBuild::begin(&a, 512).unwrap();
    let (s1, s2) = (*b1.segment(), *b2.segment());
    assert!(s1.offset + s1.len <= s2.offset || s2.offset + s2.len <= s1.offset);
    let e: IndexBuildError = SegmentArenaError::ZeroLength.into();
    assert!(!e.to_string().is_empty());
    let e: Box<dyn std::error::Error> = Box::new(IndexBuildError::Handle(RegionError::Overflow));
    assert!(e.source().is_some());
}
