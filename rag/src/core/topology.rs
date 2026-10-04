//! Dynamic topology (DAG) over the active layers of a query.
//!
//! Supports both purely sequential execution and a fork/join model for
//! parallel execution of independent layers. The topology is derived anew
//! per query from the active layers (`Layer::applies`) and their explicit
//! dependencies (`Layer::depends_on`), complemented by the coarse stage
//! ordering (Generate before Enrich before Postprocess) as a default edge
//! when no explicit dependencies exist.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;

use crate::core::layer::{Layer, LayerStage};

#[derive(Debug)]
pub struct CyclicDependencyError(pub String);

impl fmt::Display for CyclicDependencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cyclic dependency Layers: {}", self.0)
    }
}

impl std::error::Error for CyclicDependencyError {}

fn stage_order(stage: LayerStage) -> u8 {
    match stage {
        LayerStage::Generate => 0,
        LayerStage::Enrich => 1,
        LayerStage::Postprocess => 2,
    }
}

/// Topological sort into levels ("fork/join" groups).
///
/// Each level contains layers that can run in parallel, because all their
/// dependencies are already satisfied by previous levels. For sequential
/// execution the levels are simply processed one after another, and within
/// each level one layer at a time.
pub fn build_levels(
    layers: &[Arc<dyn Layer>],
) -> Result<Vec<Vec<Arc<dyn Layer>>>, CyclicDependencyError> {
    let by_id: HashMap<String, Arc<dyn Layer>> = layers
        .iter()
        .map(|l| (l.id().to_string(), l.clone()))
        .collect();

    let mut deps: HashMap<String, HashSet<String>> = HashMap::new();
    for layer in layers {
        let mut d: HashSet<String> = layer
            .depends_on()
            .into_iter()
            .filter(|dep| by_id.contains_key(dep))
            .collect();
        for other in layers {
            if other.id() != layer.id() && stage_order(other.stage()) < stage_order(layer.stage())
            {
                d.insert(other.id().to_string());
            }
        }
        deps.insert(layer.id().to_string(), d);
    }

    let mut remaining = deps;
    let mut done: HashSet<String> = HashSet::new();
    let mut levels: Vec<Vec<Arc<dyn Layer>>> = Vec::new();

    while !remaining.is_empty() {
        let ready: Vec<String> = remaining
            .iter()
            .filter(|(_, d)| d.is_subset(&done))
            .map(|(k, _)| k.clone())
            .collect();
        if ready.is_empty() {
            let mut names: Vec<String> = remaining.keys().cloned().collect();
            names.sort();
            return Err(CyclicDependencyError(format!("{:?}", names)));
        }
        levels.push(ready.iter().map(|id| by_id[id].clone()).collect());
        for id in &ready {
            done.insert(id.clone());
            remaining.remove(id);
        }
    }

    Ok(levels)
}
