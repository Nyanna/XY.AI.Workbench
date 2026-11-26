use std::collections::{BTreeMap, BTreeSet};
use crate::ir::Object;
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ObjectTable {
    objects: BTreeMap<String, Object>,
}
impl ObjectTable {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, object: Object) -> bool {
        if self.objects.contains_key(&object.name) {
            return false;
        }
        self.objects.insert(object.name.clone(), object);
        true
    }
    pub fn contains(&self, name: &str) -> bool {
        self.objects.contains_key(name)
    }
    pub fn get(&self, name: &str) -> Option<&Object> {
        self.objects.get(name)
    }
    pub fn len(&self) -> usize {
        self.objects.len()
    }
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }
    pub fn iter(&self) -> impl Iterator<Item = &Object> {
        self.objects.values()
    }
    pub fn set_dependencies(&mut self, name: &str, dependencies: Vec<String>) -> bool {
        match self.objects.get_mut(name) {
            Some(object) => {
                let own = object.name.clone();
                object.dependencies = dependencies;
                object.dependencies.retain(|dependency| *dependency != own);
                object.dependencies.sort();
                object.dependencies.dedup();
                true
            }
            None => false,
        }
    }
    pub fn attach<I>(&mut self, owner: &str, body: &str, names: I) -> bool
    where
        I: IntoIterator<Item = String>,
    {
        match self.objects.get_mut(owner) {
            Some(object) => {
                object.body.push_str("\n\n");
                object.body.push_str(body);
                object.dependencies.extend(names);
                true
            }
            None => false,
        }
    }
    pub fn resolve_dependencies(&mut self) {
        let known: BTreeSet<String> = self.objects.keys().cloned().collect();
        for object in self.objects.values_mut() {
            let own = object.name.clone();
            object
                .dependencies
                .retain(|dependency| *dependency != own && known.contains(dependency));
            object.dependencies.sort();
            object.dependencies.dedup();
        }
    }
}
#[cfg(test)]
#[path = "test/table.rs"]
mod tests;
