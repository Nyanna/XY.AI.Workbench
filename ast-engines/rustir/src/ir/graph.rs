use std::collections::{BTreeMap, BTreeSet};

use crate::ir::ObjectTable;

pub fn dependents(table: &ObjectTable) -> BTreeMap<String, BTreeSet<String>> {
    let mut incoming: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for object in table.iter() {
        for dependency in &object.dependencies {
            if *dependency != object.name && table.contains(dependency) {
                incoming
                    .entry(dependency.clone())
                    .or_default()
                    .insert(object.name.clone());
            }
        }
    }
    incoming
}

#[cfg(test)]
#[path = "test/graph.rs"]
mod tests;
