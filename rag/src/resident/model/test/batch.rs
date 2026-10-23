use super::*;

fn cfg(budget: usize) -> BatchConfig {
    BatchConfig { token_budget: budget, max_seq_len: 512, pad_token_id: 7 }
}

#[test]
fn pads_to_bucket_and_masks() {
    let b = &plan_batches(&[vec![1, 2, 3]], &cfg(4096))[0];
    assert_eq!(b.seq_len, 32);
    assert_eq!(&b.ids[..4], &[1, 2, 3, 7]);
    assert_eq!(b.mask.iter().sum::<u32>(), 3);
    assert_eq!(b.pack().len(), 2 * 32 * 4);
}

#[test]
fn respects_token_budget_and_covers_all_rows() {
    let seqs: Vec<Vec<u32>> = (1..=40).map(|n| vec![1; n * 3]).collect();
    let batches = plan_batches(&seqs, &cfg(512));
    assert!(batches.iter().all(|b| b.rows() == 1 || b.rows() * b.seq_len <= 512));
    let mut seen: Vec<usize> = batches.iter().flat_map(|b| b.indices.clone()).collect();
    seen.sort_unstable();
    assert_eq!(seen, (0..40).collect::<Vec<_>>());
}

#[test]
fn oversized_row_is_truncated_and_isolated() {
    let mut c = cfg(64);
    c.max_seq_len = 128;
    let batches = plan_batches(&[vec![1; 300], vec![1; 5]], &c);
    assert_eq!(batches.len(), 2);
    assert_eq!(batches[1].seq_len, 128);
    assert_eq!(batches[1].mask.iter().sum::<u32>(), 128);
}

#[test]
fn rows_keep_original_indices() {
    let batches = plan_batches(&[vec![1; 100], vec![1; 2]], &cfg(100_000));
    assert_eq!(batches[0].indices, vec![1, 0]);
}
