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
    let edges = read_edges(file, &text)?;
    for (owner, dependencies) in edges.dependencies {
        project.objects.set_dependencies(&owner, dependencies);
    }
    for (owner, aliases) in edges.aliases {
        if project.objects.contains(&owner) {
            project.aliases.insert(owner, aliases);
        }
    }
    Ok(())
}
#[derive(Debug, Default)]
struct Edges {
    dependencies: BTreeMap<String, Vec<String>>,
    aliases: BTreeMap<String, BTreeMap<String, String>>,
}
fn read_edges(file: &Path, text: &str) -> Result<Edges> {
    let mut edges = Edges::default();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let mut columns = line.splitn(3, '\t');
        let owner = columns.next().unwrap_or_default();
        let dependency = columns
            .next()
            .ok_or_else(|| Error::Parse {
                path: file.to_path_buf(),
                message: format!(
                    "line {}: expected <object><TAB><dependency>", index + 1
                ),
            })?;
        let list = edges.dependencies.entry(owner.to_string()).or_default();
        if dependency.is_empty() {
            continue;
        }
        list.push(dependency.to_string());
        if let Some(alias) = columns.next().filter(|alias| !alias.is_empty()) {
            edges
                .aliases
                .entry(owner.to_string())
                .or_default()
                .insert(alias.to_string(), dependency.to_string());
        }
    }
    Ok(edges)
}
#[cfg(test)]
#[path = "test/edges.rs"]
mod tests;
