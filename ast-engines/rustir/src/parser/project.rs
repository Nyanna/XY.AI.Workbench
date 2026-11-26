use std::fs;
use std::path::Path;

use crate::error::{Error, Result};
use crate::ir::{Conflicts, ObjectTable, Project};
use crate::parser::collect::{collect_items, Collected, ImplBlock};
use crate::parser::files::source_files;

pub fn parse_project(root: &Path) -> Result<Project> {
    if !root.is_dir() {
        return Err(Error::NotFound(root.to_path_buf()));
    }
    let mut collected = Collected::default();
    for file in source_files(root)? {
        let text = fs::read_to_string(&file)?;
        let ast = syn::parse_file(&text).map_err(|error| Error::Parse {
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
    for located in collected.objects {
        project.objects.insert(located.object);
    }
    for block in collected.impls {
        if let Some(owner) = owner_of(&block, &project.objects) {
            project.objects.attach(&owner, &block.body, block.names);
        }
    }
    project.objects.resolve_dependencies();
    Ok(project)
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
