//! Tracing conventions and init.
//!
//! Span topology: one span per task, one span per leased resource (child of the task span),
//! one span per manager cycle (scan/communication slot). Spans carry only the stable field
//! names below. Across `.await` points use `#[tracing::instrument(skip_all, fields(...))]` or
//! `.instrument(span)`; never hold `span.enter()` across an `.await`. Spans don't propagate
//! into `spawn_blocking`/plain threads automatically: capture `Span::current()`, then
//! `span.in_scope(...)` inside the closure (see `for_blocking`). GPU completion callbacks must
//! not log directly (no tracing/CUDA calls in `cuLaunchHostFunc`); hand data to a channel or
//! atomic and log from the worker that drains it.
//!
//! The engine library never installs a global subscriber or appender; only the binary/daemon
//! does (see `init_tracing` and the CLI's non-blocking setup).
use tracing::Span;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;
/// Stable span/event field names (verbatim everywhere so filters and JSON pipelines stay
/// stable; never invent ad-hoc field names for these concepts).
pub const FIELD_TASK_ID: &str = "task_id";
pub const FIELD_RESOURCE_KIND: &str = "resource_kind";
pub const FIELD_MODEL_ID: &str = "model_id";
pub const FIELD_INDEX_ID: &str = "index_id";
pub const FIELD_SEGMENT_GEN: &str = "segment_gen";
pub const FIELD_SLOT: &str = "slot";
pub const FIELD_LANE: &str = "lane";
pub const FIELD_MANAGER_CYCLE: &str = "manager_cycle";
pub const FIELD_OP: &str = "op";
pub const FIELD_PRECISION: &str = "precision";
pub const FIELD_PROFILE: &str = "profile";
/// Initializes the global subscriber from `RUST_LOG`, defaulting to `info`.
///
/// Safe to call multiple times (e.g. in tests); subsequent calls are no-ops. For production
/// setups with a non-blocking appender or JSON output, build the subscriber directly in the
/// binary instead (see the CLI's `main`); this function is the plain default for library
/// callers and tests.
pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
}
/// Builds a non-blocking, optionally JSON-formatted subscriber from `RUST_LOG` (default
/// `info`). Returns the [`WorkerGuard`]; the caller (binary only) must keep it alive for the
/// process lifetime, or buffered log lines are lost on exit.
pub fn init_tracing_non_blocking<W>(writer: W, json: bool) -> WorkerGuard
where
    W: std::io::Write + Send + 'static,
{
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let (non_blocking, guard) = tracing_appender::non_blocking(writer);
    let builder = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(non_blocking);
    if json {
        #[cfg(feature = "json")]
        let _ = builder.json().try_init();
        #[cfg(not(feature = "json"))]
        let _ = builder.try_init();
    } else {
        let _ = builder.try_init();
    }
    guard
}
/// Creates the task span: one per task, carrying [`FIELD_TASK_ID`]. Parent of resource and
/// op spans for that task.
pub fn task_span(task_id: &str) -> Span {
    tracing::info_span!("task", task_id = % task_id)
}
/// Creates the resource span: one per leased resource (device/index), nested under the
/// current task span. Carries [`FIELD_TASK_ID`] and [`FIELD_RESOURCE_KIND`].
pub fn resource_span(task_id: &str, resource_kind: &str) -> Span {
    tracing::info_span!("resource", task_id = % task_id, resource_kind = % resource_kind)
}
/// Creates the manager-cycle span: one per communication slot / scan cycle, carrying
/// [`FIELD_MANAGER_CYCLE`].
pub fn manager_cycle_span(manager_cycle: u64) -> Span {
    tracing::info_span!("manager_cycle", manager_cycle = manager_cycle)
}
/// Clones `span` for use inside a `spawn_blocking`/plain-thread closure, per the propagation
/// rule above.
pub fn for_blocking(span: &Span) -> Span {
    span.clone()
}
#[cfg(test)]
mod tests {
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
}
