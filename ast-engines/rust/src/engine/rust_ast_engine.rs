//! `syn`/`prettyplease` mechanics: parse/print, syntax validation, fragment parsing
//! keyed by the target node's container kind, and the four mutation primitives.
//! Every write re-renders through `prettyplease::unparse`, so every write normalises
//! formatting -- the Rust analogue of Java's `JavaAstEngine`.
use syn::{Attribute, Expr, ImplItem, Item, Lit, Meta, TraitItem};
use crate::engine::addressable_node::AddressableNode;
use crate::engine::ast_engine_exception::{AstEngineException, AstResult, Kind};
use crate::engine::node_path::{resolve, resolve_mut, NodePath, Owner, Target, TargetRef};
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
fn print_attrs(attrs: &[Attribute]) -> String {
    let file = syn::File {
        shebang: None,
        frontmatter: None,
        attrs: attrs.to_vec(),
        items: Vec::new(),
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
        Some(TargetRef::Attrs(attrs)) => {
            print_attrs(&attrs[node.path.start..node.path.end])
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
    let mut rest = printed;
    loop {
        let trimmed = rest.trim_start();
        let is_leading_meta = trimmed.starts_with("///") || trimmed.starts_with("//!")
            || trimmed.starts_with("#[") || trimmed.starts_with("#![");
        if !is_leading_meta {
            rest = trimmed;
            break;
        }
        match trimmed.find('\n') {
            Some(idx) => rest = &trimmed[idx + 1..],
            None => return String::new(),
        }
    }
    let mut depth = 0usize;
    for (idx, ch) in rest.char_indices() {
        match ch {
            '(' | '[' => depth += 1,
            ')' | ']' => depth = depth.saturating_sub(1),
            '{' | ';' if depth == 0 => return rest[..=idx].to_string(),
            _ => {}
        }
    }
    first_line(rest)
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
        Some(TargetRef::Attrs(_)) => None,
        None => None,
    }
}
/// Parses `code` as optional leading inner attributes (`//!`/`#![..]`) followed
/// by zero or more top-level items. The file-level `attrs` container only holds
/// module doc comments/attributes, but callers (e.g. an agent regenerating a
/// module's header) routinely hand back the doc comment together with the
/// `use`/`const`/`fn` items that originally followed it -- that combined text is
/// perfectly valid Rust, just not valid as an attributes-only fragment. Rather
/// than rejecting it, the engine recognizes the split point itself and routes
/// each part to its proper container (see callers in [`replace`]/[`insert`]).
fn parse_inner_attrs_and_items(code: &str) -> AstResult<(Vec<Attribute>, Vec<Item>)> {
    let parser = |input: syn::parse::ParseStream| {
        let mut attrs = Vec::new();
        loop {
            let before = input.cursor();
            attrs.extend(input.call(Attribute::parse_inner)?);
            if input.cursor() == before {
                break;
            }
        }
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok((attrs, items))
    };
    let (attrs, items) = syn::parse::Parser::parse_str(parser, code)
        .map_err(|e| syntax_err("inner attribute/item", e))?;
    if attrs.is_empty() && items.is_empty() {
        return Err(
            AstEngineException::new(
                Kind::Syntax,
                format!("no attributes or items found: {}", code),
            ),
        );
    }
    Ok((attrs, items))
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
    if path.owner == Owner::FileAttrs {
        let (attr_fragments, item_fragments) = parse_inner_attrs_and_items(code)?;
        file.attrs.splice(path.start..path.end, attr_fragments);
        file.items.splice(0..0, item_fragments);
        return Ok(());
    }
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
        Target::Attrs(_) => unreachable!("Owner::FileAttrs handled above"),
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
    if path.owner == Owner::FileAttrs {
        let (attr_fragments, item_fragments) = parse_inner_attrs_and_items(code)?;
        let n = attr_fragments.len() + item_fragments.len();
        file.attrs.splice(idx..idx, attr_fragments);
        file.items.splice(0..0, item_fragments);
        return Ok(n as i64);
    }
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
        Target::Attrs(_) => unreachable!("Owner::FileAttrs handled above"),
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
        Target::Attrs(attrs) => {
            attrs.drain(path.start..path.end);
        }
    }
    Ok(())
}
#[cfg(test)]
mod parse_inner_attrs_tests {
    use super::parse_inner_attrs_and_items;
    #[test]
    fn parses_doc_comments_only() {
        let code = "//! line one\n//! line two\n";
        let (attrs, items) = parse_inner_attrs_and_items(code).expect("should parse");
        assert_eq!(attrs.len(), 2);
        assert_eq!(items.len(), 0);
    }
    /// Regression test: a source blob where inner doc comments are followed by
    /// non-attribute code (e.g. an agent re-sends `use`/`fn` items alongside a
    /// module doc replacement) must no longer hang forever in the parser loop,
    /// and the engine should route the trailing code to `items` instead of
    /// erroring, since the combined text is perfectly valid Rust.
    #[test]
    fn splits_trailing_code_into_items_without_hanging() {
        let code = "//! module doc\nuse tracing::Span;\npub fn x() {}\n";
        let (attrs, items) = parse_inner_attrs_and_items(code).expect("should parse");
        assert_eq!(attrs.len(), 1);
        assert_eq!(items.len(), 2);
    }
}
#[cfg(test)]
mod replace_file_attrs_tests {
    use super::*;
    use crate::engine::addressable_node::AddressableNode;
    use crate::engine::node_path::{NodePath, Owner};
    /// End-to-end regression test for the original hang: replacing the file-level
    /// `ModuleDoc` (`Attrs`) node with a block that mixes `//!` doc comments and
    /// real items must succeed -- not hang, not error -- splitting the result
    /// between `file.attrs` and `file.items`.
    #[test]
    fn replace_module_doc_with_mixed_doc_and_items() {
        let mut file = empty_file();
        let node = AddressableNode {
            id: "ModuleDoc".to_string(),
            path: NodePath::range(Vec::new(), Owner::FileAttrs, None, 0, 0),
            node_type: "Attrs".to_string(),
            name: None,
            lineno: 1,
            end_lineno: 1,
            parent_type: "File".to_string(),
            expandable: false,
            is_definition: true,
        };
        let code = "//! Tracing conventions and init.\n//!\n//! Second line.\n\nuse tracing::Span;\n\npub const FIELD_TASK_ID: &str = \"task_id\";\n\npub fn init_tracing() {}\n";
        replace(&mut file, &node, code)
            .expect("replace should succeed, not hang or error");
        assert_eq!(file.attrs.len(), 3, "each //! line is its own attr in file.attrs");
        assert_eq!(file.items.len(), 3, "use + const + fn go to file.items");
    }
}
