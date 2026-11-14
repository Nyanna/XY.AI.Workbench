use super::*;
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::arenas::ArenaKind;
use std::sync::Arc;
static_assertions::assert_impl_all!(SegmentView : Send, Sync);
static_assertions::assert_impl_all!(Descriptor : Send, Sync);
static_assertions::assert_impl_all!(SlotClass : Send, Sync);
static_assertions::assert_impl_all!(EntryRef : Send, Sync);
static_assertions::assert_impl_all!(SlotMeta : Send, Sync);
static_assertions::assert_impl_all!(SlotStates : Send, Sync);
static_assertions::assert_impl_all!(SlotAssignment : Send, Sync);
static_assertions::assert_impl_all!(SlotError : Send, Sync);
fn region(offset: u64, len: u64) -> RegionHandle {
    RegionHandle::new(ArenaKind::IndexSegments, offset, len, Generation(7))
        .expect("region")
}
fn view(slot_count: u32, region_len: u64) -> SegmentView {
    SegmentView::new(
            region(4096, region_len),
            slot_count,
            SlotClass::C32,
            4,
            DType::F32,
            Generation(3),
        )
        .expect("view")
}
fn eref(id: u64, section: u32) -> EntryRef {
    EntryRef {
        entry: EntryId(id),
        section,
    }
}
#[test]
fn class_constants_and_custom() {
    assert_eq!(SlotClass::C32.max_tokens(), 32);
    assert_eq!(SlotClass::C64.max_tokens(), 64);
    assert_eq!(SlotClass::C128.max_tokens(), 128);
    assert_eq!(SlotClass::custom(32).unwrap(), SlotClass::C32);
    assert_eq!(SlotClass::custom(65_535).unwrap().max_tokens(), 65_535);
    assert_eq!(SlotClass::custom(0), Err(SlotError::InvalidClass { max_tokens : 0 }));
    assert_eq!(
        SlotClass::custom(65_536), Err(SlotError::InvalidClass { max_tokens : 65_536 })
    );
}
#[test]
fn slot_size_values_and_errors() {
    assert_eq!(slot_size(SlotClass::C32, 128, DType::F16), Ok(8192));
    assert_eq!(slot_size(SlotClass::C32, 128, DType::F32), Ok(16384));
    assert_eq!(slot_size(SlotClass::C32, 1, DType::U8), Ok(32));
    assert_eq!(
        slot_size(SlotClass::C32, 0, DType::F32), Err(SlotError::InvalidDimension)
    );
    assert_eq!(
        slot_size(SlotClass::C128, u32::MAX, DType::F32),
        Err(SlotError::SlotSizeOverflow)
    );
    let huge = SlotClass::custom(65_535).unwrap();
    assert_eq!(slot_size(huge, u32::MAX, DType::I64), Err(SlotError::SlotSizeOverflow));
    assert_eq!(
        slot_size(SlotClass::C32, 134_217_728, DType::F32),
        Err(SlotError::SlotSizeOverflow)
    );
    assert_eq!(
        slot_size(SlotClass::C32, 33_554_431, DType::F32), Ok(32 * 33_554_431 * 4)
    );
}
#[test]
fn encode_f32_rows_and_padding() {
    let class = SlotClass::custom(3).unwrap();
    let bytes = encode_slot(class, 2, DType::F32, 2, &[1.0, 2.0, 3.0, 4.0]).unwrap();
    assert_eq!(bytes.len(), 3 * 2 * 4);
    let words: Vec<f32> = bytes
        .chunks(4)
        .map(|c| f32::from_le_bytes(c.try_into().unwrap()))
        .collect();
    assert_eq!(words, vec![1.0, 2.0, 3.0, 4.0, MASK_VALUE, MASK_VALUE]);
}
#[test]
fn encode_zero_tokens_is_all_padding() {
    let class = SlotClass::custom(2).unwrap();
    let bytes = encode_slot(class, 2, DType::F16, 0, &[]).unwrap();
    assert_eq!(bytes.len(), 2 * 2 * 2);
    let mask = f32_to_f16_bits(MASK_VALUE).to_le_bytes();
    for chunk in bytes.chunks(2) {
        assert_eq!(chunk, mask);
    }
}
#[test]
fn encode_error_order() {
    let class = SlotClass::custom(2).unwrap();
    assert_eq!(
        encode_slot(class, 0, DType::U8, 9, & []), Err(SlotError::InvalidDimension)
    );
    assert_eq!(
        encode_slot(class, 2, DType::U8, 9, & []), Err(SlotError::UnsupportedDType {
        dtype : DType::U8 })
    );
    assert_eq!(
        encode_slot(class, 2, DType::F32, 3, & []), Err(SlotError::TooManyTokens { tokens
        : 3, max : 2 })
    );
    assert_eq!(
        encode_slot(class, 2, DType::F32, 2, & [1.0]), Err(SlotError::ValueCountMismatch
        { expected : 4, got : 1 })
    );
    for dt in [DType::U32, DType::I64] {
        assert!(
            matches!(encode_slot(class, 2, dt, 0, & []), Err(SlotError::UnsupportedDType
            { .. }))
        );
    }
}
#[test]
fn f16_known_values() {
    assert_eq!(f32_to_f16_bits(0.0), 0x0000);
    assert_eq!(f32_to_f16_bits(- 0.0), 0x8000);
    assert_eq!(f32_to_f16_bits(1.0), 0x3c00);
    assert_eq!(f32_to_f16_bits(- 2.0), 0xc000);
    assert_eq!(f32_to_f16_bits(65504.0), 0x7bff);
    assert_eq!(f32_to_f16_bits(65520.0), 0x7c00);
    assert_eq!(f32_to_f16_bits(1.0e9), 0x7c00);
    assert_eq!(f32_to_f16_bits(f32::NEG_INFINITY), 0xfc00);
    assert_eq!(f32_to_f16_bits(f32::from_bits(0x3380_0000)), 0x0001);
    assert_eq!(f32_to_f16_bits(MASK_VALUE), 0xf0e2);
    assert_eq!(f32_to_f16_bits(1.0e-10), 0x0000);
    let nan = f32_to_f16_bits(f32::NAN);
    assert_eq!(nan & 0x7c00, 0x7c00);
    assert_ne!(nan & 0x03ff, 0);
    let tiny_nan = f32_to_f16_bits(f32::from_bits(0x7f80_0001));
    assert_ne!(tiny_nan & 0x03ff, 0);
}
#[test]
fn f16_matches_reference_on_sampled_bits() {
    let mut bits: u32 = 0;
    loop {
        let v = f32::from_bits(bits);
        let got = f32_to_f16_bits(v);
        let want = half::f16::from_f32(v).to_bits();
        if v.is_nan() {
            assert_eq!(got & 0x7c00, 0x7c00);
            assert_ne!(got & 0x03ff, 0);
        } else {
            assert_eq!(got, want, "bits {bits:#010x}");
        }
        match bits.checked_add(4099) {
            Some(n) => bits = n,
            None => break,
        }
    }
    for exp in 90u32..145 {
        for man in [
            0u32, 1, 0x1000, 0x1fff, 0x2000, 0x3000, 0x7f_ffff, 0x7f_e000, 0x7f_f000,
        ] {
            for sign in [0u32, 0x8000_0000] {
                let v = f32::from_bits(sign | (exp << 23) | man);
                assert_eq!(
                    f32_to_f16_bits(v), half::f16::from_f32(v).to_bits(),
                    "bits {:#010x}", v.to_bits()
                );
            }
        }
    }
}
#[test]
fn bf16_matches_reference() {
    assert_eq!(f32_to_bf16_bits(1.0), 0x3f80);
    assert_eq!(f32_to_bf16_bits(MASK_VALUE), half::bf16::from_f32(MASK_VALUE).to_bits());
    let nan = f32_to_bf16_bits(f32::NAN);
    assert_eq!(nan & 0x7f80, 0x7f80);
    assert_ne!(nan & 0x007f, 0);
    assert_ne!(f32_to_bf16_bits(f32::from_bits(0x7f80_0001)) & 0x007f, 0);
    assert_eq!(f32_to_bf16_bits(f32::MAX), 0x7f80);
    let mut bits: u32 = 0;
    loop {
        let v = f32::from_bits(bits);
        let got = f32_to_bf16_bits(v);
        if v.is_nan() {
            assert_eq!(got & 0x7f80, 0x7f80);
            assert_ne!(got & 0x007f, 0);
        } else {
            assert_eq!(got, half::bf16::from_f32(v).to_bits(), "bits {bits:#010x}");
        }
        match bits.checked_add(4099) {
            Some(n) => bits = n,
            None => break,
        }
    }
}
#[test]
fn mask_padding_is_finite_and_nonzero_in_all_dtypes() {
    assert_ne!(f32_to_f16_bits(MASK_VALUE) & 0x7fff, 0);
    assert_ne!(f32_to_f16_bits(MASK_VALUE) & 0x7c00, 0x7c00);
    assert_ne!(f32_to_bf16_bits(MASK_VALUE) & 0x7fff, 0);
    assert_ne!(f32_to_bf16_bits(MASK_VALUE) & 0x7f80, 0x7f80);
}
#[test]
fn view_construction_and_accessors() {
    let r = region(4096, 2000);
    let v = SegmentView::new(r, 3, SlotClass::C32, 4, DType::F32, Generation(9))
        .unwrap();
    assert_eq!(v.slot_bytes(), 512);
    assert_eq!(v.total_bytes(), 1536);
    assert_eq!(v.generation(), Generation(9));
    assert_eq!(v.region(), r);
    assert_eq!(v.slot_count(), 3);
    assert_eq!(v.dim(), 4);
    assert_eq!(v.dtype(), DType::F32);
    assert_eq!(v.class(), SlotClass::C32);
}
#[test]
fn view_region_too_small_and_larger_ok() {
    let r = region(0, 1000);
    assert_eq!(
        SegmentView::new(r, 2, SlotClass::C32, 4, DType::F32, Generation(0)),
        Err(SlotError::RegionTooSmall { needed : 1024, available : 1000 })
    );
    assert!(
        SegmentView::new(region(0, 5000), 2, SlotClass::C32, 4, DType::F32,
        Generation(0)).is_ok()
    );
    assert!(
        SegmentView::new(region(0, 0), 0, SlotClass::C32, 4, DType::F32, Generation(0))
        .is_ok()
    );
    assert_eq!(
        SegmentView::new(r, 1, SlotClass::C32, 0, DType::F32, Generation(0)),
        Err(SlotError::InvalidDimension)
    );
}
#[test]
fn view_slot_range() {
    let v = view(3, 4096);
    assert_eq!(v.slot_range(Slot(0)).unwrap(), 0..512);
    assert_eq!(v.slot_range(Slot(2)).unwrap(), 1024..1536);
    assert_eq!(
        v.slot_range(Slot(3)), Err(SlotError::SlotOutOfRange { slot : 3, count : 3 })
    );
}
#[test]
fn descriptor_and_fixed_layout() {
    let v = view(3, 4096);
    let d = v.descriptor(Addr(1000)).unwrap();
    assert_eq!(d, Descriptor { start : Addr(1000), slot_count : 3, stride : 512 });
    assert_eq!(d.to_fixed_layout().unwrap(), FixedLayout::new(1000, 512, 3).unwrap());
    assert_eq!(
        v.descriptor(Addr(u64::MAX - 100)),
        Err(SlotError::Addr(AddrLayoutError::Overflow))
    );
    assert!(v.descriptor(Addr(u64::MAX - 1536)).is_ok());
}
#[test]
fn view_encode_uses_view_parameters() {
    let v = view(1, 512);
    let bytes = v.encode_slot(1, &[1.0, 2.0, 3.0, 4.0]).unwrap();
    assert_eq!(bytes.len() as u32, v.slot_bytes());
    assert_eq!(& bytes[..4], & 1.0f32.to_le_bytes());
    assert_eq!(& bytes[16..20], & MASK_VALUE.to_le_bytes());
}
#[test]
fn states_basic() {
    let v = view(70, 70 * 512);
    let s = SlotStates::new(&v);
    assert_eq!(s.slot_count(), 70);
    assert_eq!(s.valid_count(), 0);
    assert_eq!(s.snapshot_words(), vec![0, 0]);
    s.set_filled(Slot(0), 5).unwrap();
    s.set_filled(Slot(65), 0).unwrap();
    assert!(s.is_valid(Slot(0)));
    assert!(s.is_valid(Slot(65)));
    assert!(! s.is_valid(Slot(1)));
    assert!(! s.is_valid(Slot(70)));
    assert_eq!(s.valid_count(), 2);
    assert_eq!(s.snapshot_words(), vec![1, 2]);
    assert_eq!(s.meta(Slot(0)).unwrap(), SlotMeta { valid : true, tokens : 5 });
    assert_eq!(s.meta(Slot(65)).unwrap(), SlotMeta { valid : true, tokens : 0 });
    assert_eq!(s.meta(Slot(2)).unwrap(), SlotMeta { valid : false, tokens : 0 });
    s.invalidate(Slot(0)).unwrap();
    s.invalidate(Slot(0)).unwrap();
    assert_eq!(s.meta(Slot(0)).unwrap(), SlotMeta { valid : false, tokens : 0 });
    assert_eq!(s.valid_count(), 1);
}
#[test]
fn states_errors_and_empty() {
    let v = view(2, 2048);
    let s = SlotStates::new(&v);
    assert_eq!(
        s.set_filled(Slot(2), 1), Err(SlotError::SlotOutOfRange { slot : 2, count : 2 })
    );
    assert_eq!(
        s.set_filled(Slot(5), 99), Err(SlotError::SlotOutOfRange { slot : 5, count : 2 })
    );
    assert_eq!(
        s.set_filled(Slot(0), 33), Err(SlotError::TokensExceedClass { tokens : 33, max :
        32 })
    );
    assert!(! s.is_valid(Slot(0)));
    assert!(s.set_filled(Slot(0), 32).is_ok());
    assert_eq!(
        s.invalidate(Slot(2)), Err(SlotError::SlotOutOfRange { slot : 2, count : 2 })
    );
    assert_eq!(s.meta(Slot(2)), Err(SlotError::SlotOutOfRange { slot : 2, count : 2 }));
    let empty = SlotStates::new(&view(0, 0));
    assert!(empty.snapshot_words().is_empty());
    assert_eq!(empty.valid_count(), 0);
}
#[test]
fn states_snapshot_word_boundaries() {
    let v = view(128, 128 * 512);
    let s = SlotStates::new(&v);
    s.set_filled(Slot(63), 1).unwrap();
    s.set_filled(Slot(64), 1).unwrap();
    s.set_filled(Slot(127), 1).unwrap();
    assert_eq!(s.snapshot_words(), vec![1u64 << 63, 1 | (1u64 << 63)]);
}
#[test]
fn states_concurrent_same_word() {
    let v = view(64, 64 * 512);
    let s = Arc::new(SlotStates::new(&v));
    let handles: Vec<_> = (0..8u32)
        .map(|t| {
            let s = Arc::clone(&s);
            std::thread::spawn(move || {
                for round in 0..200u32 {
                    for i in 0..8u32 {
                        let slot = Slot(t * 8 + i);
                        s.set_filled(slot, (round % 32) + 1).unwrap();
                        if round % 2 == 0 {
                            s.invalidate(slot).unwrap();
                        }
                    }
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    assert_eq!(s.valid_count(), 64);
    for i in 0..64 {
        assert_eq!(s.meta(Slot(i)).unwrap(), SlotMeta { valid : true, tokens : 8 });
    }
}
#[test]
fn assignment_fixed_bind_lookup_unbind() {
    let v = view(4, 4 * 512);
    let a = SlotAssignment::fixed(&v, Addr(1 << 20)).unwrap();
    assert_eq!(a.slot_count(), 4);
    assert_eq!(a.spanning_range(), None);
    a.bind(Slot(1), eref(10, 0)).unwrap();
    a.bind(Slot(3), eref(10, 1)).unwrap();
    assert_eq!(a.lookup(Slot(1)), Some(eref(10, 0)));
    assert_eq!(a.lookup(Slot(0)), None);
    assert_eq!(a.lookup(Slot(99)), None);
    assert_eq!(a.slot_of_entry(eref(10, 1)), Some(Slot(3)));
    assert_eq!(a.slot_of_entry(eref(11, 0)), None);
    assert_eq!(a.device_addr(Slot(2)).unwrap(), Addr((1 << 20) + 1024));
    assert_eq!(a.byte_range(Slot(1)).unwrap(), 4608..5120);
    assert_eq!(a.spanning_range(), Some(4608..6144));
    assert_eq!(a.unbind(Slot(3)).unwrap(), Some(eref(10, 1)));
    assert_eq!(a.unbind(Slot(3)).unwrap(), None);
    assert_eq!(a.slot_of_entry(eref(10, 1)), None);
    assert_eq!(a.spanning_range(), Some(4608..5120));
}
#[test]
fn assignment_bind_error_order() {
    let v = view(2, 1024);
    let a = SlotAssignment::fixed(&v, Addr(0)).unwrap();
    a.bind(Slot(0), eref(1, 0)).unwrap();
    assert_eq!(
        a.bind(Slot(2), eref(1, 0)), Err(SlotError::SlotOutOfRange { slot : 2, count : 2
        })
    );
    assert_eq!(
        a.bind(Slot(0), eref(1, 0)), Err(SlotError::SlotOccupied { slot : 0, entry :
        eref(1, 0) })
    );
    assert_eq!(
        a.bind(Slot(0), eref(2, 0)), Err(SlotError::SlotOccupied { slot : 0, entry :
        eref(1, 0) })
    );
    assert_eq!(
        a.bind(Slot(1), eref(1, 0)), Err(SlotError::EntryAlreadyBound { entry : eref(1,
        0), slot : 0 })
    );
    assert_eq!(
        a.unbind(Slot(2)), Err(SlotError::SlotOutOfRange { slot : 2, count : 2 })
    );
    assert_eq!(
        a.byte_range(Slot(2)), Err(SlotError::SlotOutOfRange { slot : 2, count : 2 })
    );
    assert_eq!(
        a.device_addr(Slot(2)), Err(SlotError::SlotOutOfRange { slot : 2, count : 2 })
    );
}
#[test]
fn assignment_fixed_zero_slots() {
    let v = view(0, 0);
    let a = SlotAssignment::fixed(&v, Addr(0)).unwrap();
    assert_eq!(a.slot_count(), 0);
    assert_eq!(a.spanning_range(), None);
    assert!(a.bind(Slot(0), eref(1, 0)).is_err());
}
#[test]
fn assignment_fixed_overflow() {
    let v = view(2, 1024);
    assert_eq!(
        SlotAssignment::fixed(& v, Addr(u64::MAX - 10)).err(),
        Some(SlotError::Addr(AddrLayoutError::Overflow))
    );
}
#[test]
fn assignment_variable() {
    let r = region(1000, 100);
    let a = SlotAssignment::variable(r, Addr(50_000), &[10, 0, 30]).unwrap();
    assert_eq!(a.slot_count(), 3);
    assert_eq!(a.device_addr(Slot(2)).unwrap(), Addr(50_010));
    assert_eq!(a.byte_range(Slot(0)).unwrap(), 1000..1010);
    assert_eq!(a.byte_range(Slot(1)).unwrap(), 1010..1010);
    assert_eq!(a.byte_range(Slot(2)).unwrap(), 1010..1040);
    a.bind(Slot(2), eref(5, 2)).unwrap();
    a.bind(Slot(0), eref(6, 0)).unwrap();
    assert_eq!(a.spanning_range(), Some(1000..1040));
    assert_eq!(
        a.byte_range(Slot(3)), Err(SlotError::SlotOutOfRange { slot : 3, count : 3 })
    );
}
#[test]
fn assignment_variable_errors() {
    let r = region(0, 100);
    assert_eq!(
        SlotAssignment::variable(r, Addr(0), & [60, 41]).err(),
        Some(SlotError::RegionTooSmall { needed : 101, available : 100 })
    );
    assert!(SlotAssignment::variable(r, Addr(0), & [60, 40]).is_ok());
    assert_eq!(
        SlotAssignment::variable(r, Addr(0), & [u64::MAX, 1]).err(),
        Some(SlotError::Addr(AddrLayoutError::Overflow))
    );
    let empty = SlotAssignment::variable(r, Addr(0), &[]).unwrap();
    assert_eq!(empty.slot_count(), 0);
}
#[test]
fn assignment_concurrent_bind() {
    let v = view(64, 64 * 512);
    let a = Arc::new(SlotAssignment::fixed(&v, Addr(0)).unwrap());
    let handles: Vec<_> = (0..8u32)
        .map(|t| {
            let a = Arc::clone(&a);
            std::thread::spawn(move || {
                for i in 0..8u32 {
                    let slot = Slot(t * 8 + i);
                    a.bind(slot, eref(u64::from(slot.0), 0)).unwrap();
                    assert_eq!(a.slot_of_entry(eref(u64::from(slot.0), 0)), Some(slot));
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
    for i in 0..64u32 {
        assert_eq!(a.lookup(Slot(i)), Some(eref(u64::from(i), 0)));
    }
    assert_eq!(a.spanning_range(), Some(4096..4096 + 64 * 512));
}
#[test]
fn assignment_survives_poisoned_lock() {
    let v = view(2, 1024);
    let a = Arc::new(SlotAssignment::fixed(&v, Addr(0)).unwrap());
    let a2 = Arc::clone(&a);
    let _ = std::thread::spawn(move || {
            let _g = a2.bindings.write().unwrap();
            panic!("poison");
        })
        .join();
    a.bind(Slot(0), eref(1, 0)).unwrap();
    assert_eq!(a.lookup(Slot(0)), Some(eref(1, 0)));
}
#[test]
fn error_display_and_source() {
    let e = SlotError::Addr(AddrLayoutError::Overflow);
    assert!(! e.to_string().is_empty());
    assert!(std::error::Error::source(& e).is_some());
    assert!(! SlotError::InvalidDimension.to_string().is_empty());
    assert!(std::error::Error::source(& SlotError::InvalidDimension).is_none());
}
