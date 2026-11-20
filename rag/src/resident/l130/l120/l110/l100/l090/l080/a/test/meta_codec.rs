use super::*;

fn fields(name: &str, start: u64, end: u64) -> MetaFields {
    MetaFields {
        target_file: name.to_string(),
        char_start: start,
        char_end: end,
    }
}

fn raw(name: &[u8], declared: u64, start: u64, end: u64) -> Vec<u8> {
    let mut v = declared.to_le_bytes().to_vec();
    v.extend_from_slice(name);
    v.extend_from_slice(&start.to_le_bytes());
    v.extend_from_slice(&end.to_le_bytes());
    v
}

#[test]
fn default_encodes_to_empty() {
    assert!(encode_meta(&MetaFields::default()).is_empty());
    assert!(encode_meta_parts("", 0, 0).is_empty());
    assert_eq!(decode_meta(&[]), Ok(MetaFields::default()));
}

#[test]
fn layout_is_length_prefixed_little_endian() {
    let bytes = encode_meta_parts("ab", 0x0102, 0x0304);
    assert_eq!(bytes.len(), 24 + 2);
    assert_eq!(&bytes[..8], &2u64.to_le_bytes());
    assert_eq!(&bytes[8..10], b"ab");
    assert_eq!(&bytes[10..18], &0x0102u64.to_le_bytes());
    assert_eq!(&bytes[18..26], &0x0304u64.to_le_bytes());
}

#[test]
fn round_trips() {
    let cases = [
        fields("", 1, 0),
        fields("", 0, 5),
        fields("", u64::MAX, u64::MAX),
        fields("src/lib.rs", 0, 0),
        fields("src/lib.rs", 10, 20),
        fields("  padded name \n", 3, 4),
        fields("日本語/ファイル🦀.txt", 7, 9),
        fields("a\0b\0", 1, 2),
        fields("backwards", 100, 3),
    ];
    for f in cases {
        let bytes = encode_meta(&f);
        assert_eq!(decode_meta(&bytes), Ok(f.clone()), "{f:?}");
        assert_eq!(encode_meta(&decode_meta(&bytes).unwrap()), bytes);
    }
}

#[test]
fn encode_parts_equals_encode() {
    let f = fields("x/y", 4, 8);
    assert_eq!(encode_meta(&f), encode_meta_parts("x/y", 4, 8));
}

#[test]
fn every_proper_prefix_is_truncated() {
    let bytes = encode_meta_parts("hello", 1, 2);
    for len in 1..bytes.len() {
        match decode_meta(&bytes[..len]) {
            Err(MetaError::Truncated { needed, have }) => {
                assert_eq!(have, len);
                assert!(needed > have as u64);
            }
            other => panic!("len {len}: {other:?}"),
        }
    }
}

#[test]
fn truncated_reports_needed_length() {
    let bytes = encode_meta_parts("hello", 1, 2);
    assert_eq!(
        decode_meta(&bytes[..bytes.len() - 1]),
        Err(MetaError::Truncated {
            needed: bytes.len() as u64,
            have: bytes.len() - 1
        })
    );
}

#[test]
fn trailing_bytes_are_rejected() {
    let mut bytes = encode_meta_parts("hello", 1, 2);
    bytes.extend_from_slice(&[0, 0, 0]);
    assert_eq!(decode_meta(&bytes), Err(MetaError::Trailing { extra: 3 }));
}

#[test]
fn invalid_utf8_is_rejected() {
    assert_eq!(
        decode_meta(&raw(&[0xff, 0xfe], 2, 1, 2)),
        Err(MetaError::Utf8)
    );
}

#[test]
fn huge_declared_length_does_not_overflow() {
    for declared in [u64::MAX, u64::MAX - 23, u64::MAX - 24, 1 << 63, 1 << 40] {
        let bytes = raw(b"ab", declared, 1, 2);
        match decode_meta(&bytes) {
            Err(MetaError::Truncated { needed, have }) => {
                assert_eq!(have, bytes.len());
                assert_eq!(needed, 24u64.saturating_add(declared));
            }
            other => panic!("{declared}: {other:?}"),
        }
    }
}

#[test]
fn zero_form_decodes_to_default() {
    let zero = raw(b"", 0, 0, 0);
    assert_eq!(zero.len(), 24);
    assert_eq!(decode_meta(&zero), Ok(MetaFields::default()));
    assert!(encode_meta(&MetaFields::default()).is_empty());
}

#[test]
fn short_inputs_never_panic() {
    for len in 1..8 {
        let bytes = vec![0xffu8; len];
        assert!(matches!(
            decode_meta(&bytes),
            Err(MetaError::Truncated { .. })
        ));
    }
    for seed in 0u8..=255 {
        let bytes: Vec<u8> = (0..40u8).map(|i| i.wrapping_mul(seed).wrapping_add(seed)).collect();
        for len in 0..bytes.len() {
            let _ = decode_meta(&bytes[..len]);
        }
    }
}

#[test]
fn error_messages_are_stable() {
    assert_eq!(
        MetaError::Truncated { needed: 30, have: 4 }.to_string(),
        "meta truncated: need 30 bytes, have 4"
    );
    assert_eq!(
        MetaError::Trailing { extra: 2 }.to_string(),
        "meta has 2 trailing bytes"
    );
    assert_eq!(
        MetaError::Utf8.to_string(),
        "meta target_file is not valid UTF-8"
    );
}
