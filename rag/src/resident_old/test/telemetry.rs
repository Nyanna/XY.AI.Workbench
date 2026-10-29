use super::*;
use tracing::field::{Field, Visit};
use tracing::span::{Attributes, Id};
use tracing::subscriber::{Interest, Subscriber};
use tracing::Metadata;
/// Minimal subscriber capturing span field names/values for assertions, since we only
/// need to verify the stable-field-name contract, not full event formatting.
#[derive(Clone)]
struct Capture {
    fields: std::sync::Arc<std::sync::Mutex<Vec<(String, String)>>>,
}
struct FieldDump<'a>(&'a mut Vec<(String, String)>);
impl Visit for FieldDump<'_> {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.0.push((field.name().to_string(), format!("{value:?}")));
    }
}
impl Subscriber for Capture {
    fn enabled(&self, _: &Metadata<'_>) -> bool {
        true
    }
    fn register_callsite(&self, _: &'static Metadata<'static>) -> Interest {
        Interest::always()
    }
    fn new_span(&self, attrs: &Attributes<'_>) -> Id {
        let mut dump = FieldDump(&mut self.fields.lock().unwrap());
        attrs.record(&mut dump);
        Id::from_u64(1)
    }
    fn record(&self, _: &Id, _: &tracing::span::Record<'_>) {}
    fn record_follows_from(&self, _: &Id, _: &Id) {}
    fn event(&self, _: &tracing::Event<'_>) {}
    fn enter(&self, _: &Id) {}
    fn exit(&self, _: &Id) {}
}
#[test]
fn task_span_carries_stable_task_id_field() {
    let capture = Capture {
        fields: Default::default(),
    };
    let fields = capture.fields.clone();
    tracing::subscriber::with_default(
        capture,
        || {
            let _span = task_span("t-1");
        },
    );
    let recorded = fields.lock().unwrap();
    assert!(
        recorded.iter().any(| (name, value) | name == FIELD_TASK_ID && value
        .contains("t-1"))
    );
}
#[test]
fn for_blocking_preserves_span_identity_fields() {
    let capture = Capture {
        fields: Default::default(),
    };
    tracing::subscriber::with_default(
        capture,
        || {
            let span = task_span("t-2");
            let cloned = for_blocking(&span);
            assert_eq!(span.id(), cloned.id());
            assert!(span.id().is_some());
        },
    );
}
#[test]
fn stable_field_names_match_convention() {
    assert_eq!(FIELD_TASK_ID, "task_id");
    assert_eq!(FIELD_RESOURCE_KIND, "resource_kind");
    assert_eq!(FIELD_MODEL_ID, "model_id");
    assert_eq!(FIELD_INDEX_ID, "index_id");
    assert_eq!(FIELD_SEGMENT_GEN, "segment_gen");
    assert_eq!(FIELD_SLOT, "slot");
    assert_eq!(FIELD_LANE, "lane");
    assert_eq!(FIELD_MANAGER_CYCLE, "manager_cycle");
    assert_eq!(FIELD_OP, "op");
    assert_eq!(FIELD_PRECISION, "precision");
    assert_eq!(FIELD_PROFILE, "profile");
}
