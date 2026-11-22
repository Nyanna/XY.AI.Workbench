use super::*;

fn reference_fnv(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn reference_input(vector: &[u8], file: &str, start: u64, end: u64) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&(vector.len() as u64).to_le_bytes());
    buf.extend_from_slice(vector);
    buf.extend_from_slice(&(file.len() as u64).to_le_bytes());
    buf.extend_from_slice(file.as_bytes());
    buf.extend_from_slice(&start.to_le_bytes());
    buf.extend_from_slice(&end.to_le_bytes());
    buf
}

fn src() -> SourceRef {
    SourceRef::new("docs/a.txt", 3, 99)
}

#[test]
fn default_source_is_empty() {
    let s = SourceRef::default();
    assert_eq!(s, SourceRef::new("", 0, 0));
    assert!(s.to_meta().is_empty());
    assert_eq!(SourceRef::from_meta(&[]).unwrap(), s);
}

#[test]
fn default_checksum_is_fnv_over_32_zero_bytes() {
    let expected = reference_fnv(&[0u8; 32]);
    assert_eq!(compute_checksum(&[], &SourceRef::default()), expected);
    assert_ne!(expected, 0);
}

#[test]
fn matches_reference_for_sample_input() {
    let v = [1u8, 2, 3, 250, 0, 7];
    let s = SourceRef::new("é/日本\0x", u64::MAX, 5);
    let expected = reference_fnv(&reference_input(&v, &s.target_file, s.char_start, s.char_end));
    assert_eq!(compute_checksum(&v, &s), expected);
}

#[test]
fn parts_equal_struct_variant() {
    let v = [9u8; 13];
    let s = src();
    assert_eq!(
        compute_checksum(&v, &s),
        compute_checksum_parts(&v, &s.target_file, s.char_start, s.char_end)
    );
}

#[test]
fn length_prefixes_disambiguate_concatenation() {
    let a = compute_checksum(&[1, 2], &SourceRef::new("a", 0, 0));
    let b = compute_checksum(&[1], &SourceRef::new("\x02a", 0, 0));
    assert_ne!(a, b);
}

#[test]
fn single_byte_changes_are_detected() {
    let v = vec![10u8, 20, 30, 40];
    let s = src();
    let base = compute_checksum(&v, &s);
    for i in 0..v.len() {
        for bit in 0..8 {
            let mut w = v.clone();
            w[i] ^= 1 << bit;
            assert_ne!(compute_checksum(&w, &s), base);
        }
    }
    for bit in 0..64 {
        let mut t = s.clone();
        t.char_start ^= 1 << bit;
        assert_ne!(compute_checksum(&v, &t), base);
        let mut t = s.clone();
        t.char_end ^= 1 << bit;
        assert_ne!(compute_checksum(&v, &t), base);
    }
    let t = SourceRef::new("docs/b.txt", 3, 99);
    assert_ne!(compute_checksum(&v, &t), base);
}

#[test]
fn file_name_is_not_normalized() {
    let a = compute_checksum(&[], &SourceRef::new("a", 0, 0));
    assert_ne!(a, compute_checksum(&[], &SourceRef::new(" a", 0, 0)));
    assert_ne!(a, compute_checksum(&[], &SourceRef::new("a ", 0, 0)));
    assert_ne!(a, compute_checksum(&[], &SourceRef::new("A", 0, 0)));
}

#[test]
fn reversed_range_and_empty_inputs_are_valid() {
    let s = SourceRef::new("", 10, 2);
    let e = EntryData::new(SharedBytes::default(), s);
    assert!(e.verify().is_ok());
}

#[test]
fn verify_checksum_reports_both_values() {
    let v = [1u8, 2, 3];
    let s = src();
    let good = compute_checksum(&v, &s);
    assert_eq!(verify_checksum(&v, &s, good), Ok(()));
    let err = verify_checksum(&v, &s, good ^ 1).unwrap_err();
    assert_eq!(err.stored, good ^ 1);
    assert_eq!(err.computed, good);
    let text = err.to_string();
    assert!(text.contains(&format!("{:016x}", err.stored)));
    assert!(text.contains(&format!("{:016x}", err.computed)));
}

#[test]
fn entry_new_is_consistent() {
    let e = EntryData::new(SharedBytes::from(vec![1u8, 2, 3]), src());
    assert_eq!(e.checksum(), compute_checksum(&[1, 2, 3], &src()));
    assert!(e.verify().is_ok());
}

#[test]
fn entry_default_checksum_is_not_zero() {
    let e = EntryData::default();
    assert!(e.vector.is_empty());
    assert_eq!(e.source, SourceRef::default());
    assert_eq!(e.checksum(), compute_checksum(&[], &SourceRef::default()));
    assert!(e.verify().is_ok());
}

#[test]
fn from_stored_does_not_verify() {
    let good = EntryData::new(SharedBytes::from(vec![5u8]), src()).checksum();
    let ok = EntryData::from_stored(SharedBytes::from(vec![5u8]), src(), good);
    assert!(ok.verify().is_ok());
    let bad = EntryData::from_stored(SharedBytes::from(vec![5u8]), src(), good.wrapping_add(1));
    assert_eq!(bad.checksum(), good.wrapping_add(1));
    let err = bad.verify().unwrap_err();
    assert_eq!(err.stored, good.wrapping_add(1));
    assert_eq!(err.computed, good);
}

#[test]
fn mutation_requires_recompute() {
    let mut e = EntryData::new(SharedBytes::from(vec![1u8, 2]), src());
    let before = e.checksum();
    e.source.char_end += 1;
    assert!(e.verify().is_err());
    assert_eq!(e.checksum(), before);
    let after = e.recompute();
    assert_ne!(after, before);
    assert_eq!(e.checksum(), after);
    assert!(e.verify().is_ok());

    e.vector = SharedBytes::from(vec![9u8]);
    assert!(e.verify().is_err());
    e.recompute();
    assert!(e.verify().is_ok());
}

#[test]
fn meta_round_trip_and_conversions() {
    let s = src();
    let bytes = s.to_meta();
    assert_eq!(
        bytes,
        meta_codec::encode_meta(&MetaFields {
            target_file: s.target_file.clone(),
            char_start: 3,
            char_end: 99,
        })
    );
    assert_eq!(SourceRef::from_meta(&bytes).unwrap(), s);
    let fields = MetaFields::from(&s);
    assert_eq!(SourceRef::from(fields), s);
    assert!(SourceRef::from_meta(&bytes[..bytes.len() - 1]).is_err());
}

#[test]
fn types_are_send_sync_static() {
    fn check<T: Send + Sync + 'static>() {}
    check::<SourceRef>();
    check::<EntryData>();
    check::<ChecksumMismatch>();
}
