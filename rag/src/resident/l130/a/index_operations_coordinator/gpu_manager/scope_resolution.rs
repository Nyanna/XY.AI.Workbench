use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::Arc;

use crate::resident::l130::l120::l110::l100::l090::l080::l070::a::reconcile::{
    load_entry, LoadedEntry,
};
use crate::resident::l130::l120::l110::l100::l090::l080::l070::l060::l050::l040::l030::l020 as l020;
use l020::a::index_contract::{AccessMethod, CellId, EntryId, Index, IndexError, NodeId};
use l020::l010::scopes_and_entry_sets::{EntrySet, Scope, ScopeKind};

pub const fn method_name(m: AccessMethod) -> &'static str {
    match m {
        AccessMethod::Centroid => "centroid",
        AccessMethod::Prefix => "prefix",
        AccessMethod::Subtree => "subtree",
        AccessMethod::Range => "range",
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResolveError {
    Unsupported { method: String },
    IndexUnavailable,
    DuplicateResolver { kind: ScopeKind },
    Index(IndexError),
}

impl fmt::Display for ResolveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ResolveError::Unsupported { method } => write!(f, "scope not supported: {method}"),
            ResolveError::IndexUnavailable => f.write_str("index not available"),
            ResolveError::DuplicateResolver { kind } => {
                write!(f, "resolver already registered for scope kind {}", kind.name())
            }
            ResolveError::Index(e) => write!(f, "index error: {e}"),
        }
    }
}

impl std::error::Error for ResolveError {}

fn unsupported(m: AccessMethod) -> ResolveError {
    ResolveError::Unsupported {
        method: method_name(m).to_string(),
    }
}

pub trait CellSelector: Send + Sync {
    fn select(
        &self,
        index: &dyn Index,
        count: u32,
        radius: Option<f32>,
    ) -> Result<Vec<CellId>, IndexError>;
}

#[derive(Clone, Copy, Default)]
pub struct ResolveContext<'a> {
    pub index: Option<&'a dyn Index>,
    pub cell_selector: Option<&'a dyn CellSelector>,
}

pub trait ScopeResolver: Send + Sync {
    fn kind(&self) -> ScopeKind;
    fn access_method(&self) -> AccessMethod;
    fn resolve(&self, scope: &Scope, ctx: &ResolveContext<'_>) -> Result<EntrySet, ResolveError>;
}

#[derive(Debug, Default, Clone)]
pub struct RangeResolver;

#[derive(Debug, Default, Clone)]
pub struct SubtreeResolver;

#[derive(Debug, Default, Clone)]
pub struct PrefixResolver;

#[derive(Debug, Default, Clone)]
pub struct CentroidResolver;

impl ScopeResolver for RangeResolver {
    fn kind(&self) -> ScopeKind {
        ScopeKind::Range
    }

    fn access_method(&self) -> AccessMethod {
        AccessMethod::Range
    }

    fn resolve(&self, scope: &Scope, _ctx: &ResolveContext<'_>) -> Result<EntrySet, ResolveError> {
        match scope {
            Scope::Range { start, end } => Ok(EntrySet::from_ranges([*start..*end])),
            _ => Err(unsupported(self.access_method())),
        }
    }
}

impl ScopeResolver for SubtreeResolver {
    fn kind(&self) -> ScopeKind {
        ScopeKind::Subtree
    }

    fn access_method(&self) -> AccessMethod {
        AccessMethod::Subtree
    }

    fn resolve(&self, scope: &Scope, ctx: &ResolveContext<'_>) -> Result<EntrySet, ResolveError> {
        let Scope::Subtree { root } = scope else {
            return Err(unsupported(self.access_method()));
        };
        let index = ctx.index.ok_or(ResolveError::IndexUnavailable)?;
        let children = index.children(NodeId(*root)).map_err(ResolveError::Index)?;
        Ok(EntrySet::from_ids(children.into_iter().map(|n| n.0)))
    }
}

impl ScopeResolver for PrefixResolver {
    fn kind(&self) -> ScopeKind {
        ScopeKind::Prefix
    }

    fn access_method(&self) -> AccessMethod {
        AccessMethod::Prefix
    }

    fn resolve(&self, scope: &Scope, _ctx: &ResolveContext<'_>) -> Result<EntrySet, ResolveError> {
        let Scope::Prefix { bytes } = scope else {
            return Err(unsupported(self.access_method()));
        };
        Ok(prefix_set(bytes))
    }
}

fn prefix_set(bytes: &[u8]) -> EntrySet {
    if bytes.len() > 8 {
        return EntrySet::empty();
    }
    let mut lo = [0x00u8; 8];
    let mut hi = [0xFFu8; 8];
    lo[..bytes.len()].copy_from_slice(bytes);
    hi[..bytes.len()].copy_from_slice(bytes);
    let (lo, hi) = (u64::from_be_bytes(lo), u64::from_be_bytes(hi));
    EntrySet::from_ranges([lo..hi]).union(&EntrySet::from_ids([hi]))
}

impl ScopeResolver for CentroidResolver {
    fn kind(&self) -> ScopeKind {
        ScopeKind::Centroid
    }

    fn access_method(&self) -> AccessMethod {
        AccessMethod::Centroid
    }

    fn resolve(&self, scope: &Scope, ctx: &ResolveContext<'_>) -> Result<EntrySet, ResolveError> {
        let Scope::Centroid { count, radius } = scope else {
            return Err(unsupported(self.access_method()));
        };
        let index = ctx.index.ok_or(ResolveError::IndexUnavailable)?;
        let selector = ctx
            .cell_selector
            .ok_or_else(|| unsupported(self.access_method()))?;
        if *count == 0 {
            return Ok(EntrySet::empty());
        }
        let selected = selector
            .select(index, *count, *radius)
            .map_err(ResolveError::Index)?;
        let mut seen = HashSet::new();
        let cells = selected
            .into_iter()
            .filter(|c| seen.insert(c.0))
            .take(*count as usize);
        let mut ids: Vec<u64> = Vec::new();
        for cell in cells {
            let entries = index.cell_entries(cell).map_err(ResolveError::Index)?;
            ids.extend(entries.into_iter().map(|e| e.0));
        }
        Ok(EntrySet::from_ids(ids))
    }
}

pub struct ScopeResolverRegistry {
    resolvers: HashMap<ScopeKind, Arc<dyn ScopeResolver>>,
}

impl ScopeResolverRegistry {
    pub fn new() -> Self {
        Self {
            resolvers: HashMap::new(),
        }
    }

    pub fn with_defaults() -> Self {
        let mut registry = Self::new();
        let defaults: [Arc<dyn ScopeResolver>; 4] = [
            Arc::new(RangeResolver),
            Arc::new(SubtreeResolver),
            Arc::new(PrefixResolver),
            Arc::new(CentroidResolver),
        ];
        for resolver in defaults {
            registry
                .resolvers
                .insert(resolver.kind(), resolver);
        }
        registry
    }

    pub fn register(&mut self, r: Arc<dyn ScopeResolver>) -> Result<(), ResolveError> {
        let kind = r.kind();
        if matches!(kind, ScopeKind::All | ScopeKind::Set) || self.resolvers.contains_key(&kind) {
            return Err(ResolveError::DuplicateResolver { kind });
        }
        self.resolvers.insert(kind, r);
        Ok(())
    }

    pub fn resolve(&self, scope: &Scope, ctx: &ResolveContext<'_>) -> Result<EntrySet, ResolveError> {
        let span = tracing::debug_span!("scope_resolve", method = scope.name());
        let _entered = span.enter();
        let result = self.dispatch(scope, ctx);
        if let Ok(set) = &result {
            tracing::debug!(count = set.len(), "scope resolved");
        }
        result
    }

    fn dispatch(&self, scope: &Scope, ctx: &ResolveContext<'_>) -> Result<EntrySet, ResolveError> {
        if let Scope::Set(set) = scope {
            return Ok(set.clone());
        }
        let index = ctx.index.ok_or(ResolveError::IndexUnavailable)?;
        if matches!(scope, Scope::All) {
            return Ok(EntrySet::from_ids(
                index.stamps().into_iter().map(|(id, _)| id.0),
            ));
        }
        let resolver = self
            .resolvers
            .get(&scope.kind())
            .ok_or_else(|| ResolveError::Unsupported {
                method: scope.name().to_string(),
            })?;
        let method = resolver.access_method();
        if !index.access_methods().contains(&method) {
            return Err(unsupported(method));
        }
        resolver.resolve(scope, ctx)
    }
}

impl Default for ScopeResolverRegistry {
    fn default() -> Self {
        Self::with_defaults()
    }
}

pub fn load_candidates<F>(index: &dyn Index, candidates: &EntrySet, mut f: F)
where
    F: FnMut(EntryId, Option<LoadedEntry>),
{
    for id in candidates.iter() {
        let id = EntryId(id);
        f(id, load_entry(index, id));
    }
}

#[cfg(test)]
#[path = "test/scope_resolution.rs"]
mod tests;
