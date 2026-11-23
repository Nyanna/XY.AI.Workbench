use std::sync::Arc;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020::a::index_contract::Index;

/// Lazily activated index. `get` activates on its first successful call and then returns the same Arc.
/// A failure is not cached. Callers never call it on a lookup path.
pub trait LazyIndex: Send + Sync {
    fn get(&self) -> Result<Arc<dyn Index>, String>;
}

fn _object_safe(_: &dyn LazyIndex) {}

#[cfg(test)]
#[path = "test/manager.rs"]
mod tests;
