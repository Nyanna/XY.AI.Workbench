use std::sync::Arc;

use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::Index;

/// Lazily activated index. `get` activates on its first successful call and then returns the same Arc.
/// A failure is not cached. Callers never call it on a lookup path.
pub trait LazyIndex: Send + Sync {
    fn get(&self) -> Result<Arc<dyn Index>, String>;
}

fn _object_safe3(_: &dyn LazyIndex) {}

#[cfg(test)]
#[path = "test/manager.rs"]
mod tests;
