use std::collections::{BTreeMap, BTreeSet};

use crate::ir::{ObjectKind, ObjectTable};

const RESERVED: &[&str] = &[
    "abstract", "alloc", "as", "async", "await", "become", "box", "break", "const", "continue",
    "core", "crate", "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "gen",
    "if", "impl", "in", "let", "lib", "loop", "macro", "main", "match", "mod", "move", "mut",
    "override", "priv", "proc_macro", "pub", "ref", "return", "self", "static", "std", "struct",
    "super", "trait", "true", "try", "type", "typeof", "unsafe", "unsized", "use", "virtual",
    "where", "while", "yield",
];

#[derive(Debug, Default)]
pub struct Layout {
    modules: BTreeMap<String, String>,
    macros: BTreeSet<String>,
    used: BTreeSet<String>,
}

impl Layout {
    pub fn new(table: &ObjectTable) -> Self {
        let mut layout = Self::default();
        for object in table.iter() {
            let mut base = snake_case(&object.name);
            if RESERVED.contains(&base.as_str()) {
                base.push('_');
            }
            let module = layout.free_module(&base, object.kind);
            layout.used.insert(module.clone());
            layout.modules.insert(object.name.clone(), module);
            if object.kind == ObjectKind::Macro {
                layout.macros.insert(object.name.clone());
            }
        }
        layout
    }

    fn free_module(&self, base: &str, kind: ObjectKind) -> String {
        let labelled = format!("{base}_{}", kind.label());
        [base.to_string(), labelled.clone()]
            .into_iter()
            .find(|candidate| !self.used.contains(candidate))
            .or_else(|| {
                (2..)
                    .map(|number| format!("{labelled}_{number}"))
                    .find(|candidate| !self.used.contains(candidate))
            })
            .unwrap_or(labelled)
    }

    pub fn module(&self, object: &str) -> Option<&str> {
        self.modules.get(object).map(String::as_str)
    }

    pub fn is_macro(&self, object: &str) -> bool {
        self.macros.contains(object)
    }

    pub fn rewritable(&self, object: &str) -> Option<&str> {
        if self.is_macro(object) {
            return None;
        }
        self.module(object)
    }

    pub fn is_module_name(&self, name: &str) -> bool {
        self.used.contains(name)
    }
}

pub fn snake_case(name: &str) -> String {
    let chars: Vec<char> = name
        .chars()
        .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
        .collect();
    let mut out = String::new();
    for (index, &current) in chars.iter().enumerate() {
        if current.is_uppercase() {
            let previous = index.checked_sub(1).map(|p| chars[p]);
            let next = chars.get(index + 1);
            let boundary = previous.is_some_and(|prev| {
                prev != '_'
                    && (prev.is_lowercase()
                        || prev.is_ascii_digit()
                        || (prev.is_uppercase() && next.is_some_and(|n| n.is_lowercase())))
            });
            if boundary {
                out.push('_');
            }
            out.extend(current.to_lowercase());
        } else {
            out.push(current);
        }
    }
    if out.is_empty() || out.starts_with(|c: char| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    out
}

#[cfg(test)]
#[path = "test/layout.rs"]
mod tests;
