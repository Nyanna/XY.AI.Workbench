use std::any::Any;
use std::sync::{Mutex, MutexGuard};

/// Locks the mutex and recovers the guard of a poisoned lock.
pub fn lock<T: ?Sized>(m: &Mutex<T>) -> MutexGuard<'_, T> {
    m.lock().unwrap_or_else(|e| e.into_inner())
}

/// Text of a panic payload; `None` if the payload is neither `&str` nor `String`.
pub fn panic_payload_text(payload: &(dyn Any + Send)) -> Option<&str> {
    if let Some(s) = payload.downcast_ref::<&str>() {
        Some(s)
    } else {
        payload.downcast_ref::<String>().map(String::as_str)
    }
}

#[cfg(test)]
#[path = "test/sync_util.rs"]
mod tests;
