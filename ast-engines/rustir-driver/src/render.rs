use std::collections::{BTreeMap, BTreeSet};

pub fn render_edges(edges: &BTreeMap<String, BTreeSet<String>>) -> String {
    let mut text = String::new();
    for (owner, dependencies) in edges {
        if dependencies.is_empty() {
            text.push_str(&format!("{owner}\t\n"));
        }
        for dependency in dependencies {
            text.push_str(&format!("{owner}\t{dependency}\n"));
        }
    }
    text
}

#[cfg(test)]
#[path = "test/render.rs"]
mod tests;
