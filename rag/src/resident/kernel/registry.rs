//! Lazily populated kernel table keyed by `(name, dtype, sm, variant)`.

use std::collections::HashMap;
use std::sync::Arc;

use parking_lot::RwLock;

use super::KernelError;
use crate::resident::memory::DType;
use crate::resident::subengine::{Resource, ResourceKey};

/// `variant` selects compile-time specialisations such as the embedding dimension.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct KernelKey {
    pub name: &'static str,
    pub dtype: DType,
    pub sm: u32,
    pub variant: u32,
}

impl KernelKey {
    pub const fn new(name: &'static str, dtype: DType, sm: u32, variant: u32) -> Self {
        Self { name, dtype, sm, variant }
    }
}

/// Produces the handle for a key, e.g. by compiling with NVRTC.
pub trait KernelLoader<F>: Send + Sync {
    fn load(&self, key: &KernelKey) -> Result<F, KernelError>;
}

pub struct KernelRegistry<F> {
    loader: Arc<dyn KernelLoader<F>>,
    entries: RwLock<HashMap<KernelKey, F>>,
}

impl<F: Clone + Send + Sync + 'static> KernelRegistry<F> {
    pub fn new(loader: Arc<dyn KernelLoader<F>>) -> Self {
        Self { loader, entries: RwLock::new(HashMap::new()) }
    }

    /// Loads on first use. Loading holds the write lock, so concurrent callers never compile the
    /// same key twice.
    pub fn get(&self, key: &KernelKey) -> Result<F, KernelError> {
        if let Some(f) = self.entries.read().get(key) {
            return Ok(f.clone());
        }
        let mut entries = self.entries.write();
        if let Some(f) = entries.get(key) {
            return Ok(f.clone());
        }
        let started = std::time::Instant::now();
        let f = self.loader.load(key)?;
        tracing::info!(
            kernel = key.name,
            dtype = ?key.dtype,
            sm = key.sm,
            variant = key.variant,
            elapsed_ms = started.elapsed().as_millis() as u64,
            "kernel loaded"
        );
        entries.insert(*key, f.clone());
        Ok(f)
    }

    pub fn len(&self) -> usize {
        self.entries.read().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Drops every loaded kernel; the next `get` reloads.
    pub fn evict_all(&self) -> usize {
        let mut entries = self.entries.write();
        let n = entries.len();
        entries.clear();
        n
    }
}

impl<F: Clone + Send + Sync + 'static> Resource for KernelRegistry<F> {
    fn key(&self) -> ResourceKey {
        ResourceKey::new("kernel-registry")
    }
}

#[cfg(test)]
#[path = "test/registry.rs"]
mod tests;
