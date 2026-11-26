use std::collections::BTreeSet;
use proc_macro2::{Ident, TokenStream, TokenTree};
use syn::visit::{self, Visit};
use syn::{File, ItemUse, UseTree};
use crate::emit::edits::{Edit, Source};
use crate::emit::layout::Layout;
struct PathRewriter<'a> {
    source: &'a Source<'a>,
    layout: &'a Layout,
    qualifiers: &'a BTreeSet<String>,
    edits: Vec<Edit>,
}
impl PathRewriter<'_> {
    fn is_relative(&self, name: &str) -> bool {
        matches!(name, "crate" | "self" | "super") || self.qualifiers.contains(name)
    }
    fn rewrite(&self, idents: &[Ident]) -> Option<Edit> {
        let first = idents.first()?;
        if !self.is_relative(&first.to_string()) {
            return None;
        }
        let mut target = None;
        for ident in idents.iter().skip(1) {
            let name = ident.to_string();
            if let Some(module) = self.layout.rewritable(&name) {
                target = Some((ident.span(), format!("crate::{module}::{name}")));
            }
            if !self.is_relative(&name) {
                break;
            }
        }
        let (last, text) = target?;
        Some(Edit {
            start: self.source.start_of(first.span()),
            end: self.source.end_of(last),
            text,
        })
    }
    fn scan_tokens(&mut self, tokens: TokenStream) {
        let trees: Vec<TokenTree> = tokens.into_iter().collect();
        let mut index = 0;
        while index < trees.len() {
            match &trees[index] {
                TokenTree::Group(group) => {
                    self.scan_tokens(group.stream());
                    index += 1;
                }
                TokenTree::Ident(first) => {
                    let mut idents = vec![first.clone()];
                    index += 1;
                    while let Some(ident) = path_continuation(&trees, index) {
                        idents.push(ident);
                        index += 3;
                    }
                    self.edits.extend(self.rewrite(&idents));
                }
                _ => index += 1,
            }
        }
    }
}
fn path_continuation(trees: &[TokenTree], index: usize) -> Option<Ident> {
    match (trees.get(index), trees.get(index + 1), trees.get(index + 2)) {
        (
            Some(TokenTree::Punct(a)),
            Some(TokenTree::Punct(b)),
            Some(TokenTree::Ident(ident)),
        ) if a.as_char() == ':' && b.as_char() == ':' => Some(ident.clone()),
        _ => None,
    }
}
impl<'ast> Visit<'ast> for PathRewriter<'_> {
    fn visit_path(&mut self, path: &'ast syn::Path) {
        let idents: Vec<Ident> = path.segments.iter().map(|s| s.ident.clone()).collect();
        self.edits.extend(self.rewrite(&idents));
        visit::visit_path(self, path);
    }
    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        self.scan_tokens(mac.tokens.clone());
        visit::visit_macro(self, mac);
    }
    fn visit_item_use(&mut self, item: &'ast ItemUse) {
        let internal = leading_module(&item.tree)
            .is_some_and(|name| self.qualifiers.contains(&name));
        if internal && !has_rename(&item.tree) {
            let first = item
                .attrs
                .first()
                .map_or(item.use_token.span, |attr| attr.pound_token.span);
            self.edits
                .push(Edit {
                    start: self.source.start_of(first),
                    end: self.source.end_of(item.semi_token.span),
                    text: String::new(),
                });
        }
    }
}
fn leading_module(tree: &UseTree) -> Option<String> {
    match tree {
        UseTree::Path(step) => {
            let name = step.ident.to_string();
            if matches!(name.as_str(), "crate" | "self" | "super") {
                leading_module(&step.tree)
            } else {
                Some(name)
            }
        }
        _ => None,
    }
}
fn has_rename(tree: &UseTree) -> bool {
    match tree {
        UseTree::Rename(_) => true,
        UseTree::Path(step) => has_rename(&step.tree),
        UseTree::Group(group) => group.items.iter().any(has_rename),
        _ => false,
    }
}
pub fn path_edits(
    file: &File,
    source: &Source,
    layout: &Layout,
    qualifiers: &BTreeSet<String>,
) -> Vec<Edit> {
    let mut rewriter = PathRewriter {
        source,
        layout,
        qualifiers,
        edits: Vec::new(),
    };
    rewriter.visit_file(file);
    rewriter.edits
}
#[cfg(test)]
#[path = "test/paths.rs"]
mod tests;
