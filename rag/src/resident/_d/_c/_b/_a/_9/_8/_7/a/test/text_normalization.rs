use super::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::resource_lifecycle::{
    InitRequest, LifecycleConfig, MemoryClass, ResourceRegistry,
};
struct Upper {
    steps: NormalizationParams,
    calls: AtomicUsize,
}
impl Upper {
    fn new(steps: NormalizationParams) -> Arc<Self> {
        Arc::new(Self {
            steps,
            calls: AtomicUsize::new(0),
        })
    }
}
impl TextNormalizer for Upper {
    fn normalize<'a>(&self, text: &'a str) -> Result<Cow<'a, str>, NormalizeError> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        Ok(Cow::Owned(text.to_uppercase()))
    }
    fn performed_steps(&self) -> NormalizationParams {
        self.steps
    }
}
struct Failing;
impl TextNormalizer for Failing {
    fn normalize<'a>(&self, _text: &'a str) -> Result<Cow<'a, str>, NormalizeError> {
        Err(NormalizeError("bad input".into()))
    }
}
fn all_steps() -> NormalizationParams {
    NormalizationParams {
        unicode_form: UnicodeForm::Nfkc,
        lowercase: true,
        strip_accents: true,
    }
}
fn same(resolved: ResolvedNormalization, params: NormalizationParams) -> bool {
    resolved.unicode_form == params.unicode_form
        && resolved.lowercase == params.lowercase
        && resolved.strip_accents == params.strip_accents
}
#[test]
fn apply_without_external_borrows_input() {
    let pre = TextPreprocessor::new(None);
    let out = pre.apply("Ärger Straße").unwrap();
    assert!(matches!(out, Cow::Borrowed(_)));
    assert_eq!(out, "Ärger Straße");
}
#[test]
fn apply_with_identity_borrows_input() {
    let pre = TextPreprocessor::new(Some(Arc::new(IdentityNormalizer)));
    let out = pre.apply("abc").unwrap();
    assert!(matches!(out, Cow::Borrowed(_)));
}
#[test]
fn apply_empty_text_passes_through() {
    assert_eq!(TextPreprocessor::new(None).apply("").unwrap(), "");
    let pre = TextPreprocessor::new(Some(Arc::new(IdentityNormalizer)));
    assert_eq!(pre.apply("").unwrap(), "");
}
#[test]
fn apply_returns_external_result_exactly_once() {
    let upper = Upper::new(NormalizationParams::default());
    let pre = TextPreprocessor::new(Some(upper.clone()));
    assert_eq!(pre.apply("abc").unwrap(), "ABC");
    assert_eq!(upper.calls.load(Ordering::SeqCst), 1);
}
#[test]
fn apply_never_performs_tokenizer_steps() {
    let pre = TextPreprocessor::new(None);
    let text = "Éa\u{301}ＡＢ";
    assert_eq!(pre.apply(text).unwrap(), text);
}
#[test]
fn apply_propagates_error_unchanged() {
    let pre = TextPreprocessor::new(Some(Arc::new(Failing)));
    assert_eq!(pre.apply("x"), Err(NormalizeError("bad input".into())));
    assert_eq!(pre.apply("x"), Err(NormalizeError("bad input".into())));
}
#[test]
fn apply_does_not_catch_panics() {
    struct Panicking;
    impl TextNormalizer for Panicking {
        fn normalize<'a>(&self, _text: &'a str) -> Result<Cow<'a, str>, NormalizeError> {
            panic!("boom");
        }
    }
    let pre = TextPreprocessor::new(Some(Arc::new(Panicking)));
    let result = std::panic::catch_unwind(
        std::panic::AssertUnwindSafe(|| {
            let _ = pre.apply("x");
        }),
    );
    assert!(result.is_err());
}
#[test]
fn resolve_without_external_equals_params() {
    let params = all_steps();
    assert!(same(TextPreprocessor::new(None).resolve(& params), params));
    let identity = TextPreprocessor::new(Some(Arc::new(IdentityNormalizer)));
    assert!(same(identity.resolve(& params), params));
    let none = NormalizationParams::default();
    assert!(same(identity.resolve(& none), none));
}
#[test]
fn resolve_removes_declared_unicode_form() {
    let steps = NormalizationParams {
        unicode_form: UnicodeForm::Nfc,
        ..Default::default()
    };
    let pre = TextPreprocessor::new(Some(Upper::new(steps)));
    let resolved = pre.resolve(&all_steps());
    assert_eq!(resolved.unicode_form, UnicodeForm::None);
    assert!(resolved.lowercase);
    assert!(resolved.strip_accents);
}
#[test]
fn resolve_removes_declared_lowercase() {
    let steps = NormalizationParams {
        lowercase: true,
        ..Default::default()
    };
    let pre = TextPreprocessor::new(Some(Upper::new(steps)));
    let resolved = pre.resolve(&all_steps());
    assert_eq!(resolved.unicode_form, UnicodeForm::Nfkc);
    assert!(! resolved.lowercase);
    assert!(resolved.strip_accents);
}
#[test]
fn resolve_removes_declared_strip_accents() {
    let steps = NormalizationParams {
        strip_accents: true,
        ..Default::default()
    };
    let pre = TextPreprocessor::new(Some(Upper::new(steps)));
    let resolved = pre.resolve(&all_steps());
    assert_eq!(resolved.unicode_form, UnicodeForm::Nfkc);
    assert!(resolved.lowercase);
    assert!(! resolved.strip_accents);
}
#[test]
fn resolve_removes_all_when_all_declared() {
    let pre = TextPreprocessor::new(Some(Upper::new(all_steps())));
    assert_eq!(pre.resolve(& all_steps()), ResolvedNormalization::default());
}
#[test]
fn resolve_does_not_enable_steps_missing_from_params() {
    let pre = TextPreprocessor::new(Some(Upper::new(all_steps())));
    let resolved = pre.resolve(&NormalizationParams::default());
    assert_eq!(resolved, ResolvedNormalization::default());
}
#[test]
fn resolve_is_deterministic_and_idempotent() {
    let steps = NormalizationParams {
        lowercase: true,
        ..Default::default()
    };
    let pre = TextPreprocessor::new(Some(Upper::new(steps)));
    let first = pre.resolve(&all_steps());
    assert_eq!(first, pre.resolve(& all_steps()));
    let again = pre
        .resolve(
            &NormalizationParams {
                unicode_form: first.unicode_form,
                lowercase: first.lowercase,
                strip_accents: first.strip_accents,
            },
        );
    assert_eq!(first, again);
}
#[test]
fn constructors_do_no_work() {
    let upper = Upper::new(all_steps());
    let pre = TextPreprocessor::new(Some(upper.clone()));
    let _loader = NormalizerLoader::new(all_steps(), pre);
    assert_eq!(upper.calls.load(Ordering::SeqCst), 0);
}
#[test]
fn loader_returns_resolved_and_is_repeatable() {
    let steps = NormalizationParams {
        lowercase: true,
        ..Default::default()
    };
    let pre = TextPreprocessor::new(Some(Upper::new(steps)));
    let expected = pre.resolve(&all_steps());
    let loader = NormalizerLoader::new(all_steps(), pre);
    assert_eq!(loader.load(), Ok(expected));
    assert_eq!(loader.load(), Ok(expected));
}
#[test]
fn loader_without_external_returns_params() {
    let loader = NormalizerLoader::new(all_steps(), TextPreprocessor::new(None));
    let resolved = loader.load().unwrap();
    assert!(same(resolved, all_steps()));
}
#[test]
fn loader_works_through_registry() {
    let config = LifecycleConfig::new(
            Duration::from_secs(10),
            Duration::from_secs(60),
            Duration::from_secs(5),
        )
        .unwrap();
    let registry: ResourceRegistry<ResolvedNormalization> = ResourceRegistry::new(
        config,
    );
    let loader = NormalizerLoader::new(all_steps(), TextPreprocessor::new(None));
    registry
        .register("normalizer".to_string(), MemoryClass::Ram, Arc::new(loader))
        .unwrap();
    assert!(! registry.is_loaded("normalizer"));
    let now = Instant::now();
    match registry.request_init("normalizer", 0, now) {
        InitRequest::Started(ticket) => ticket.run(now).unwrap(),
        _ => panic!("expected Started"),
    }
    match registry.request_init("normalizer", 1, now) {
        InitRequest::Ready(lease) => {
            assert!(same(* lease.handle().as_ref(), all_steps()));
            lease.release(now);
        }
        _ => panic!("expected Ready"),
    }
}
#[test]
fn clones_share_normalizer_and_work_concurrently() {
    let upper = Upper::new(NormalizationParams::default());
    let pre = TextPreprocessor::new(Some(upper.clone()));
    let handles: Vec<_> = (0..8)
        .map(|_| {
            let pre = pre.clone();
            thread::spawn(move || {
                for _ in 0..100 {
                    assert_eq!(pre.apply("ab").unwrap(), "AB");
                    assert!(same(pre.resolve(& all_steps()), all_steps()));
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    assert_eq!(upper.calls.load(Ordering::SeqCst), 800);
}
