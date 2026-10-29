//! Named fault-injection points at synchronisation boundaries.
//!
//! Without `test`/`mock` every call compiles to a constant `false`. Points are keyed by
//! `(name, key)` so concurrent tests on different instances do not interfere.

#[cfg(any(test, feature = "mock"))]
mod imp {
    use std::collections::HashMap;
    use std::sync::LazyLock;

    use parking_lot::Mutex;

    static ARMED: LazyLock<Mutex<HashMap<(&'static str, u64), u32>>> =
        LazyLock::new(|| Mutex::new(HashMap::new()));

    /// Disarms the point on drop.
    pub struct Armed(&'static str, u64);

    impl Drop for Armed {
        fn drop(&mut self) {
            ARMED.lock().remove(&(self.0, self.1));
        }
    }

    /// Makes the next `times` hits of `(name, key)` fire.
    pub fn arm(name: &'static str, key: u64, times: u32) -> Armed {
        ARMED.lock().insert((name, key), times);
        Armed(name, key)
    }

    pub fn hit(name: &'static str, key: u64) -> bool {
        let mut g = ARMED.lock();
        match g.get_mut(&(name, key)) {
            Some(n) if *n > 0 => {
                *n -= 1;
                true
            }
            _ => false,
        }
    }
}

#[cfg(any(test, feature = "mock"))]
pub use imp::{arm, hit, Armed};

#[cfg(not(any(test, feature = "mock")))]
#[inline(always)]
pub fn hit(_name: &'static str, _key: u64) -> bool {
    false
}
