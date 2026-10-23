//! Trigram signature extraction (spec part A, section 6).
//!
//! Turns normalised text into a set of boundary-marked trigram strings:
//! tokenise on spaces, wrap each token in `_` markers, slide a length-3
//! window (a token of length L yields exactly L trigrams), deduplicate, and
//! for file text drop the most frequent trigrams (section 6, step 4).
use std::collections::HashMap;
/// Fraction of distinct trigrams considered "most frequent" and dropped.
pub const DEFAULT_PRUNE_RATIO: f64 = 0.10;
/// Only trigrams whose count exceeds this are eligible for pruning.
pub const DEFAULT_PRUNE_MIN_COUNT: usize = 10;
/// Boundary-marked trigrams of a single token (`user -> _us use ser er_`).
fn token_trigrams(token: &str, out: &mut Vec<String>) {
    let mut marked: Vec<char> = Vec::with_capacity(token.len() + 2);
    marked.push('_');
    marked.extend(token.chars());
    marked.push('_');
    for w in marked.windows(3) {
        out.push(w.iter().collect());
    }
}
fn tokens(norm: &str) -> impl Iterator<Item = &str> {
    norm.split([' ', '\n']).filter(|t| !t.is_empty())
}
/// Distinct trigrams of a query. No frequency pruning: a query has no corpus
/// to count against.
pub fn query_trigrams(norm: &str) -> Vec<String> {
    let mut buf = Vec::new();
    for tok in tokens(norm) {
        token_trigrams(tok, &mut buf);
    }
    buf.sort();
    buf.dedup();
    buf
}
/// Distinct trigrams of a file, with the most frequent ones pruned.
pub fn file_trigrams(
    norm: &str,
    prune_ratio: f64,
    prune_min_count: usize,
) -> Vec<String> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut buf = Vec::new();
    for tok in tokens(norm) {
        buf.clear();
        token_trigrams(tok, &mut buf);
        for tg in buf.drain(..) {
            *counts.entry(tg).or_insert(0) += 1;
        }
    }
    if counts.is_empty() {
        return Vec::new();
    }
    let mut ranked: Vec<(String, usize)> = counts.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let remove_n = (ranked.len() as f64 * prune_ratio).floor() as usize;
    let mut kept: Vec<String> = Vec::with_capacity(ranked.len());
    for (i, (tg, count)) in ranked.into_iter().enumerate() {
        if i < remove_n && count > prune_min_count {
            continue;
        }
        kept.push(tg);
    }
    kept.sort();
    kept
}
#[cfg(test)]
#[path = "test/signature.rs"]
mod tests;
