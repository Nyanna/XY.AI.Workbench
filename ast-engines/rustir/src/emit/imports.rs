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
pub fn use_block(project: &Project, layout: &Layout, object: &Object) -> String {
    let internal: BTreeSet<&str> = object
        .dependencies
        .iter()
        .map(String::as_str)
        .filter(|name| *name != object.name && project.objects.contains(name))
        .filter(|name| !layout.is_macro(name))
        .collect();
    let mut lines: Vec<String> = internal
        .iter()
        .filter_map(|name| {
            layout
                .module(name)
                .map(|module| {
                    let gate = project.gates.get(*name).map(String::as_str);
                    format!("{}use crate::{module}::{name};", gate_attribute(gate))
                })
        })
        .collect();
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
