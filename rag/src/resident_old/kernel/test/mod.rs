use super::mock::{Call, MockKernel};
use super::*;
fn spans(n: u64) -> Vec<RegionSpan> {
    (0..n)
        .map(|i| RegionSpan {
            offset: i * 64,
            len: 64,
        })
        .collect()
}
fn desc() -> BatchDesc {
    BatchDesc {
        queries: 2,
        ..Default::default()
    }
}
#[test]
fn entry_of_fixed_bounds() {
    assert_eq!(entry_of_fixed(100, 10, 4, 99), None);
    assert_eq!(entry_of_fixed(100, 10, 4, 100), Some(0));
    assert_eq!(entry_of_fixed(100, 10, 4, 119), Some(1));
    assert_eq!(entry_of_fixed(100, 10, 4, 139), Some(3));
    assert_eq!(entry_of_fixed(100, 10, 4, 140), None);
    assert_eq!(entry_of_fixed(100, 0, 4, 100), None);
}
#[test]
fn entry_of_variable_bounds() {
    let offsets = [10, 15, 15, 40];
    assert_eq!(entry_of(& offsets, 9), None);
    assert_eq!(entry_of(& offsets, 10), Some(0));
    assert_eq!(entry_of(& offsets, 14), Some(0));
    assert_eq!(entry_of(& offsets, 15), Some(2));
    assert_eq!(entry_of(& offsets, 39), Some(2));
    assert_eq!(entry_of(& offsets, 40), None);
    assert_eq!(entry_of(& [], 0), None);
}
#[test]
fn scoring_is_deterministic() {
    let run = || {
        let k = MockKernel::scoring();
        let b = k.begin(&desc()).unwrap();
        let out = k.run(b, &spans(5)).unwrap();
        (out, k.end(b).unwrap())
    };
    let (a, b) = (run(), run());
    assert_eq!(a, b);
    assert!(matches!(& a.0, KernelOutput::Hits(h) if h.len() == 5));
    assert!(a.1.is_empty());
}
#[test]
fn partial_then_follow_up_range_covers_input() {
    let k = MockKernel::embedding(120);
    let b = k.begin(&desc()).unwrap();
    let input = [
        RegionSpan { offset: 0, len: 100 },
        RegionSpan { offset: 100, len: 50 },
    ];
    assert_eq!(k.run(b, & input).unwrap(), KernelOutput::Partial);
    k.grant(100);
    let rest = [RegionSpan { offset: 120, len: 30 }];
    assert!(matches!(k.run(b, & rest).unwrap(), KernelOutput::Hits(_)));
    let written = k.end(b).unwrap();
    assert_eq!(written.iter().map(| w | w.len).sum::< u64 > (), 150);
}
#[test]
fn progress_is_monotonic() {
    let k = MockKernel::scoring();
    let b = k.begin(&desc()).unwrap();
    let mut last = k.progress(b);
    for _ in 0..4 {
        k.run(b, &spans(3)).unwrap();
        let now = k.progress(b);
        assert!(now > last);
        last = now;
    }
}
#[test]
fn injected_failure_leaves_batch_endable() {
    let k = MockKernel::scoring().fail_after(1);
    let b = k.begin(&desc()).unwrap();
    assert!(k.run(b, & spans(1)).is_ok());
    assert!(matches!(k.run(b, & spans(1)), Err(IndexError::KernelFailed { .. })));
    assert!(k.end(b).is_ok());
    assert_eq!(k.calls().last(), Some(& Call::End(b)));
    assert!(matches!(k.end(b), Err(IndexError::UnknownBatch { .. })));
}
#[test]
fn usable_as_trait_object() {
    let k: std::sync::Arc<dyn Kernel> = std::sync::Arc::new(MockKernel::scoring());
    let b = k.begin(&desc()).unwrap();
    assert!(k.run(b, & spans(1)).is_ok());
}
