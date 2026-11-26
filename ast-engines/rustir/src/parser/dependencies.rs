use std::collections::BTreeSet;
use proc_macro2::{TokenStream, TokenTree};
use syn::visit::{self, Visit};
use syn::Item;
#[derive(Default)]
struct NameCollector {
    names: BTreeSet<String>,
}
impl NameCollector {
    fn insert_name(&mut self, name: &str) {
        if !matches!(name, "crate" | "self" | "super" | "Self") {
            self.names.insert(name.to_string());
        }
    }
    fn scan_tokens(&mut self, tokens: TokenStream) {
        for tree in tokens {
            match tree {
                TokenTree::Ident(ident) => {
                    self.names.insert(ident.to_string());
                }
                TokenTree::Group(group) => self.scan_tokens(group.stream()),
                TokenTree::Literal(literal) => self.scan_captures(&literal.to_string()),
                _ => {}
            }
        }
    }
    fn scan_captures(&mut self, literal: &str) {
        let mut chars = literal.chars().peekable();
        while let Some(current) = chars.next() {
            if current != '{' {
                continue;
            }
            if chars.peek() == Some(&'{') {
                chars.next();
                continue;
            }
            let name: String = chars
                .by_ref()
                .take_while(|c| c.is_alphanumeric() || *c == '_')
                .collect();
            if name.starts_with(|c: char| c.is_alphabetic() || c == '_') {
                self.names.insert(name);
            }
        }
    }
}
impl<'ast> Visit<'ast> for NameCollector {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        let segments: Vec<String> = path
            .segments
            .iter()
            .map(|s| s.ident.to_string())
            .collect();
        if let Some((last, leading)) = segments.split_last() {
            self.insert_name(last);
            if let Some(parent) = leading
                .last()
                .filter(|s| s.starts_with(char::is_uppercase))
            {
                self.insert_name(parent);
            }
        }
        visit::visit_path(self, path);
    }
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        self.visit_path(&mac.path);
        self.scan_tokens(mac.tokens.clone());
    }
}
pub fn item_names(item: &Item) -> BTreeSet<String> {
    let mut collector = NameCollector::default();
    collector.visit_item(item);
    collector.names
}
#[cfg(test)]
#[path = "test/dependencies.rs"]
mod tests;
