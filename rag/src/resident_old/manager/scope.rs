//! Scope resolution: the manager turns a task scope into a compact entry set.
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use crate::resident::error::IndexError;
use crate::resident::index::{AccessMethod, CellId, EntrySet, Index, Scope, SummaryRef};
/// Picks the cells nearest to the query from a resident summary (e.g. `S = C·Qᵀ`, top `nprobe`).
pub trait CellSelector: Send + Sync {
    fn select(
        &self,
        summary: &SummaryRef,
        k: u32,
        radius: Option<f32>,
    ) -> Result<Vec<CellId>, IndexError>;
}
#[derive(Clone, Default)]
pub struct ResolveCtx {
    pub cells: Option<Arc<dyn CellSelector>>,
}
pub trait ScopeResolver: Send + Sync {
    fn method(&self) -> AccessMethod;
    fn resolve(
        &self,
        index: &dyn Index,
        scope: &Scope,
        ctx: &ResolveCtx,
    ) -> Result<EntrySet, IndexError>;
}
#[derive(Debug, Error)]
#[error("a resolver for access method {0:?} is already registered")]
pub struct DuplicateResolver(pub AccessMethod);
#[derive(Default)]
pub struct ResolverRegistry {
    map: HashMap<AccessMethod, Arc<dyn ScopeResolver>>,
}
impl ResolverRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_defaults() -> Self {
        let mut r = Self::new();
        for resolver in [
            Arc::new(RangeResolver) as Arc<dyn ScopeResolver>,
            Arc::new(SubtreeResolver),
            Arc::new(PrefixResolver),
            Arc::new(CentroidResolver),
        ] {
            r.register(resolver).expect("default resolvers have distinct methods");
        }
        r
    }
    pub fn register(
        &mut self,
        resolver: Arc<dyn ScopeResolver>,
    ) -> Result<(), DuplicateResolver> {
        let method = resolver.method();
        if self.map.contains_key(&method) {
            return Err(DuplicateResolver(method));
        }
        self.map.insert(method, resolver);
        Ok(())
    }
    pub fn resolve(
        &self,
        index: &dyn Index,
        scope: &Scope,
        ctx: &ResolveCtx,
    ) -> Result<EntrySet, IndexError> {
        let method = match required_method(scope) {
            Some(m) => m,
            None => return Ok(resolve_direct(index, scope)),
        };
        let span = tracing::debug_span!("scope_resolve", ? method);
        let _enter = span.enter();
        let unsupported = || IndexError::UnsupportedScope {
            method: format!("{method:?}"),
        };
        if !index.capabilities().contains(&method) {
            return Err(unsupported());
        }
        let resolver = self.map.get(&method).ok_or_else(unsupported)?;
        let set = resolver.resolve(index, scope, ctx)?;
        tracing::debug!(entries = set.len(), "scope resolved");
        Ok(set)
    }
}
fn required_method(scope: &Scope) -> Option<AccessMethod> {
    match scope {
        Scope::All | Scope::Set(_) => None,
        Scope::Range(_) => Some(AccessMethod::Range),
        Scope::Subtree(_) => Some(AccessMethod::Subtree),
        Scope::Prefix(_) => Some(AccessMethod::Prefix),
        Scope::Centroid { .. } => Some(AccessMethod::Centroid),
    }
}
fn resolve_direct(index: &dyn Index, scope: &Scope) -> EntrySet {
    match scope {
        Scope::Set(set) => set.clone(),
        _ => EntrySet::from_ids(index.stamps().map(|(id, _)| id)),
    }
}
fn mismatch(method: AccessMethod) -> IndexError {
    IndexError::UnsupportedScope {
        method: format!("{method:?}"),
    }
}
pub struct RangeResolver;
impl ScopeResolver for RangeResolver {
    fn method(&self) -> AccessMethod {
        AccessMethod::Range
    }
    fn resolve(
        &self,
        _: &dyn Index,
        scope: &Scope,
        _: &ResolveCtx,
    ) -> Result<EntrySet, IndexError> {
        match scope {
            Scope::Range(r) => Ok(EntrySet::from_ranges([r.clone()])),
            _ => Err(mismatch(self.method())),
        }
    }
}
pub struct SubtreeResolver;
impl ScopeResolver for SubtreeResolver {
    fn method(&self) -> AccessMethod {
        AccessMethod::Subtree
    }
    fn resolve(
        &self,
        index: &dyn Index,
        scope: &Scope,
        _: &ResolveCtx,
    ) -> Result<EntrySet, IndexError> {
        match scope {
            Scope::Subtree(node) => Ok(index.children(*node)),
            _ => Err(mismatch(self.method())),
        }
    }
}
/// Interprets the prefix as the leading big-endian bytes of the 64-bit entry id,
/// which yields one contiguous range. Prefixes longer than 8 bytes match nothing.
pub struct PrefixResolver;
impl ScopeResolver for PrefixResolver {
    fn method(&self) -> AccessMethod {
        AccessMethod::Prefix
    }
    fn resolve(
        &self,
        _: &dyn Index,
        scope: &Scope,
        _: &ResolveCtx,
    ) -> Result<EntrySet, IndexError> {
        let Scope::Prefix(prefix) = scope else {
            return Err(mismatch(self.method()));
        };
        let n = prefix.len();
        if n > 8 {
            return Ok(EntrySet::new());
        }
        let mut bytes = [0u8; 8];
        bytes[..n].copy_from_slice(prefix);
        let start = u64::from_be_bytes(bytes);
        let width = if n == 0 {
            u128::from(u64::MAX) + 1
        } else {
            1u128 << (8 * (8 - n))
        };
        let stop = (u128::from(start) + width).min(u128::from(u64::MAX));
        Ok(EntrySet::from_ranges([start..stop as u64]))
    }
}
pub struct CentroidResolver;
impl ScopeResolver for CentroidResolver {
    fn method(&self) -> AccessMethod {
        AccessMethod::Centroid
    }
    fn resolve(
        &self,
        index: &dyn Index,
        scope: &Scope,
        ctx: &ResolveCtx,
    ) -> Result<EntrySet, IndexError> {
        let Scope::Centroid { k, radius } = scope else {
            return Err(mismatch(self.method()));
        };
        let summary = index
            .summary(AccessMethod::Centroid)
            .ok_or_else(|| IndexError::IndexUnavailable {
                scope: "centroid".into(),
            })?;
        let selector = ctx.cells.as_ref().ok_or_else(|| mismatch(self.method()))?;
        let cells = selector.select(&summary, *k, *radius)?;
        Ok(
            cells
                .into_iter()
                .take(*k as usize)
                .fold(EntrySet::new(), |acc, cell| acc.union(&index.members(cell))),
        )
    }
}
#[cfg(test)]
#[path = "test/scope.rs"]
mod tests;
