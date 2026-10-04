//! Registry for layer implementations.

use std::sync::Arc;

use anyhow::{bail, Result};

use crate::core::layer::Layer;

/// Manages all available layer instances, addressed by their id.
#[derive(Default)]
pub struct LayerRegistry {
    layers: Vec<Arc<dyn Layer>>,
}

impl LayerRegistry {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn register(&mut self, layer: Arc<dyn Layer>) -> Result<()> {
        if layer.id().is_empty() {
            bail!("Layer needs a unique non empty ID");
        }
        if self.layers.iter().any(|l| l.id() == layer.id()) {
            bail!("Layer-ID already registered: {}", layer.id());
        }
        self.layers.push(layer);
        Ok(())
    }

    pub fn unregister(&mut self, layer_id: &str) {
        self.layers.retain(|l| l.id() != layer_id);
    }

    pub fn get(&self, layer_id: &str) -> Option<Arc<dyn Layer>> {
        self.layers.iter().find(|l| l.id() == layer_id).cloned()
    }

    pub fn all(&self) -> Vec<Arc<dyn Layer>> {
        self.layers.clone()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Arc<dyn Layer>> {
        self.layers.iter()
    }

    pub fn len(&self) -> usize {
        self.layers.len()
    }

    pub fn is_empty(&self) -> bool {
        self.layers.is_empty()
    }

    pub fn contains(&self, layer_id: &str) -> bool {
        self.layers.iter().any(|l| l.id() == layer_id)
    }
}
