//! Structural addressing into a `syn::File`, replacing JavaParser's live `Node`
//! references (which Rust's ownership model forbids holding across a request):
//! a [`NodePath`] names a `[start, end)` range inside some container, reached by
//! descending through zero or more inline `mod { ... }` items, optionally landing
//! inside one `impl`/`trait` block's own item list. Re-resolved fresh on every
//! request against the live tree, exactly like the Java engine re-resolves ids via
//! `NodeLocator.locateAll` each call.
use syn::{Attribute, File, Item, ImplItem, TraitItem};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Owner {
    /// The range lives directly in an items list (`File.items` or a `mod`'s items).
    None,
    /// The range lives in the `items` of the `ItemImpl` at `owner_idx` of that items list.
    Impl,
    /// The range lives in the `items` of the `ItemTrait` at `owner_idx` of that items list.
    Trait,
    /// The range lives in the file-level inner `attrs` (e.g. `//!` module doc comments);
    /// `mod_path`/`owner_idx` are unused and must be empty/`None`.
    FileAttrs,
}
#[derive(Debug, Clone)]
pub struct NodePath {
    /// Indices of `ItemMod` to descend through (in order) to reach the items list this path targets.
    pub mod_path: Vec<usize>,
    pub owner: Owner,
    pub owner_idx: Option<usize>,
    pub start: usize,
    pub end: usize,
}
impl NodePath {
    pub fn single(
        mod_path: Vec<usize>,
        owner: Owner,
        owner_idx: Option<usize>,
        index: usize,
    ) -> Self {
        Self {
            mod_path,
            owner,
            owner_idx,
            start: index,
            end: index + 1,
        }
    }
    pub fn range(
        mod_path: Vec<usize>,
        owner: Owner,
        owner_idx: Option<usize>,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            mod_path,
            owner,
            owner_idx,
            start,
            end,
        }
    }
}
fn items_vec<'a>(file: &'a File, mod_path: &[usize]) -> Option<&'a Vec<Item>> {
    let mut cur = &file.items;
    for &idx in mod_path {
        match cur.get(idx)? {
            Item::Mod(m) => cur = &m.content.as_ref()?.1,
            _ => return None,
        }
    }
    Some(cur)
}
fn items_vec_mut<'a>(
    file: &'a mut File,
    mod_path: &[usize],
) -> Option<&'a mut Vec<Item>> {
    let mut cur = &mut file.items;
    for &idx in mod_path {
        match cur.get_mut(idx)? {
            Item::Mod(m) => cur = &mut m.content.as_mut()?.1,
            _ => return None,
        }
    }
    Some(cur)
}
/// What a [`NodePath`] ultimately addresses: either a slice of `Item`s, or a slice
/// of `ImplItem`s/`TraitItem`s one level inside one `impl`/`trait` of that slice.
pub enum Target<'a> {
    Items(&'a mut Vec<Item>),
    Impl(&'a mut Vec<ImplItem>),
    Trait(&'a mut Vec<TraitItem>),
    Attrs(&'a mut Vec<Attribute>),
}
pub fn resolve_mut<'a>(file: &'a mut File, path: &NodePath) -> Option<Target<'a>> {
    if path.owner == Owner::FileAttrs {
        return Some(Target::Attrs(&mut file.attrs));
    }
    let items = items_vec_mut(file, &path.mod_path)?;
    match path.owner {
        Owner::None => Some(Target::Items(items)),
        Owner::Impl => {
            match items.get_mut(path.owner_idx?)? {
                Item::Impl(imp) => Some(Target::Impl(&mut imp.items)),
                _ => None,
            }
        }
        Owner::Trait => {
            match items.get_mut(path.owner_idx?)? {
                Item::Trait(tr) => Some(Target::Trait(&mut tr.items)),
                _ => None,
            }
        }
    }
}
/// Read-only counterpart of [`Target`]: what a [`NodePath`] ultimately addresses.
pub enum TargetRef<'a> {
    Items(&'a [Item]),
    Impl(&'a [ImplItem]),
    Trait(&'a [TraitItem]),
    Attrs(&'a [Attribute]),
}
pub fn resolve<'a>(file: &'a File, path: &NodePath) -> Option<TargetRef<'a>> {
    if path.owner == Owner::FileAttrs {
        return Some(TargetRef::Attrs(&file.attrs));
    }
    let items = items_vec(file, &path.mod_path)?;
    match path.owner {
        Owner::None => Some(TargetRef::Items(items)),
        Owner::Impl => {
            match items.get(path.owner_idx?)? {
                Item::Impl(imp) => Some(TargetRef::Impl(&imp.items)),
                _ => None,
            }
        }
        Owner::Trait => {
            match items.get(path.owner_idx?)? {
                Item::Trait(tr) => Some(TargetRef::Trait(&tr.items)),
                _ => None,
            }
        }
    }
}
