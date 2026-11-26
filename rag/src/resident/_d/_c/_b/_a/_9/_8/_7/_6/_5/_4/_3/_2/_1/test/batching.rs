use super::*;
fn cfg(budget: usize, max: usize) -> BatchConfig {
    BatchConfig {
        token_budget: budget,
        max_seq_len: max,
        pad_token_id: 7,
    }
}
#[test]
fn padded_len_examples() {
    assert_eq!(padded_len(0, 100), 32);
    assert_eq!(padded_len(40, 50), 50);
    assert_eq!(padded_len(600, 1000), 600);
    assert_eq!(padded_len(5, 16), 16);
    assert_eq!(padded_len(33, 512), 64);
}
#[test]
fn invalid_config_and_empty() {
    assert_eq!(build_batches(& [], & cfg(0, 8)), Err(BatchError::InvalidConfig));
    assert_eq!(build_batches(& [], & cfg(8, 0)), Err(BatchError::InvalidConfig));
    assert_eq!(build_batches(& [], & cfg(8, 8)), Ok(vec![]));
}
#[test]
fn groups_sorted_with_padding() {
    let seqs = vec![vec![1, 2, 3], vec![], vec![4]];
    let b = build_batches(&seqs, &cfg(1000, 100)).unwrap();
    assert_eq!(b.len(), 1);
    let b = &b[0];
    assert_eq!(b.rows(), 3);
    assert_eq!(b.seq_len(), 32);
    assert_eq!(b.positions(), & [1, 2, 0]);
    assert_eq!(& b.input_ids() [..32], & [7u32; 32] [..]);
    assert_eq!(b.attention_mask() [..32].iter().sum::< u32 > (), 0);
    assert_eq!(b.attention_mask() [64..].iter().sum::< u32 > (), 3);
    assert_eq!(b.input_ids() [64..67], [1, 2, 3]);
}
#[test]
fn splits_by_budget_and_oversize_single() {
    let seqs = vec![vec![1; 10], vec![1; 10], vec![1; 100]];
    let b = build_batches(&seqs, &cfg(64, 200)).unwrap();
    assert_eq!(b.len(), 2);
    assert_eq!(b[0].rows(), 2);
    assert_eq!(b[1].rows(), 1);
    assert_eq!(b[1].seq_len(), 128);
}
#[test]
fn truncates() {
    let seqs = vec![vec![1, 2, 3, 4, 5]];
    let b = build_batches(&seqs, &cfg(100, 3)).unwrap();
    assert_eq!(b[0].seq_len(), 3);
    assert_eq!(b[0].input_ids(), & [1, 2, 3]);
    assert_eq!(b[0].attention_mask(), & [1, 1, 1]);
}
#[test]
fn serialization() {
    let seqs = vec![vec![1, 2]];
    let b = build_batches(&seqs, &cfg(100, 2)).unwrap();
    let bytes = b[0].to_bytes();
    assert_eq!(bytes.len(), 2 * 2 * 4);
    assert_eq!(& bytes[..4], & 1u32.to_le_bytes());
    assert_eq!(& bytes[8..12], & 1u32.to_le_bytes());
    assert_eq!(b[0].positions_to_bytes(), 0u32.to_le_bytes().to_vec());
}
#[test]
fn batch_size() {
    let none = ClassDeadline {
        max_batch_duration: None,
        cost_per_token: Duration::from_micros(1),
    };
    assert_eq!(batch_size_for(1024, 128, & none), 8);
    assert_eq!(batch_size_for(10, 128, & none), 1);
    assert_eq!(batch_size_for(10, 0, & none), 1);
    let d = ClassDeadline {
        max_batch_duration: Some(Duration::from_micros(300)),
        cost_per_token: Duration::from_micros(1),
    };
    assert_eq!(batch_size_for(1024, 32, & d), 9);
    assert_eq!(batch_size_for(1024, 512, & d), 1);
}
