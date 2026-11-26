use std::path::{Path, PathBuf};

use syn::{Attribute, Item, ItemImpl, Type};

use crate::ir::{Object, ObjectKind};
use crate::parser::dependencies::item_names;
use crate::parser::source::item_text;

#[derive(Debug)]
pub struct Located {
    pub object: Object,
    pub file: PathBuf,
}

#[derive(Debug)]
pub struct ImplBlock {
    pub self_name: Option<String>,
    pub trait_name: Option<String>,
    pub names: Vec<String>,
    pub body: String,
}

#[derive(Debug, Default)]
pub struct Collected {
    pub objects: Vec<Located>,
    pub impls: Vec<ImplBlock>,
}

pub fn collect_items(file: &Path, source: &str, items: &[Item], out: &mut Collected) {
    for item in items {
        if is_test_only(attributes(item)) {
            continue;
        }
        match item {
            Item::Mod(module) => {
                if let Some((_, inner)) = &module.content {
                    collect_items(file, source, inner, out);
                }
            }
            Item::Impl(block) => out.impls.push(ImplBlock {
                self_name: impl_target(block),
                trait_name: impl_trait(block),
                names: item_names(item).into_iter().collect(),
                body: item_text(source, item),
            }),
            other => {
                if let Some((name, kind)) = describe(other) {
                    let mut names = item_names(other);
                    names.remove(&name);
                    out.objects.push(Located {
                        object: Object {
                            name,
                            kind,
                            body: item_text(source, other),
                            dependencies: names.into_iter().collect(),
                        },
                        file: file.to_path_buf(),
                    });
                }
            }
        }
    }
}

fn describe(item: &Item) -> Option<(String, ObjectKind)> {
    let (ident, kind) = match item {
        Item::Struct(i) => (&i.ident, ObjectKind::Struct),
        Item::Enum(i) => (&i.ident, ObjectKind::Enum),
        Item::Union(i) => (&i.ident, ObjectKind::Union),
        Item::Trait(i) => (&i.ident, ObjectKind::Trait),
        Item::Type(i) => (&i.ident, ObjectKind::TypeAlias),
        Item::Fn(i) => (&i.sig.ident, ObjectKind::Function),
        Item::Const(i) => (&i.ident, ObjectKind::Const),
        Item::Static(i) => (&i.ident, ObjectKind::Static),
        Item::Macro(i) => (i.ident.as_ref()?, ObjectKind::Macro),
        _ => return None,
    };
    let name = ident.to_string();
    (name != "_").then_some((name, kind))
}

fn attributes(item: &Item) -> &[Attribute] {
    match item {
        Item::Struct(i) => &i.attrs,
        Item::Enum(i) => &i.attrs,
        Item::Union(i) => &i.attrs,
        Item::Trait(i) => &i.attrs,
        Item::Type(i) => &i.attrs,
        Item::Fn(i) => &i.attrs,
        Item::Const(i) => &i.attrs,
        Item::Static(i) => &i.attrs,
        Item::Macro(i) => &i.attrs,
        Item::Mod(i) => &i.attrs,
        Item::Impl(i) => &i.attrs,
        _ => &[],
    }
}

fn is_test_only(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|attr| {
        if attr.path().is_ident("test") {
            return true;
        }
        if !attr.path().is_ident("cfg") {
            return false;
        }
        let tokens = attr
            .meta
            .require_list()
            .map(|list| list.tokens.to_string())
            .unwrap_or_default();
        let words: Vec<&str> = tokens
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .collect();
        words.contains(&"test") && !words.contains(&"not")
    })
}

fn impl_target(block: &ItemImpl) -> Option<String> {
    let name = type_name(&block.self_ty)?;
    let generic = block.generics.type_params().any(|param| param.ident == name);
    (!generic).then_some(name)
}

fn impl_trait(block: &ItemImpl) -> Option<String> {
    let (_, path, _) = block.trait_.as_ref()?;
    path.segments.last().map(|segment| segment.ident.to_string())
}

fn type_name(ty: &Type) -> Option<String> {
    match ty {
        Type::Path(path) => path.path.segments.last().map(|s| s.ident.to_string()),
        Type::Reference(reference) => type_name(&reference.elem),
        _ => None,
    }
}

#[cfg(test)]
#[path = "test/collect.rs"]
mod tests;
