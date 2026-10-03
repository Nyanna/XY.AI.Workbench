//! Flattens a `syn::File` into every addressable node, in document order — the Rust
//! analogue of Java's `NodeLocator`. Individually addressable "definitions" are
//! functions, types (struct/enum/union/trait/trait-alias/type-alias), modules and
//! impls; module/impl/trait bodies recurse, everything else (`use`, `const`,
//! `static`, `extern crate`, item macros, foreign blocks) is far too fine-grained
//! to be individually useful, so consecutive runs of it are collapsed into anonymous
//! group segments capped at [`SEGMENT_MAX_CHARS`] — "ImportGroup" for `use` runs,
//! "StatementGroup" otherwise — exactly like the Java/Python/tree-sitter engines'
//! "imports"/"statements" segments. Function/method bodies are leaves, never expanded.
//!
//! Ids are name-based structural paths (e.g. `"MyStruct.foo"`) for named definitions,
//! and stable content-hash paths (e.g. `"a1B2c3|d4E5f6"`) for anonymous group
//! segments, rebuilt fresh on every call from the live tree.

use std::collections::HashMap;

use sha1::{Digest, Sha1};
use syn::spanned::Spanned;
use syn::{File, ImplItem, Item, TraitItem};

use crate::engine::addressable_node::AddressableNode;
use crate::engine::node_path::{NodePath, Owner};

/// A group keeps accumulating siblings until adding the next one would push its (token) length past this many characters.
const SEGMENT_MAX_CHARS: usize = 1000;
const HASH_ALPHABET: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn locate_all(file: &File) -> Vec<AddressableNode> {
    let mut out = Vec::new();
    walk_items(&file.items, Vec::new(), "File", "", &mut out);
    out
}

fn line(span: proc_macro2::Span) -> i64 {
    span.start().line as i64
}

fn end_line(span: proc_macro2::Span) -> i64 {
    span.end().line as i64
}

fn sanitize(name: &str) -> String {
    let mut out = String::with_capacity(name.len());
    for c in name.chars() {
        if c.is_ascii_alphanumeric() || c == '_' { out.push(c); } else { out.push('_'); }
    }
    out
}

fn segment(name: Option<&str>, type_name: &str, used: &mut HashMap<String, i32>) -> String {
    let base = match name { Some(n) => sanitize(n), None => type_name.to_string() };
    let count = used.entry(base.clone()).and_modify(|c| *c += 1).or_insert(1);
    if *count == 1 { base } else { format!("{}_{}", base, count) }
}

fn base62_hash(text: &str, length: usize) -> String {
    let digest = Sha1::digest(text.as_bytes());
    // Big-endian byte sequence treated as a big integer, repeatedly reduced mod 62 -- same as Java's BigInteger approach.
    let mut digits: Vec<u8> = digest.to_vec();
    let mut out = String::with_capacity(length);
    for _ in 0..length {
        let mut remainder: u32 = 0;
        for d in digits.iter_mut() {
            let acc = remainder * 256 + *d as u32;
            *d = (acc / 62) as u8;
            remainder = acc % 62;
        }
        out.push(HASH_ALPHABET[remainder as usize] as char);
    }
    out
}

fn content_hash(content: &str) -> String {
    base62_hash(content, 6)
}

/// Hash of the group's whitespace-stripped first/last 20 chars: stays put even when unrelated edits shift the group's interior.
fn content_prefix_hash(content: &str) -> String {
    let stripped: String = content.chars().filter(|c| !c.is_whitespace()).collect();
    let len = stripped.chars().count();
    let chars: Vec<char> = stripped.chars().collect();
    let prefix: String = chars[..len.min(20)].iter().collect();
    let suffix_start = if len > 20 { len - 20 } else { 0 };
    let suffix: String = chars[suffix_start..].iter().collect();
    base62_hash(&format!("{}{}", prefix, suffix), 6)
}

fn anonymous_segment(content: &str, used: &mut HashMap<String, i32>) -> String {
    let base = format!("{}|{}", content_prefix_hash(content), content_hash(content));
    let count = used.entry(base.clone()).and_modify(|c| *c += 1).or_insert(1);
    if *count == 1 { base } else { format!("{}_{}", base, count) }
}

fn join_id(parent_id: &str, segment: &str) -> String {
    if parent_id.is_empty() { segment.to_string() } else { format!("{}.{}", parent_id, segment) }
}

fn token_len(item: &Item) -> usize {
    quote::quote! { #item }.to_string().len()
}

fn is_definition_item(item: &Item) -> bool {
    matches!(item, Item::Fn(_) | Item::Struct(_) | Item::Enum(_) | Item::Union(_) | Item::Trait(_) | Item::TraitAlias(_) | Item::Type(_) | Item::Mod(_) | Item::Impl(_))
}

fn group_kind(item: &Item) -> &'static str {
    if matches!(item, Item::Use(_)) { "ImportGroup" } else { "StatementGroup" }
}

fn item_type_name(item: &Item) -> &'static str {
    match item {
        Item::Const(_) => "ItemConst",
        Item::Enum(_) => "ItemEnum",
        Item::ExternCrate(_) => "ItemExternCrate",
        Item::Fn(_) => "ItemFn",
        Item::ForeignMod(_) => "ItemForeignMod",
        Item::Impl(_) => "ItemImpl",
        Item::Macro(_) => "ItemMacro",
        Item::Mod(_) => "ItemMod",
        Item::Static(_) => "ItemStatic",
        Item::Struct(_) => "ItemStruct",
        Item::Trait(_) => "ItemTrait",
        Item::TraitAlias(_) => "ItemTraitAlias",
        Item::Type(_) => "ItemType",
        Item::Union(_) => "ItemUnion",
        Item::Use(_) => "ItemUse",
        _ => "ItemUnsupported",
    }
}

fn item_name(item: &Item) -> Option<String> {
    match item {
        Item::Fn(f) => Some(f.sig.ident.to_string()),
        Item::Struct(s) => Some(s.ident.to_string()),
        Item::Enum(e) => Some(e.ident.to_string()),
        Item::Union(u) => Some(u.ident.to_string()),
        Item::Trait(t) => Some(t.ident.to_string()),
        Item::TraitAlias(t) => Some(t.ident.to_string()),
        Item::Type(t) => Some(t.ident.to_string()),
        Item::Mod(m) => Some(m.ident.to_string()),
        Item::Impl(imp) => Some(impl_name(imp)),
        _ => None,
    }
}

fn impl_name(imp: &syn::ItemImpl) -> String {
    let ty = &imp.self_ty;
    let ty_str = sanitize(&quote::quote! { #ty }.to_string().replace(' ', ""));
    match &imp.trait_ {
        Some((path, _)) => {
            let trait_str = sanitize(&quote::quote! { #path }.to_string().replace(' ', ""));
            format!("impl_{}_for_{}", trait_str, ty_str)
        }
        None => format!("impl_{}", ty_str),
    }
}

fn is_expandable(item: &Item) -> bool {
    match item {
        Item::Mod(m) => m.content.as_ref().map(|(_, items)| items.iter().any(is_definition_item)).unwrap_or(false),
        Item::Impl(imp) => imp.items.iter().any(|m| matches!(m, ImplItem::Fn(_))),
        Item::Trait(t) => t.items.iter().any(|m| matches!(m, TraitItem::Fn(_))),
        _ => false,
    }
}

fn walk_items(items: &[Item], mod_path: Vec<usize>, parent_type: &str, parent_id: &str, out: &mut Vec<AddressableNode>) {
    let mut used: HashMap<String, i32> = HashMap::new();
    let n = items.len();
    let mut i = 0;
    while i < n {
        let item = &items[i];
        if is_definition_item(item) {
            emit_definition(items, i, mod_path.clone(), parent_type, parent_id, &mut used, out);
            i += 1;
            continue;
        }
        let kind = group_kind(item);
        let start = i;
        let mut length = 0usize;
        while i < n {
            let cur = &items[i];
            if is_definition_item(cur) || group_kind(cur) != kind { break; }
            let piece = token_len(cur);
            if i > start && length + piece > SEGMENT_MAX_CHARS { break; }
            length += piece;
            i += 1;
        }
        add_item_group(items, kind, mod_path.clone(), start, i, parent_type, parent_id, &mut used, out);
    }
}

fn emit_definition(items: &[Item], idx: usize, mod_path: Vec<usize>, parent_type: &str, parent_id: &str, used: &mut HashMap<String, i32>, out: &mut Vec<AddressableNode>) {
    let item = &items[idx];
    let type_name = item_type_name(item);
    let name = item_name(item);
    let seg = segment(name.as_deref(), type_name, used);
    let id = join_id(parent_id, &seg);
    let expandable = is_expandable(item);
    out.push(AddressableNode {
        id: id.clone(),
        path: NodePath::single(mod_path.clone(), Owner::None, None, idx),
        node_type: type_name.to_string(),
        name: name.clone(),
        lineno: line(item.span()),
        end_lineno: end_line(item.span()),
        parent_type: parent_type.to_string(),
        expandable,
        is_definition: true,
    });
    match item {
        Item::Mod(m) => {
            if let Some((_, inner)) = &m.content {
                let mut child_mod_path = mod_path;
                child_mod_path.push(idx);
                walk_items(inner, child_mod_path, type_name, &id, out);
            }
        }
        Item::Impl(imp) => walk_impl_items(&imp.items, mod_path, idx, type_name, &id, out),
        Item::Trait(t) => walk_trait_items(&t.items, mod_path, idx, type_name, &id, out),
        _ => {}
    }
}

fn add_item_group(items: &[Item], kind: &str, mod_path: Vec<usize>, start: usize, end: usize, parent_type: &str, parent_id: &str, used: &mut HashMap<String, i32>, out: &mut Vec<AddressableNode>) {
    let slice = &items[start..end];
    let content = slice.iter().map(|m| quote::quote! { #m }.to_string()).collect::<Vec<_>>().join("\n");
    let seg = anonymous_segment(&content, used);
    let id = join_id(parent_id, &seg);
    out.push(AddressableNode {
        id,
        path: NodePath::range(mod_path, Owner::None, None, start, end),
        node_type: kind.to_string(),
        name: None,
        lineno: line(slice[0].span()),
        end_lineno: end_line(slice[slice.len() - 1].span()),
        parent_type: parent_type.to_string(),
        expandable: false,
        is_definition: false,
    });
}

fn walk_impl_items(items: &[ImplItem], mod_path: Vec<usize>, owner_idx: usize, parent_type: &str, parent_id: &str, out: &mut Vec<AddressableNode>) {
    let mut used: HashMap<String, i32> = HashMap::new();
    let n = items.len();
    let mut i = 0;
    while i < n {
        if let ImplItem::Fn(f) = &items[i] {
            let seg = segment(Some(&f.sig.ident.to_string()), "ImplItemFn", &mut used);
            let id = join_id(parent_id, &seg);
            out.push(AddressableNode {
                id,
                path: NodePath::single(mod_path.clone(), Owner::Impl, Some(owner_idx), i),
                node_type: "ImplItemFn".to_string(),
                name: Some(f.sig.ident.to_string()),
                lineno: line(f.span()),
                end_lineno: end_line(f.span()),
                parent_type: parent_type.to_string(),
                expandable: false,
                is_definition: true,
            });
            i += 1;
            continue;
        }
        let start = i;
        let mut length = 0usize;
        while i < n {
            if matches!(items[i], ImplItem::Fn(_)) { break; }
            let piece = quote::quote! { #(&items[i]) }.to_string().len();
            if i > start && length + piece > SEGMENT_MAX_CHARS { break; }
            length += piece;
            i += 1;
        }
        let slice = &items[start..i];
        let content = slice.iter().map(|m| quote::quote! { #m }.to_string()).collect::<Vec<_>>().join("\n");
        let seg = anonymous_segment(&content, &mut used);
        let id = join_id(parent_id, &seg);
        out.push(AddressableNode {
            id,
            path: NodePath::range(mod_path.clone(), Owner::Impl, Some(owner_idx), start, i),
            node_type: "StatementGroup".to_string(),
            name: None,
            lineno: line(slice[0].span()),
            end_lineno: end_line(slice[slice.len() - 1].span()),
            parent_type: parent_type.to_string(),
            expandable: false,
            is_definition: false,
        });
    }
}

fn walk_trait_items(items: &[TraitItem], mod_path: Vec<usize>, owner_idx: usize, parent_type: &str, parent_id: &str, out: &mut Vec<AddressableNode>) {
    let mut used: HashMap<String, i32> = HashMap::new();
    let n = items.len();
    let mut i = 0;
    while i < n {
        if let TraitItem::Fn(f) = &items[i] {
            let seg = segment(Some(&f.sig.ident.to_string()), "TraitItemFn", &mut used);
            let id = join_id(parent_id, &seg);
            out.push(AddressableNode {
                id,
                path: NodePath::single(mod_path.clone(), Owner::Trait, Some(owner_idx), i),
                node_type: "TraitItemFn".to_string(),
                name: Some(f.sig.ident.to_string()),
                lineno: line(f.span()),
                end_lineno: end_line(f.span()),
                parent_type: parent_type.to_string(),
                expandable: false,
                is_definition: true,
            });
            i += 1;
            continue;
        }
        let start = i;
        let mut length = 0usize;
        while i < n {
            if matches!(items[i], TraitItem::Fn(_)) { break; }
            let piece = quote::quote! { #(&items[i]) }.to_string().len();
            if i > start && length + piece > SEGMENT_MAX_CHARS { break; }
            length += piece;
            i += 1;
        }
        let slice = &items[start..i];
        let content = slice.iter().map(|m| quote::quote! { #m }.to_string()).collect::<Vec<_>>().join("\n");
        let seg = anonymous_segment(&content, &mut used);
        let id = join_id(parent_id, &seg);
        out.push(AddressableNode {
            id,
            path: NodePath::range(mod_path.clone(), Owner::Trait, Some(owner_idx), start, i),
            node_type: "StatementGroup".to_string(),
            name: None,
            lineno: line(slice[0].span()),
            end_lineno: end_line(slice[slice.len() - 1].span()),
            parent_type: parent_type.to_string(),
            expandable: false,
            is_definition: false,
        });
    }
}
