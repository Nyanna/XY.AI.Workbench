use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};
use log::{debug, warn};
/// Raw task identifier used by `TaskContext`.
///
/// This is a plain `u64` alias, not a distinct type. Do not confuse it with `operation::TaskId`,
/// the typed struct stored in `Operation` and used in reports. Both carry the same numeric value.
/// Convert explicitly at the boundary: `.0` of the struct gives the `u64`, and `TaskId(x)` of the
/// struct wraps one. Any further type with this name must be told apart the same way.
pub type TaskId = u64;
pub struct TaskContext<E> {
    id: TaskId,
    priority: i32,
    open: AtomicUsize,
    cancelled: AtomicBool,
    first_error: OnceLock<E>,
}
impl<E> TaskContext<E> {
    pub fn new(id: TaskId, priority: i32, initial_open: usize) -> Arc<Self> {
        Arc::new(Self {
            id,
            priority,
            open: AtomicUsize::new(initial_open),
            cancelled: AtomicBool::new(false),
            first_error: OnceLock::new(),
        })
    }
    pub fn id(&self) -> TaskId {
        self.id
    }
    pub fn priority(&self) -> i32 {
        self.priority
    }
    pub fn open_operations(&self) -> usize {
        self.open.load(Ordering::SeqCst)
    }
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }
    pub fn first_error(&self) -> Option<&E> {
        self.first_error.get()
    }
    pub fn register_follow_ups(&self, n: usize) {
        if n > 0 {
            self.open.fetch_add(n, Ordering::SeqCst);
        }
    }
    pub fn complete_operation(&self) -> bool {
        let mut cur = self.open.load(Ordering::SeqCst);
        let prev = loop {
            if cur == 0 {
                break Err(());
            }
            match self
                .open
                .compare_exchange_weak(cur, cur - 1, Ordering::SeqCst, Ordering::SeqCst)
            {
                Ok(_) => break Ok(cur),
                Err(actual) => cur = actual,
            }
        };
        match prev {
            Ok(1) => !self.is_cancelled(),
            Ok(_) => false,
            Err(_) => {
                debug_assert!(
                    false, "complete_operation called with no open operations (task {})",
                    self.id
                );
                warn!(
                    "task {}: complete_operation called with no open operations", self.id
                );
                false
            }
        }
    }
    pub fn fail(&self, error: E) -> bool {
        match self.first_error.set(error) {
            Ok(()) => {
                self.cancelled.store(true, Ordering::SeqCst);
                warn!("task {} cancelled due to error", self.id);
                true
            }
            Err(_) => {
                debug!("task {}: additional error discarded", self.id);
                false
            }
        }
    }
}
#[cfg(test)]
#[path = "test/task_context.rs"]
mod tests;
