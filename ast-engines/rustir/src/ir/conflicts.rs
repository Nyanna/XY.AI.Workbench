use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::error::NameConflict;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Conflicts {
    origins: BTreeMap<String, Vec<PathBuf>>,
}

impl Conflicts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, name: &str, file: &Path) {
        self.origins
            .entry(name.to_string())
            .or_default()
            .push(file.to_path_buf());
    }

    pub fn duplicates(&self) -> Vec<NameConflict> {
        self.origins
            .iter()
            .filter(|(_, files)| files.len() > 1)
            .map(|(name, files)| NameConflict {
                name: name.clone(),
                files: files.clone(),
            })
            .collect()
    }
}

#[cfg(test)]
#[path = "test/conflicts.rs"]
mod tests;
