use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use syn::UseTree;
use crate::ir::ObjectTable;
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Import {
    pub file: PathBuf,
    pub local: Option<String>,
    pub path: String,
}
pub fn flatten_use(file: &Path, tree: &UseTree) -> Vec<Import> {
    let mut out = Vec::new();
    flatten_tree(file, tree, "", &mut out);
    out
}
fn join_path(prefix: &str, segment: &str) -> String {
    if prefix.is_empty() { segment.to_string() } else { format!("{prefix}::{segment}") }
}
fn flatten_tree(file: &Path, tree: &UseTree, prefix: &str, out: &mut Vec<Import>) {
    let import = |local: Option<String>, path: String| Import {
        file: file.to_path_buf(),
        local,
        path,
    };
    match tree {
        UseTree::Path(step) => {
            let next = join_path(prefix, &step.ident.to_string());
            flatten_tree(file, &step.tree, &next, out);
        }
        UseTree::Name(name) => {
            let ident = name.ident.to_string();
            if ident == "self" {
                let local = prefix.rsplit("::").next().map(str::to_string);
                out.push(import(local, prefix.to_string()));
            } else {
                out.push(import(Some(ident.clone()), join_path(prefix, &ident)));
            }
        }
        UseTree::Rename(rename) => {
            let ident = rename.ident.to_string();
            let alias = rename.rename.to_string();
            let base = if ident == "self" {
                prefix.to_string()
            } else {
                join_path(prefix, &ident)
            };
            let local = (alias != "_").then(|| alias.clone());
            out.push(import(local, format!("{base} as {alias}")));
        }
        UseTree::Glob(_) => out.push(import(None, join_path(prefix, "*"))),
        UseTree::Group(group) => {
            for item in &group.items {
                flatten_tree(file, item, prefix, out);
            }
        }
    }
}
fn root_of(path: &str) -> &str {
    let base = path.split(" as ").next().unwrap_or(path);
    base.split("::").next().unwrap_or(base)
}
fn is_internal(
    path: &str,
    qualifiers: &BTreeSet<String>,
    objects: &ObjectTable,
) -> bool {
    let root = root_of(path);
    matches!(root, "crate" | "self" | "super") || qualifiers.contains(root)
        || objects.contains(root)
}
pub fn select_imports(
    imports: &[Import],
    origins: &[(PathBuf, BTreeSet<String>)],
    qualifiers: &BTreeSet<String>,
    objects: &ObjectTable,
) -> Vec<String> {
    let mut referenced = Vec::new();
    let mut globs = Vec::new();
    let mut spare = Vec::new();
    for import in imports {
        let mut names = origins
            .iter()
            .filter(|(file, _)| *file == import.file)
            .map(|(_, names)| names)
            .peekable();
        if names.peek().is_none() || is_internal(&import.path, qualifiers, objects) {
            continue;
        }
        match &import.local {
            None => globs.push(import),
            Some(local) if names.any(|set| set.contains(local)) => {
                referenced.push(import)
            }
            Some(_) => spare.push(import),
        }
    }
    let mut bound: BTreeSet<String> = referenced
        .iter()
        .filter_map(|import| import.local.clone())
        .collect();
    let spare = spare
        .into_iter()
        .filter(|import| import.local.clone().is_some_and(|local| bound.insert(local)));
    let mut seen = BTreeSet::new();
    let mut selected = Vec::new();
    for import in referenced.iter().copied().chain(globs).chain(spare) {
        if seen.insert(import.path.clone()) {
            selected.push(import.path.clone());
        }
    }
    selected
}
#[cfg(test)]
#[path = "test/imports.rs"]
mod tests;
