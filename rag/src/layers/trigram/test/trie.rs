use super::*;
use std::collections::HashSet;
#[test]
fn test_vector_bytes_and_mask() {
    let params = TrieParams::new(6, 0, 2).unwrap();
    let trie = CompactTrie::build(params, [13u32, 14, 48]).unwrap();
    let (payload, bits) = trie.to_payload();
    assert_eq!(bits, 20);
    assert_eq!(payload, vec![0x89, 0x16, 0x01]);
    let mask = trie.expand();
    assert_eq!(mask.words(), & [0x0001_0000_0000_6000u64]);
}
#[test]
fn container_roundtrip() {
    let params = TrieParams::new(6, 0, 2).unwrap();
    let trie = CompactTrie::build(params, [13u32, 14, 48]).unwrap();
    let bytes = trie.to_bytes();
    let back = CompactTrie::from_bytes(&bytes).unwrap();
    assert_eq!(trie, back);
}
#[test]
fn order_and_dup_invariance() {
    let params = TrieParams::new(6, 0, 2).unwrap();
    let a = CompactTrie::build(params, [48u32, 14, 13, 13, 48]).unwrap();
    let b = CompactTrie::build(params, [13u32, 14, 48]).unwrap();
    assert_eq!(a.to_bytes(), b.to_bytes());
}
#[test]
fn edge_cases() {
    for (kb, q, r) in [
        (6u8, 0u8, 2u8),
        (6, 0, 6),
        (16, 0, 4),
        (16, 4, 4),
        (10, 0, 2),
    ] {
        let params = TrieParams::new(kb, q, r).unwrap();
        let maxk = (1u32 << kb) - 1;
        for keys in [vec![], vec![0u32], vec![maxk], vec![0, maxk]] {
            let trie = CompactTrie::build(params, keys.clone()).unwrap();
            let expanded = trie.expand();
            let direct = CellMask::from_keys(params, keys.clone()).unwrap();
            assert_eq!(expanded, direct, "kb={kb} q={q} r={r} keys={keys:?}");
            for k in &keys {
                assert!(expanded.contains_key(* k));
            }
        }
    }
}
#[test]
fn full_set() {
    let params = TrieParams::new(6, 0, 2).unwrap();
    let all: Vec<u32> = (0..64).collect();
    let trie = CompactTrie::build(params, all.clone()).unwrap();
    let mask = trie.expand();
    assert_eq!(mask.count_ones(), 64);
    assert_eq!(mask.words(), & [u64::MAX]);
}
#[test]
fn key_out_of_range() {
    let params = TrieParams::new(6, 0, 2).unwrap();
    assert_eq!(CompactTrie::build(params, [64u32]), Err(Error::KeyOutOfRange));
}
#[test]
fn invalid_params() {
    assert!(TrieParams::new(6, 0, 3).is_err());
    assert!(TrieParams::new(6, 0, 0).is_err());
    assert!(TrieParams::new(6, 0, 7).is_err());
    assert!(TrieParams::new(26, 0, 2).is_err());
}
#[test]
fn corrupt_inputs_never_panic() {
    let params = TrieParams::new(16, 0, 4).unwrap();
    let trie = CompactTrie::build(params, [1u32, 2, 1000, 50000, 65535]).unwrap();
    let good = trie.to_bytes();
    for n in 0..good.len() {
        let _ = CompactTrie::from_bytes(&good[..n]);
    }
    for byte in 0..good.len() {
        for bit in 0..8 {
            let mut c = good.clone();
            c[byte] ^= 1 << bit;
            let _ = CompactTrie::from_bytes(&c);
        }
    }
}
#[test]
fn bad_magic_and_version() {
    let params = TrieParams::new(6, 0, 2).unwrap();
    let mut bytes = CompactTrie::build(params, [1u32]).unwrap().to_bytes();
    let mut bad = bytes.clone();
    bad[0] = b'X';
    assert_eq!(CompactTrie::from_bytes(& bad), Err(Error::BadMagic));
    bytes[4] = 2;
    assert_eq!(CompactTrie::from_bytes(& bytes), Err(Error::BadVersion));
}
#[test]
fn query_matching_basic() {
    let params = TrieParams::new(16, 0, 4).unwrap();
    let file = CellMask::from_keys(params, [1u32, 2, 3, 100, 200]).unwrap();
    let q = QueryMask::new(params, [2u32, 3, 999]).unwrap();
    assert_eq!(matched_cells(& file, & q), 2);
    assert!(matches_at_least(& file, & q, 2));
    assert!(! matches_at_least(& file, & q, 3));
}
#[test]
fn corrected_threshold_for_collisions() {
    let params = TrieParams::new(6, 2, 2).unwrap();
    let q = QueryMask::new(params, [0u32, 1, 2, 3]).unwrap();
    assert_eq!(q.n_keys, 4);
    assert_eq!(q.n_cells, 1);
    assert_eq!(q.corrected_threshold(4), 1);
}
use proptest::prelude::*;
proptest! {
    #[test] fn prop_expand_equals_from_keys(keys in prop::collection::vec(0u32
    ..65536, 0..200)) { let params = TrieParams::new(16, 0, 4).unwrap(); let trie =
    CompactTrie::build(params, keys.clone()).unwrap(); prop_assert_eq!(trie.expand(),
    CellMask::from_keys(params, keys).unwrap()); } #[test] fn
    prop_contains_all_keys(keys in prop::collection::vec(0u32..65536, 0..200)) { let
    params = TrieParams::new(16, 0, 4).unwrap(); let mask =
    CompactTrie::build(params, keys.clone()).unwrap().expand(); for k in keys {
    prop_assert!(mask.contains_key(k)); } } #[test] fn
    prop_matched_equals_naive_q0(fkeys in prop::collection::vec(0u32..4096, 0..200),
    qkeys in prop::collection::vec(0u32..4096, 0..60),) { let params =
    TrieParams::new(12, 0, 4).unwrap(); let file = CellMask::from_keys(params, fkeys
    .clone()).unwrap(); let q = QueryMask::new(params, qkeys.clone()).unwrap(); let
    fset : HashSet < u32 > = fkeys.into_iter().collect(); let qset : HashSet < u32 >
    = qkeys.into_iter().collect(); let naive = qset.iter().filter(| k | fset
    .contains(k)).count() as u32; prop_assert_eq!(matched_cells(& file, & q), naive);
    } #[test] fn prop_matched_upper_bounds_real_qgt0(fkeys in
    prop::collection::vec(0u32..65536, 0..200), qkeys in prop::collection::vec(0u32
    ..65536, 0..60),) { let params = TrieParams::new(16, 4, 4).unwrap(); let file =
    CellMask::from_keys(params, fkeys.clone()).unwrap(); let q =
    QueryMask::new(params, qkeys.clone()).unwrap(); let fset : HashSet < u32 > =
    fkeys.into_iter().collect(); let qset : HashSet < u32 > = qkeys.into_iter()
    .collect(); let real = qset.iter().filter(| k | fset.contains(k)).count() as u32;
    prop_assert!(matched_cells(& file, & q) >= real); } #[test] fn
    prop_bytes_order_invariant(mut keys in prop::collection::vec(0u32..65536, 0
    ..200)) { let params = TrieParams::new(16, 0, 4).unwrap(); let a =
    CompactTrie::build(params, keys.clone()).unwrap().to_bytes(); keys.reverse();
    keys.extend_from_within(..); let b = CompactTrie::build(params, keys).unwrap()
    .to_bytes(); prop_assert_eq!(a, b); } #[test] fn
    prop_matches_at_least_agrees(fkeys in prop::collection::vec(0u32..65536, 0..200),
    qkeys in prop::collection::vec(0u32..65536, 0..60), t in 0u32..40,) { let params
    = TrieParams::new(16, 0, 4).unwrap(); let file = CellMask::from_keys(params,
    fkeys).unwrap(); let q = QueryMask::new(params, qkeys).unwrap();
    prop_assert_eq!(matches_at_least(& file, & q, t), matched_cells(& file, & q) >=
    t); }
}
