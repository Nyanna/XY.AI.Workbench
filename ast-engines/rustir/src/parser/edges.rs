use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::error::{Error, Result};
use crate::ir::Project;

pub fn load_edges(project: &mut Project, file: &Path) -> Result<()> {
    if !file.is_file() {
        return Err(Error::NotFound(file.to_path_buf()));
    }
    let text = fs::read_to_string(file)?;
    for (owner, dependencies) in read_edges(file, &text)? {
        project.objects.set_dependencies(&owner, dependencies);
    }
    Ok(())
}

fn read_edges(file: &Path, text: &str) -> Result<BTreeMap<String, Vec<String>>> {
    let mut edges: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let (owner, dependency) = line.split_once('\t').ok_or_else(|| Error::Parse {
            path: file.to_path_buf(),
            message: format!("line {}: expected <object><TAB><dependency>", index + 1),
        })?;
        let list = edges.entry(owner.to_string()).or_default();
        if !dependency.is_empty() {
            list.push(dependency.to_string());
        }
    }
    Ok(edges)
}

#[cfg(test)]
#[path = "test/edges.rs"]
mod tests;
