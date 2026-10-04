//! Benchmarks for the trigram trie (Part B, section 8 "Benchmarks").
//!
//! * Expansion: `b = 16`, n in {100, 500, 5000}.
//! * Matching: 100 000 masks of 8 KB (b = 16) and 128 B (b = 10) against
//!   queries of 20 and 100 keys; reported per file for `matched_cells` and
//!   `matches_at_least`.

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use xy_ai_rag::layers::trigram::trie::{
    matched_cells, matches_at_least, CellMask, CompactTrie, QueryMask, TrieParams,
};

/// Cheap deterministic PRNG (xorshift64) to avoid extra dependencies.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn key(&mut self, bits: u32) -> u32 {
        (self.next() & ((1u64 << bits) - 1)) as u32
    }
}

fn keys(rng: &mut Rng, n: usize, bits: u32) -> Vec<u32> {
    (0..n).map(|_| rng.key(bits)).collect()
}

fn bench_expand(c: &mut Criterion) {
    let params = TrieParams::new(16, 0, 4).unwrap();
    let mut group = c.benchmark_group("expand_b16");
    for &n in &[100usize, 500, 5000] {
        let mut rng = Rng(0x1234_5678_9abc_def0);
        let trie = CompactTrie::build(params, keys(&mut rng, n, 16)).unwrap();
        group.bench_function(format!("n{n}"), |bch| {
            bch.iter_batched(|| &trie, |t| black_box(t.expand()), BatchSize::SmallInput)
        });
    }
    group.finish();
}

fn bench_match(c: &mut Criterion) {
    const FILES: usize = 100_000;
    for &(bits, label) in &[(16u32, "b16_8k"), (10u32, "b10_128b")] {
        let params = TrieParams::new(bits as u8, 0, 4).unwrap();
        let mut rng = Rng(0xdead_beef_cafe_babe);
        let masks: Vec<CellMask> = (0..FILES)
            .map(|_| CellMask::from_keys(params, keys(&mut rng, 40, bits)).unwrap())
            .collect();
        for &qn in &[20usize, 100] {
            let q = QueryMask::new(params, keys(&mut rng, qn, bits)).unwrap();
            let t = q.corrected_threshold((qn as u32 * 8) / 10);
            let mut group = c.benchmark_group(format!("match_{label}_q{qn}"));
            group.throughput(criterion::Throughput::Elements(FILES as u64));
            group.bench_function("matched_cells", |bch| {
                bch.iter(|| {
                    let mut acc = 0u64;
                    for m in &masks {
                        acc += black_box(matched_cells(m, &q)) as u64;
                    }
                    acc
                })
            });
            group.bench_function("matches_at_least", |bch| {
                bch.iter(|| {
                    let mut acc = 0u64;
                    for m in &masks {
                        acc += black_box(matches_at_least(m, &q, t)) as u64;
                    }
                    acc
                })
            });
            group.finish();
        }
    }
}

criterion_group!(benches, bench_expand, bench_match);
criterion_main!(benches);
