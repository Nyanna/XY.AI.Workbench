use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

struct MockBackend {
    calls: AtomicUsize,
    fail: bool,
    wrong_count: bool,
}

impl MockBackend {
    fn new() -> Self {
        Self { calls: AtomicUsize::new(0), fail: false, wrong_count: false }
    }
}

impl EmbeddingBackend for MockBackend {
    fn embed(
        &self,
        _model: &ModelId,
        _mode: EmbeddingMode,
        _precision: Precision,
        texts: &[String],
        _extensions: &Extensions,
    ) -> Result<Vec<Vec<f32>>, BackendError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.fail {
            return Err(BackendError("boom".into()));
        }
        let mut out: Vec<Vec<f32>> = texts.iter().map(|t| vec![t.len() as f32, 1.0]).collect();
        if self.wrong_count {
            out.pop();
        }
        Ok(out)
    }
}

#[derive(Default)]
struct MockTarget {
    calls: AtomicUsize,
    fail: bool,
    short: bool,
    seen: Mutex<Vec<EntryMetadata>>,
}

impl EmbeddingTarget for MockTarget {
    fn write(
        &self,
        vectors: Vec<Vec<f32>>,
        metadata: Vec<EntryMetadata>,
        _extensions: &Extensions,
    ) -> Result<Vec<EntryId>, TargetError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        if self.fail {
            return Err(TargetError("nope".into()));
        }
        assert_eq!(vectors.len(), metadata.len());
        *self.seen.lock().unwrap() = metadata;
        let mut ids: Vec<EntryId> = (0..vectors.len() as u64).map(|i| EntryId(100 + i)).collect();
        if self.short {
            ids.pop();
        }
        Ok(ids)
    }
}

fn task(input: impl Into<EmbeddingInput>) -> EmbeddingTask {
    EmbeddingTask::new(input, ModelId("m".into()), EmbeddingMode::Passage, Precision::F32)
}

#[test]
fn single_text_yields_list_of_one() {
    let b = MockBackend::new();
    let r = task("abc").run(&b).unwrap();
    assert_eq!(r.as_embeddings().unwrap(), &[vec![3.0, 1.0]]);
}

#[test]
fn many_preserves_order_and_duplicates() {
    let b = MockBackend::new();
    let t = task(vec!["a".to_string(), "bbb".to_string(), "a".to_string()]);
    let r = t.run(&b).unwrap();
    let v = r.as_embeddings().unwrap();
    assert_eq!(v.len(), 3);
    assert_eq!(v[0], v[2]);
    assert_eq!(v[1][0], 3.0);
    assert_eq!(b.calls.load(Ordering::SeqCst), 1);
}

#[test]
fn empty_input_skips_backend_and_target() {
    let b = MockBackend::new();
    let r = task(Vec::<String>::new()).run(&b).unwrap();
    assert_eq!(r.as_embeddings().unwrap().len(), 0);
    let tg = Arc::new(MockTarget::default());
    let r = task(Vec::<String>::new()).with_target(tg.clone()).run(&b).unwrap();
    assert_eq!(r.as_entry_refs().unwrap().len(), 0);
    assert_eq!(b.calls.load(Ordering::SeqCst), 0);
    assert_eq!(tg.calls.load(Ordering::SeqCst), 0);
}

#[test]
fn empty_string_is_passed_to_backend() {
    let b = MockBackend::new();
    let r = task("").run(&b).unwrap();
    assert_eq!(r.as_embeddings().unwrap(), &[vec![0.0, 1.0]]);
}

#[test]
fn target_returns_entry_refs_with_metadata() {
    let b = MockBackend::new();
    let tg = Arc::new(MockTarget::default());
    let t = task(vec!["x".to_string(), "y".to_string()]).with_target(tg.clone());
    let r = t.run(&b).unwrap();
    assert_eq!(r.as_entry_refs().unwrap(), &[EntryId(100), EntryId(101)]);
    let seen = tg.seen.lock().unwrap();
    assert_eq!(seen[1].text, "y");
    assert_eq!(seen[0].model, ModelId("m".into()));
    assert_eq!(seen[0].mode, EmbeddingMode::Passage);
    assert_eq!(seen[0].precision, Precision::F32);
    assert_eq!(tg.calls.load(Ordering::SeqCst), 1);
}

#[test]
fn backend_error_skips_target() {
    let mut b = MockBackend::new();
    b.fail = true;
    let tg = Arc::new(MockTarget::default());
    let e = task("a").with_target(tg.clone()).run(&b).unwrap_err();
    assert_eq!(e, EmbeddingError::Backend(BackendError("boom".into())));
    assert_eq!(tg.calls.load(Ordering::SeqCst), 0);
}

#[test]
fn backend_count_mismatch() {
    let mut b = MockBackend::new();
    b.wrong_count = true;
    let e = task(vec!["a".to_string(), "b".to_string()]).run(&b).unwrap_err();
    assert_eq!(e, EmbeddingError::CountMismatch { expected: 2, got: 1 });
}

#[test]
fn target_error_and_count_mismatch() {
    let b = MockBackend::new();
    let tg = Arc::new(MockTarget { fail: true, ..Default::default() });
    let e = task("a").with_target(tg).run(&b).unwrap_err();
    assert_eq!(e, EmbeddingError::Target(TargetError("nope".into())));

    let tg = Arc::new(MockTarget { short: true, ..Default::default() });
    let e = task(vec!["a".to_string(), "b".to_string()]).with_target(tg).run(&b).unwrap_err();
    assert_eq!(e, EmbeddingError::CountMismatch { expected: 2, got: 1 });
}

#[test]
fn debug_shows_target_presence_only() {
    let tg = Arc::new(MockTarget::default());
    let s = format!("{:?}", task("a").with_target(tg));
    assert!(s.contains("target: true"));
    assert!(format!("{:?}", task("a")).contains("target: false"));
}

#[test]
fn input_accessors() {
    let i: EmbeddingInput = "q".into();
    assert_eq!(i.texts(), &["q".to_string()]);
    assert_eq!(i.len(), 1);
    assert!(!i.is_empty());
    assert!(EmbeddingInput::Many(vec![]).is_empty());
}

#[test]
fn concurrent_runs_are_independent() {
    let b = MockBackend::new();
    let t = task("abcd");
    std::thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                let r = t.run(&b).unwrap();
                assert_eq!(r.as_embeddings().unwrap()[0][0], 4.0);
            });
        }
    });
    assert_eq!(b.calls.load(Ordering::SeqCst), 4);
}
