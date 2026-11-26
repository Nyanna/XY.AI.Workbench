use std::collections::{BTreeMap, BTreeSet};
use rustc_driver::{Callbacks, Compilation};
use rustc_hir::def::{CtorOf, DefKind, Res};
use rustc_hir::def_id::{DefId, LOCAL_CRATE};
use rustc_hir::intravisit::{self, Visitor};
use rustc_hir::{BodyId, Expr, HirId, ItemKind, Pat, PathSegment};
use rustc_interface::interface::Compiler;
use rustc_middle::hir::nested_filter;
use rustc_middle::ty::{self, Ty, TyCtxt, TypeckResults};
use crate::render::{render_aliases, render_edges};
pub struct Collect {
    prefix: String,
    out: String,
}
impl Collect {
    pub fn new(prefix: String, out: String) -> Self {
        Self { prefix, out }
    }
}
impl Callbacks for Collect {
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        let (edges, aliases) = gather(tcx, &self.prefix);
        let text = render_edges(&edges) + &render_aliases(&aliases);
        if let Err(error) = std::fs::write(&self.out, text) {
            eprintln!("rustir-driver: cannot write {}: {error}", self.out);
        }
        Compilation::Continue
    }
}
struct References<'tcx> {
    tcx: TyCtxt<'tcx>,
    found: Vec<DefId>,
    spelled: Vec<(String, DefId)>,
    results: Option<&'tcx TypeckResults<'tcx>>,
}
impl<'tcx> References<'tcx> {
    fn push(&mut self, id: DefId) {
        if !self.found.contains(&id) {
            self.found.push(id);
        }
    }
    fn record_resolved(&mut self, hir_id: HirId) {
        let resolved = self
            .results
            .and_then(|results| results.type_dependent_def_id(hir_id));
        if let Some(id) = resolved {
            self.push(id);
        }
    }
}
impl<'tcx> Visitor<'tcx> for References<'tcx> {
    type NestedFilter = nested_filter::All;
    fn maybe_tcx(&mut self) -> TyCtxt<'tcx> {
        self.tcx
    }
    fn visit_path(&mut self, path: &rustc_hir::Path<'tcx>, _id: HirId) {
        if let Res::Def(_, id) = path.res {
            self.push(id);
        }
        intravisit::walk_path(self, path);
    }
    fn visit_path_segment(&mut self, segment: &'tcx PathSegment<'tcx>) {
        if let Res::Def(_, id) = segment.res {
            self.spelled.push((segment.ident.name.to_string(), id));
        }
        intravisit::walk_path_segment(self, segment);
    }
    fn visit_nested_body(&mut self, id: BodyId) {
        let tcx = self.tcx;
        let owner = tcx.hir_body_owner_def_id(id);
        let previous = self.results.replace(tcx.typeck(owner));
        let body = tcx.hir_body(id);
        self.visit_body(body);
        self.results = previous;
    }
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        self.record_resolved(expr.hir_id);
        intravisit::walk_expr(self, expr);
    }
    fn visit_pat(&mut self, pat: &'tcx Pat<'tcx>) {
        self.record_resolved(pat.hir_id);
        intravisit::walk_pat(self, pat);
    }
}
type Edges = BTreeMap<String, BTreeSet<String>>;
type Aliases = BTreeMap<String, BTreeSet<(String, String)>>;
fn gather(tcx: TyCtxt<'_>, prefix: &str) -> (Edges, Aliases) {
    let mut edges = Edges::new();
    let mut aliases = Aliases::new();
    for item_id in tcx.hir_free_items() {
        let item = tcx.hir_item(item_id);
        let def_id = item.owner_id.def_id.to_def_id();
        if !in_prefix(&local_path(tcx, def_id), prefix) {
            continue;
        }
        let mut references = References {
            tcx,
            found: Vec::new(),
            spelled: Vec::new(),
            results: None,
        };
        references.visit_item(item);
        let owner = match item.kind {
            ItemKind::Impl(_) => impl_owner(tcx, def_id, &references.found),
            _ if is_object(tcx, def_id) => Some(tcx.item_name(def_id).to_string()),
            _ => None,
        };
        let Some(owner) = owner else { continue };
        let entry = edges.entry(owner.clone()).or_default();
        for target in &references.found {
            if let Some(name) = dependency_name(tcx, *target, prefix) {
                if name != owner {
                    entry.insert(name);
                }
            }
        }
        for (spelled, id) in &references.spelled {
            if let Some(target) = aliased_target(tcx, *id, spelled, prefix) {
                aliases
                    .entry(owner.clone())
                    .or_default()
                    .insert((target, spelled.clone()));
            }
        }
    }
    (edges, aliases)
}
fn aliased_target(
    tcx: TyCtxt<'_>,
    id: DefId,
    spelled: &str,
    prefix: &str,
) -> Option<String> {
    let target = match tcx.def_kind(id) {
        DefKind::Ctor(CtorOf::Struct, _) => tcx.parent(id),
        _ => id,
    };
    if !target.is_local() || item_of(tcx, target) != Some(target) {
        return None;
    }
    let name = tcx.item_name(target).to_string();
    (name != spelled && in_prefix(&local_path(tcx, target), prefix)).then_some(name)
}
fn is_object(tcx: TyCtxt<'_>, def_id: DefId) -> bool {
    let kind_matches = matches!(
        tcx.def_kind(def_id), DefKind::Struct | DefKind::Enum | DefKind::Union |
        DefKind::Trait | DefKind::TyAlias | DefKind::Fn | DefKind::Const |
        DefKind::Static { .. }
    );
    kind_matches && matches!(tcx.def_kind(tcx.parent(def_id)), DefKind::Mod)
}
fn local_path(tcx: TyCtxt<'_>, def_id: DefId) -> String {
    let path = tcx.def_path_str(def_id);
    let crate_prefix = format!("{}::", tcx.crate_name(LOCAL_CRATE));
    path.strip_prefix("crate::")
        .or_else(|| path.strip_prefix(crate_prefix.as_str()))
        .unwrap_or(&path)
        .to_string()
}
fn in_prefix(path: &str, prefix: &str) -> bool {
    prefix.is_empty() || path == prefix || path.starts_with(&format!("{prefix}::"))
}
fn adt_of(ty: Ty<'_>) -> Option<DefId> {
    match ty.kind() {
        ty::Adt(definition, _) => Some(definition.did()),
        ty::Ref(_, inner, _) => adt_of(*inner),
        _ => None,
    }
}
fn impl_anchor(tcx: TyCtxt<'_>, impl_id: DefId) -> Option<DefId> {
    let self_ty = tcx.type_of(impl_id).skip_binder();
    if let Some(adt) = adt_of(self_ty).filter(|id| id.is_local()) {
        return Some(adt);
    }
    if tcx.impl_is_of_trait(impl_id) {
        Some(tcx.impl_trait_id(impl_id)).filter(|id| id.is_local())
    } else {
        None
    }
}
fn impl_owner(tcx: TyCtxt<'_>, impl_id: DefId, references: &[DefId]) -> Option<String> {
    let anchor = impl_anchor(tcx, impl_id);
    if let Some(anchor) = anchor {
        return Some(tcx.item_name(anchor).to_string());
    }
    references
        .iter()
        .filter_map(|id| item_of(tcx, *id))
        .filter(|id| id.is_local() && is_object(tcx, *id))
        .map(|id| tcx.item_name(id).to_string())
        .min()
}
fn item_of(tcx: TyCtxt<'_>, mut id: DefId) -> Option<DefId> {
    loop {
        match tcx.def_kind(id) {
            DefKind::Impl { .. } => return impl_anchor(tcx, id),
            DefKind::Struct
            | DefKind::Enum
            | DefKind::Union
            | DefKind::Trait
            | DefKind::TyAlias
            | DefKind::Fn
            | DefKind::Const
            | DefKind::Static { .. }
            | DefKind::ForeignTy => {
                let parent = tcx.parent(id);
                if matches!(tcx.def_kind(parent), DefKind::Mod) {
                    return Some(id);
                }
                id = parent;
            }
            DefKind::Variant
            | DefKind::Ctor(..)
            | DefKind::AssocFn
            | DefKind::AssocConst
            | DefKind::AssocTy
            | DefKind::Field
            | DefKind::Closure
            | DefKind::AnonConst => id = tcx.parent(id),
            _ => return None,
        }
    }
}
fn dependency_name(tcx: TyCtxt<'_>, def_id: DefId, prefix: &str) -> Option<String> {
    let item = item_of(tcx, def_id)?;
    if item.is_local() {
        let path = local_path(tcx, item);
        if in_prefix(&path, prefix) {
            Some(tcx.item_name(item).to_string())
        } else {
            Some(format!("crate::{path}"))
        }
    } else {
        Some(rustc_middle::ty::print::with_no_trimmed_paths!(tcx.def_path_str(item)))
    }
}
