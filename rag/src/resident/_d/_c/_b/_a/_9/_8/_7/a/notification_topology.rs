use std::sync::Arc;

pub type Wake = Arc<dyn Fn() + Send + Sync>;

/// Per-sub-engine handle. Cheap to clone.
#[derive(Clone)]
pub struct SubengineNotifier {
    wake: Wake,
}

impl SubengineNotifier {
    fn new(wake: Wake) -> Self {
        Self { wake }
    }

    pub fn notify_result(&self) {
        (self.wake)();
    }

    pub fn notify_init_done(&self) {
        (self.wake)();
    }

    pub fn notify_eviction_timer(&self) {
        (self.wake)();
    }

    /// Wakes only if `progress` is true.
    pub fn notify_iteration(&self, progress: bool) {
        if progress {
            (self.wake)();
        }
    }
}

pub struct NotificationTopology {
    notifiers: Vec<SubengineNotifier>,
    wake: Wake,
}

impl NotificationTopology {
    /// Creates `subengines` handles (may be 0), index = sub-engine id.
    pub fn new(subengines: usize, wake: Wake) -> Self {
        let notifiers = (0..subengines)
            .map(|_| SubengineNotifier::new(Arc::clone(&wake)))
            .collect();
        Self { notifiers, wake }
    }

    pub fn len(&self) -> usize {
        self.notifiers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.notifiers.is_empty()
    }

    pub fn notifier(&self, index: usize) -> Option<SubengineNotifier> {
        self.notifiers.get(index).cloned()
    }

    /// Called by producers after a push into an operations queue.
    pub fn notify_push(&self) {
        (self.wake)();
    }
}

#[cfg(test)]
#[path = "test/notification_topology.rs"]
mod tests;
