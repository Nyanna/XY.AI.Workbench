//! Tracing conventions and init.
//!
//! One span per task; child spans per processing cycle/batch and per manager cycle. A span's
//! resource (device/index) is a field, never a span of its own. Across `.await` points use
//! `#[tracing::instrument]`/`.instrument(span)`, never `span.enter()` held across `.await`.
//! Spans don't propagate into `spawn_blocking`/plain threads automatically: clone the span and
//! use `span.in_scope(...)` (or `let _g = span.enter();`) inside the closure.
use tracing::Span;
use tracing_subscriber::EnvFilter;
/// Span field name for the task identifier (one span per task).
pub const FIELD_TASK_ID: &str = "task_id";
/// Span field name for the resource (device/index) a span operates on.
pub const FIELD_RESOURCE: &str = "resource";
/// Initializes the global subscriber from `RUST_LOG`, defaulting to `info`.
///
/// Safe to call multiple times (e.g. in tests); subsequent calls are no-ops.
pub fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
}
/// Clones `span` for use inside a `spawn_blocking`/plain-thread closure, per the propagation
/// rule above.
pub fn for_blocking(span: &Span) -> Span {
    span.clone()
}
