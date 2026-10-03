//! `syn`/`prettyplease` mechanics: parse/print, syntax validation, fragment parsing
//! keyed by the target node's container kind, and the four mutation primitives.
//! Every write re-renders through `prettyplease::unparse`, so every write normalises
//! formatting -- the Rust analogue of Java's `JavaAstEngine`.
use syn::{Attribute, Expr, ImplItem, Item, Lit, Meta, TraitItem};
use crate::engine::addressable_node::AddressableNode;
use crate::engine::ast_engine_exception::{AstEngineException, AstResult, Kind};
use crate::engine::node_path::{resolve, resolve_mut, NodePath, Target, TargetRef};
pub fn empty_file() -> syn::File {
    syn::File {
        shebang: None,
        frontmatter: None,
        attrs: Vec::new(),
        items: Vec::new(),
    }
}
pub fn is_empty_file(file: &syn::File) -> bool {
    file.items.is_empty() && file.attrs.is_empty() && file.shebang.is_none()
}
pub fn parse_file(source: &str) -> AstResult<syn::File> {
    syn::parse_file(source)
        .map_err(|e| AstEngineException::new(
            Kind::Syntax,
            format!("invalid source: {}", e),
        ))
}
/// Returns an error message if `source` is malformed, else `None`.
pub fn validate(source: &str) -> Option<String> {
    syn::parse_file(source).err().map(|e| e.to_string())
}
pub fn print_file(file: &syn::File) -> String {
    prettyplease::unparse(file)
}
fn print_items(items: &[Item]) -> String {
    let file = syn::File {
        shebang: None,
        frontmatter: None,
        attrs: Vec::new(),
        items: items.to_vec(),
    };
    prettyplease::unparse(&file)
}
/// Strips the synthetic `impl __Wrapper__ { ... }` / `trait __Wrapper__ { ... }` shell
/// prettyplease prints around a wrapped fragment, and dedents the body by one level.
fn unwrap_block(printed: &str) -> String {
    let mut lines: Vec<&str> = printed.lines().collect();
    if lines.len() >= 2 {
        lines.remove(0);
        lines.pop();
    }
    lines
        .iter()
        .map(|l| l.strip_prefix("    ").unwrap_or(l))
        .collect::<Vec<_>>()
        .join("\n")
}
fn print_impl_items(items: &[ImplItem]) -> String {
    let wrapper = syn::ItemImpl {
        attrs: Vec::new(),
        modifiers: Default::default(),
        unsafety: None,
        impl_token: Default::default(),
        generics: Default::default(),
        trait_: None,
        self_ty: Box::new(syn::parse_str::<syn::Type>("__Wrapper__").unwrap()),
        brace_token: Default::default(),
        items: items.to_vec(),
    };
    let file = syn::File {
        shebang: None,
        frontmatter: None,
        attrs: Vec::new(),
        items: vec![Item::Impl(wrapper)],
    };
    unwrap_block(&prettyplease::unparse(&file))
}
fn print_trait_items(items: &[TraitItem]) -> String {
    let wrapper = syn::ItemTrait {
        attrs: Vec::new(),
        vis: syn::Visibility::Inherited,
        modifiers: Default::default(),
        unsafety: None,
        trait_token: Default::default(),
        ident: syn::Ident::new("__Wrapper__", proc_macro2::Span::call_site()),
        generics: Default::default(),
        colon_token: None,
        supertraits: Default::default(),
        brace_token: Default::default(),
        items: items.to_vec(),
    };
    let file = syn::File {
        shebang: None,
        frontmatter: None,
        attrs: Vec::new(),
        items: vec![Item::Trait(wrapper)],
    };
    unwrap_block(&prettyplease::unparse(&file))
}
/// Dispatches on the path's container kind -- the Rust analogue of Java's
/// `print(Object)` dispatch between a single `Node` and a grouped `NodeGroup`.
pub fn print_node(file: &syn::File, node: &AddressableNode) -> String {
    match resolve(file, &node.path) {
        Some(TargetRef::Items(items)) => {
            print_items(&items[node.path.start..node.path.end])
        }
        Some(TargetRef::Impl(items)) => {
            print_impl_items(&items[node.path.start..node.path.end])
        }
        Some(TargetRef::Trait(items)) => {
            print_trait_items(&items[node.path.start..node.path.end])
        }
        None => String::new(),
    }
}
fn first_line(printed: &str) -> String {
    for line in printed.lines() {
        let t = line.trim();
        if !t.is_empty() {
            return t.to_string();
        }
    }
    String::new()
}
fn header(printed: &str) -> String {
    match printed.find('{') {
        Some(idx) => printed[..=idx].to_string(),
        None => first_line(printed),
    }
}
fn truncate(text: &str, limit: usize) -> String {
    let flat = text.replace('\n', " ").replace('\r', " ");
    let flat = flat.trim();
    if flat.chars().count() <= limit {
        return flat.to_string();
    }
    let mut s: String = flat.chars().take(limit.saturating_sub(1)).collect();
    s.push('\u{2026}');
    s
}
/// Dispatches to a one-line header rendering for a definition, or the group's own
/// first printed line for a grouped segment -- the Rust analogue of Java's `signature`.
pub fn signature(file: &syn::File, node: &AddressableNode, limit: usize) -> String {
    let printed = print_node(file, node);
    let text = if node.is_definition { header(&printed) } else { first_line(&printed) };
    truncate(&text, limit)
}
fn attrs_of(item: &Item) -> &[Attribute] {
    match item {
        Item::Const(i) => &i.attrs,
        Item::Enum(i) => &i.attrs,
        Item::ExternCrate(i) => &i.attrs,
        Item::Fn(i) => &i.attrs,
        Item::ForeignMod(i) => &i.attrs,
        Item::Impl(i) => &i.attrs,
        Item::Macro(i) => &i.attrs,
        Item::Mod(i) => &i.attrs,
        Item::Static(i) => &i.attrs,
        Item::Struct(i) => &i.attrs,
        Item::Trait(i) => &i.attrs,
        Item::TraitAlias(i) => &i.attrs,
        Item::Type(i) => &i.attrs,
        Item::Union(i) => &i.attrs,
        Item::Use(i) => &i.attrs,
        _ => &[],
    }
}
fn impl_item_attrs(item: &ImplItem) -> &[Attribute] {
    match item {
        ImplItem::Const(i) => &i.attrs,
        ImplItem::Fn(i) => &i.attrs,
        ImplItem::Type(i) => &i.attrs,
        ImplItem::Macro(i) => &i.attrs,
        _ => &[],
    }
}
fn trait_item_attrs(item: &TraitItem) -> &[Attribute] {
    match item {
        TraitItem::Const(i) => &i.attrs,
        TraitItem::Fn(i) => &i.attrs,
        TraitItem::Type(i) => &i.attrs,
        TraitItem::Macro(i) => &i.attrs,
        _ => &[],
    }
}
/// First non-blank `///`/`#[doc = ...]` line, if any -- the Rust analogue of Java's
/// `docstring`, which reads the first non-blank Javadoc paragraph line.
fn docstring_from_attrs(attrs: &[Attribute], limit: usize) -> Option<String> {
    for attr in attrs {
        if !attr.path().is_ident("doc") {
            continue;
        }
        let Meta::NameValue(nv) = &attr.meta else { continue };
        let Expr::Lit(lit) = &nv.value else { continue };
        let Lit::Str(s) = &lit.lit else { continue };
        let line = s.value().trim().to_string();
        if !line.is_empty() {
            return Some(truncate(&line, limit));
        }
    }
    None
}
/// A grouped segment never carries a doc comment of its own, same as Java's `NodeGroup`.
pub fn docstring(file: &syn::File, node: &AddressableNode) -> Option<String> {
    if !node.is_definition {
        return None;
    }
    match resolve(file, &node.path) {
        Some(TargetRef::Items(items)) => {
            docstring_from_attrs(attrs_of(&items[node.path.start]), 80)
        }
        Some(TargetRef::Impl(items)) => {
            docstring_from_attrs(impl_item_attrs(&items[node.path.start]), 80)
        }
        Some(TargetRef::Trait(items)) => {
            docstring_from_attrs(trait_item_attrs(&items[node.path.start]), 80)
        }
        None => None,
    }
}
fn syntax_err(what: &str, e: syn::Error) -> AstEngineException {
    AstEngineException::new(Kind::Syntax, format!("invalid {}: {}", what, e))
}
pub fn parse_item(code: &str) -> AstResult<Item> {
    syn::parse_str::<Item>(code.trim()).map_err(|e| syntax_err("item", e))
}
/// Parses `code` as one or more top-level items, for a `File`-level items container.
pub fn parse_items(code: &str) -> AstResult<Vec<Item>> {
    let file = syn::parse_file(code).map_err(|e| syntax_err("items", e))?;
    if file.items.is_empty() {
        return Err(
            AstEngineException::new(Kind::Syntax, format!("no items found: {}", code)),
        );
    }
    Ok(file.items)
}
/// Parses `code` as one or more impl members, for an `impl` block's items container.
pub fn parse_impl_fragment(code: &str) -> AstResult<Vec<ImplItem>> {
    let wrapped = format!("impl __Wrapper__ {{ {} }}", code);
    let item: syn::ItemImpl = syn::parse_str(&wrapped)
        .map_err(|e| syntax_err("impl member", e))?;
    if item.items.is_empty() {
        return Err(
            AstEngineException::new(
                Kind::Syntax,
                format!("no impl members found: {}", code),
            ),
        );
    }
    Ok(item.items)
}
/// Parses `code` as one or more trait members, for a `trait` block's items container.
pub fn parse_trait_fragment(code: &str) -> AstResult<Vec<TraitItem>> {
    let wrapped = format!("trait __Wrapper__ {{ {} }}", code);
    let item: syn::ItemTrait = syn::parse_str(&wrapped)
        .map_err(|e| syntax_err("trait member", e))?;
    if item.items.is_empty() {
        return Err(
            AstEngineException::new(
                Kind::Syntax,
                format!("no trait members found: {}", code),
            ),
        );
    }
    Ok(item.items)
}
/// Appends `code` at the file's top level: a single top-level item, or (if the
/// file is still empty) the whole file. Mirrors Java's `JavaAstEngine.append`.
pub fn append(file: syn::File, code: &str) -> AstResult<(syn::File, i64)> {
    if is_empty_file(&file) {
        return Ok((parse_file(code)?, 1));
    }
    let mut file = file;
    let item = parse_item(code)?;
    file.items.push(item);
    Ok((file, 1))
}
fn not_found(id: &str) -> AstEngineException {
    AstEngineException::new(Kind::Conflict, format!("node no longer present: {}", id))
}
pub fn replace(
    file: &mut syn::File,
    node: &AddressableNode,
    code: &str,
) -> AstResult<()> {
    let path: &NodePath = &node.path;
    match resolve_mut(file, path).ok_or_else(|| not_found(&node.id))? {
        Target::Items(items) => {
            let fragments = parse_items(code)?;
            items.splice(path.start..path.end, fragments);
        }
        Target::Impl(items) => {
            let fragments = parse_impl_fragment(code)?;
            items.splice(path.start..path.end, fragments);
        }
        Target::Trait(items) => {
            let fragments = parse_trait_fragment(code)?;
            items.splice(path.start..path.end, fragments);
        }
    }
    Ok(())
}
pub fn insert(
    file: &mut syn::File,
    node: &AddressableNode,
    code: &str,
    position: &str,
) -> AstResult<i64> {
    let path: &NodePath = &node.path;
    let idx = if position == "after" { path.end } else { path.start };
    let count = match resolve_mut(file, path).ok_or_else(|| not_found(&node.id))? {
        Target::Items(items) => {
            let fragments = parse_items(code)?;
            let n = fragments.len();
            items.splice(idx..idx, fragments);
            n
        }
        Target::Impl(items) => {
            let fragments = parse_impl_fragment(code)?;
            let n = fragments.len();
            items.splice(idx..idx, fragments);
            n
        }
        Target::Trait(items) => {
            let fragments = parse_trait_fragment(code)?;
            let n = fragments.len();
            items.splice(idx..idx, fragments);
            n
        }
    };
    Ok(count as i64)
}
pub fn delete(file: &mut syn::File, node: &AddressableNode) -> AstResult<()> {
    let path: &NodePath = &node.path;
    match resolve_mut(file, path).ok_or_else(|| not_found(&node.id))? {
        Target::Items(items) => {
            items.drain(path.start..path.end);
        }
        Target::Impl(items) => {
            items.drain(path.start..path.end);
        }
        Target::Trait(items) => {
            items.drain(path.start..path.end);
        }
    }
    Ok(())
}
