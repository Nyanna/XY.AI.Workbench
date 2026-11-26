//! Typed extension map carrying additional addon task parameters.
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
type Entry2 = Arc<dyn Any + Send + Sync>;
/// Map holding at most one value per type; clones share the stored values.
#[derive(Clone, Default)]
pub struct Extensions {
    map: HashMap<TypeId, Entry2>,
}
impl Extensions {
    pub fn new() -> Self {
        Self::default()
    }
    /// Stores `value`; replaces an existing value of type T.
    pub fn insert<T: Any + Send + Sync + 'static>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Arc::new(value));
    }
    /// Value of type T, or None.
    pub fn get<T: Any + Send + Sync + 'static>(&self) -> Option<&T> {
        self.map.get(&TypeId::of::<T>()).and_then(|entry| (**entry).downcast_ref::<T>())
    }
    /// Removes the value of type T; true if one was present.
    pub fn remove<T: Any + Send + Sync + 'static>(&mut self) -> bool {
        self.map.remove(&TypeId::of::<T>()).is_some()
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}
impl fmt::Debug for Extensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Extensions").field("len", &self.len()).finish()
    }
}
/// Equal when both hold the same types and each type refers to the same shared value.
impl PartialEq for Extensions {
    fn eq(&self, other: &Self) -> bool {
        self.map.len() == other.map.len()
            && self
                .map
                .iter()
                .all(|(key, entry)| {
                    other.map.get(key).is_some_and(|rhs| Arc::ptr_eq(entry, rhs))
                })
    }
}
impl Eq for Extensions {}
#[cfg(test)]
#[path = "test/extensions.rs"]
mod tests;
