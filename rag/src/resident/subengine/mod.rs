//! Shared subengine lifecycle & pull model, tokenizer/normalizer (CPU).

#[cfg(feature = "cuda")]
mod device;
#[cfg(feature = "cuda")]
pub use device::*;
