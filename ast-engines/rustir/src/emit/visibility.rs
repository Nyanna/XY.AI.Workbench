use proc_macro2::Span;
use syn::spanned::Spanned;
use syn::{Field, File, ImplItem, Item, ItemTrait, Visibility};

use crate::emit::edits::{Edit, Source};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exposure {
    Keep,
    Crate,
    Public,
}

pub fn is_public(file: &File) -> bool {
    file.items
        .first()
        .and_then(own_visibility)
        .is_some_and(|vis| matches!(vis, Visibility::Public(_)))
}

fn own_visibility(item: &Item) -> Option<&Visibility> {
    match item {
        Item::Struct(i) => Some(&i.vis),
        Item::Enum(i) => Some(&i.vis),
        Item::Union(i) => Some(&i.vis),
        Item::Trait(i) => Some(&i.vis),
        Item::Type(i) => Some(&i.vis),
        Item::Fn(i) => Some(&i.vis),
        Item::Const(i) => Some(&i.vis),
        Item::Static(i) => Some(&i.vis),
        _ => None,
    }
}

pub fn visibility_edits(file: &File, source: &Source, exposure: Exposure) -> Vec<Edit> {
    let mut edits = Vec::new();
    if exposure == Exposure::Keep {
        return edits;
    }
    for (index, item) in file.items.iter().enumerate() {
        let own = if index == 0 { exposure } else { Exposure::Keep };
        item_edits(item, source, own, &mut edits);
    }
    edits
}

fn item_edits(item: &Item, source: &Source, own: Exposure, out: &mut Vec<Edit>) {
    match item {
        Item::Struct(i) => {
            claim(&i.vis, i.struct_token.span, source, own, out);
            field_edits(i.fields.iter(), source, out);
        }
        Item::Enum(i) => claim(&i.vis, i.enum_token.span, source, own, out),
        Item::Union(i) => {
            claim(&i.vis, i.union_token.span, source, own, out);
            field_edits(i.fields.named.iter(), source, out);
        }
        Item::Trait(i) => claim(&i.vis, trait_anchor(i), source, own, out),
        Item::Type(i) => claim(&i.vis, i.type_token.span, source, own, out),
        Item::Fn(i) => claim(&i.vis, i.sig.span(), source, own, out),
        Item::Const(i) => claim(&i.vis, i.const_token.span, source, own, out),
        Item::Static(i) => claim(&i.vis, i.static_token.span, source, own, out),
        Item::Impl(i) if i.trait_.is_none() => {
            for member in &i.items {
                member_edits(member, source, out);
            }
        }
        _ => {}
    }
}

fn trait_anchor(item: &ItemTrait) -> Span {
    item.unsafety
        .as_ref()
        .map(|token| token.span)
        .or_else(|| item.auto_token.as_ref().map(|token| token.span))
        .unwrap_or(item.trait_token.span)
}

fn field_edits<'a>(fields: impl Iterator<Item = &'a Field>, source: &Source, out: &mut Vec<Edit>) {
    for field in fields {
        let anchor = field
            .ident
            .as_ref()
            .map_or_else(|| field.ty.span(), |ident| ident.span());
        claim(&field.vis, anchor, source, Exposure::Crate, out);
    }
}

fn member_edits(member: &ImplItem, source: &Source, out: &mut Vec<Edit>) {
    match member {
        ImplItem::Fn(f) => {
            let anchor = f.defaultness.as_ref().map_or_else(|| f.sig.span(), |d| d.span);
            claim(&f.vis, anchor, source, Exposure::Crate, out);
        }
        ImplItem::Const(c) => {
            let anchor = c.defaultness.as_ref().map_or(c.const_token.span, |d| d.span);
            claim(&c.vis, anchor, source, Exposure::Crate, out);
        }
        ImplItem::Type(t) => {
            let anchor = t.defaultness.as_ref().map_or(t.type_token.span, |d| d.span);
            claim(&t.vis, anchor, source, Exposure::Crate, out);
        }
        _ => {}
    }
}

fn claim(vis: &Visibility, anchor: Span, source: &Source, wanted: Exposure, out: &mut Vec<Edit>) {
    let keyword = match wanted {
        Exposure::Keep => return,
        Exposure::Crate => "pub(crate)",
        Exposure::Public => "pub",
    };
    match vis {
        Visibility::Public(_) => {}
        Visibility::Inherited => {
            let at = source.start_of(anchor);
            out.push(Edit {
                start: at,
                end: at,
                text: format!("{keyword} "),
            });
        }
        Visibility::Restricted(restricted) => {
            let crate_wide = restricted.in_token.is_none() && restricted.path.is_ident("crate");
            if crate_wide && wanted == Exposure::Crate {
                return;
            }
            out.push(Edit {
                start: source.start_of(restricted.pub_token.span),
                end: source.end_of(restricted.paren_token.span.close()),
                text: keyword.to_string(),
            });
        }
    }
}

#[cfg(test)]
#[path = "test/visibility.rs"]
mod tests;
