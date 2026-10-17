//! Mock/test-double conventions (Q3).
//!
//! Narrow traits for GPU/measurement interfaces use generics (static dispatch) in the hot
//! path; `dyn` only at the edges. Deterministic fakes (fixed output, simulated failure) are
//! hand-written rather than `mockall`-generated, since that composes more simply across
//! `async`/`Send` boundaries; `mockall::automock` is reserved for orchestrator-level traits.
//! Everything here is gated so it never ships in a production build without the `mock` feature.

#![cfg(any(test, feature = "mock"))]

/// Example of a narrow trait a hot-path component depends on generically.
pub trait Clock: Send + Sync {
    fn now_millis(&self) -> u64;
}

/// Hand-written deterministic fake, per the convention above.
pub struct FakeClock {
    pub fixed_millis: u64,
}

impl Clock for FakeClock {
    fn now_millis(&self) -> u64 {
        self.fixed_millis
    }
}
