use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use crate::error::{Error, Result};
use crate::ir::{Conflicts, ObjectTable, Project};
use crate::parser::collect::{collect_items, Collected, ImplBlock};
use crate::parser::files::source_files;
use crate::parser::imports::select_imports;
use crate::parser::manifest::read_manifest;
type Origins = BTreeMap<String, Vec<(PathBuf, BTreeSet<String>)>>;
pub fn parse_project(root: &Path) -> Result<Project> {
    if !root.is_dir() {
        return Err(Error::NotFound(root.to_path_buf()));
    }
    let mut collected = Collected::default();
    for file in source_files(root)? {
        let text = fs::read_to_string(&file)?;
        let ast = syn::parse_file(&text)
            .map_err(|error| Error::Parse {
                path: file.clone(),
                message: error.to_string(),
            })?;
        collect_items(&file, &text, &ast.items, &mut collected);
    }
    let mut origins = Conflicts::new();
    for located in &collected.objects {
        origins.record(&located.object.name, &located.file);
    }
    let duplicates = origins.duplicates();
    if !duplicates.is_empty() {
        return Err(Error::NameConflicts(duplicates));
    }
    let mut project = Project::new(root.to_path_buf());
    project.manifest = read_manifest(root)?;
    let mut sources = Origins::new();
    for located in collected.objects {
        let names = located.object.dependencies.iter().cloned().collect();
        sources
            .entry(located.object.name.clone())
            .or_default()
            .push((located.scope, names));
        if let Some(gate) = located.gate {
            project.gates.insert(located.object.name.clone(), gate);
        }
        project.objects.insert(located.object);
    }
    for block in collected.impls {
        if let Some(owner) = owner_of(&block, &project.objects) {
            let names = block.names.iter().cloned().collect();
            sources.entry(owner.clone()).or_default().push((block.scope.clone(), names));
            project.objects.attach(&owner, &block.body, block.names);
        }
    }
    project.qualifiers = collected.modules;
    if let Some(package) = &project.manifest.package {
        project.qualifiers.insert(package.replace('-', "_"));
    } else if let Some(name) = root_module(root) {
        project.qualifiers.insert(name);
    }
    project.objects.resolve_dependencies();
    for (owner, origins) in sources {
        let selected = select_imports(
            &collected.imports,
            &origins,
            &project.qualifiers,
            &project.objects,
        );
        if !selected.is_empty() {
            project.imports.insert(owner, selected);
        }
    }
    Ok(project)
}
fn root_module(root: &Path) -> Option<String> {
    let name = root.file_name()?.to_str()?;
    let identifier = !name.starts_with(|c: char| c.is_ascii_digit()) && !name.is_empty()
        && name.chars().all(|c| c.is_alphanumeric() || c == '_');
    identifier.then(|| name.to_string())
}
fn owner_of(block: &ImplBlock, table: &ObjectTable) -> Option<String> {
    block
        .self_name
        .iter()
        .chain(block.trait_name.iter())
        .chain(block.names.iter())
        .find(|name| table.contains(name))
        .cloned()
}
#[cfg(test)]
#[path = "test/project.rs"]
mod tests;
