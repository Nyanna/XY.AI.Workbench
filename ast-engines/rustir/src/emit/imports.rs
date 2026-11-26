use std::collections::BTreeSet;
use crate::emit::layout::Layout;
use crate::ir::{Object, Project};
pub fn gate_attribute(gate: Option<&str>) -> String {
    gate.map_or_else(String::new, |predicate| format!("#[cfg({predicate})]\n"))
}
pub fn local_name(path: &str) -> Option<&str> {
    if let Some((_, alias)) = path.rsplit_once(" as ") {
        return (alias != "_").then_some(alias);
    }
    path.rsplit("::").next().filter(|last| *last != "*")
}
fn mentions(body: &str, name: &str) -> bool {
    let is_ident = |c: char| c.is_alphanumeric() || c == '_';
    body.match_indices(name)
        .any(|(start, found)| {
            let before = body[..start].chars().next_back();
            let after = body[start + found.len()..].chars().next();
            !before.is_some_and(is_ident) && !after.is_some_and(is_ident)
        })
}
pub fn use_block(project: &Project, layout: &Layout, object: &Object) -> String {
    let internal: BTreeSet<&str> = object
        .dependencies
        .iter()
        .map(String::as_str)
        .filter(|name| *name != object.name && project.objects.contains(name))
        .filter(|name| !layout.is_macro(name))
        .collect();
    let alias_only = |name: &str| {
        project
            .aliases
            .get(&object.name)
            .is_some_and(|aliases| aliases.values().any(|target| target == name))
            && !mentions(&object.body, name)
    };
    let mut lines: Vec<String> = internal
        .iter()
        .filter(|name| !alias_only(name))
        .filter_map(|name| {
            layout
                .module(name)
                .map(|module| {
                    let gate = project.gates.get(*name).map(String::as_str);
                    format!("{}use crate::{module}::{name};", gate_attribute(gate))
                })
        })
        .collect();
    let aliased: BTreeSet<String> = project
        .aliases
        .get(&object.name)
        .into_iter()
        .flatten()
        .filter(|(alias, _)| !internal.contains(alias.as_str()))
        .filter(|(_, target)| !layout.is_macro(target))
        .filter_map(|(alias, target)| {
            let module = layout.module(target)?;
            let gate = project.gates.get(target).map(String::as_str);
            Some(
                format!(
                    "{}use crate::{module}::{target} as {alias};", gate_attribute(gate)
                ),
            )
        })
        .collect();
    lines.extend(aliased);
    let external: BTreeSet<String> = project
        .imports
        .get(&object.name)
        .into_iter()
        .flatten()
        .filter(|path| {
            local_name(path)
                .map_or(true, |local| local != object.name && !internal.contains(local))
        })
        .map(|path| format!("use {path};"))
        .collect();
    lines.extend(external);
    if lines.is_empty() {
        return String::new();
    }
    format!("{}\n\n", lines.join("\n"))
}
#[cfg(test)]
#[path = "test/imports.rs"]
mod tests;
