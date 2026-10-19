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
mod tests {
    use std::sync::atomic::{AtomicU32, Ordering};
    use tokio::sync::watch;
    use super::*;
    use crate::resident::index::memory::InMemoryIndex;
    use crate::resident::index::{EntryData, EntryId, NodeId, Stamp};
    struct FakeIndex {
        inner: InMemoryIndex,
        caps: Vec<AccessMethod>,
        children: HashMap<u64, EntrySet>,
        cells: HashMap<u64, EntrySet>,
        summary: Option<SummaryRef>,
    }
    impl FakeIndex {
        fn new(caps: &[AccessMethod]) -> Self {
            Self {
                inner: InMemoryIndex::new(),
                caps: caps.to_vec(),
                children: HashMap::new(),
                cells: HashMap::new(),
                summary: None,
            }
        }
    }
    impl Index for FakeIndex {
        fn stamp(&self, entry: EntryId) -> Option<Stamp> {
            self.inner.stamp(entry)
        }
        fn stamps(&self) -> Box<dyn Iterator<Item = (EntryId, Stamp)> + '_> {
            self.inner.stamps()
        }
        fn read(&self, entry: EntryId) -> EntryData<'_> {
            self.inner.read(entry)
        }
        fn apply(&self, entry: EntryId, data: &[u8]) -> Stamp {
            self.inner.apply(entry, data)
        }
        fn changed(&self) -> watch::Receiver<u64> {
            self.inner.changed()
        }
        fn capabilities(&self) -> &[AccessMethod] {
            &self.caps
        }
        fn children(&self, node: NodeId) -> EntrySet {
            self.children.get(&node.0).cloned().unwrap_or_default()
        }
        fn members(&self, cell: CellId) -> EntrySet {
            self.cells.get(&cell.0).cloned().unwrap_or_default()
        }
        fn summary(&self, _: AccessMethod) -> Option<SummaryRef> {
            self.summary.clone()
        }
        fn prefetch(&self, _: &EntrySet) {}
    }
    struct FixedCells {
        cells: Vec<CellId>,
        calls: AtomicU32,
    }
    impl CellSelector for FixedCells {
        fn select(
            &self,
            _: &SummaryRef,
            _: u32,
            _: Option<f32>,
        ) -> Result<Vec<CellId>, IndexError> {
            self.calls.fetch_add(1, Ordering::Relaxed);
            Ok(self.cells.clone())
        }
    }
    fn ids(set: &EntrySet) -> Vec<u64> {
        set.iter().map(EntryId::get).collect()
    }
    fn is_unsupported(r: Result<EntrySet, IndexError>) -> bool {
        matches!(r, Err(IndexError::UnsupportedScope { .. }))
    }
    #[test]
    fn undeclared_method_is_unsupported() {
        let index = FakeIndex::new(&[AccessMethod::Range]);
        let reg = ResolverRegistry::with_defaults();
        let r = reg.resolve(&index, &Scope::Subtree(NodeId(1)), &ResolveCtx::default());
        assert!(is_unsupported(r));
    }
    #[test]
    fn declared_without_resolver_is_unsupported() {
        let index = FakeIndex::new(&[AccessMethod::Range]);
        let reg = ResolverRegistry::new();
        let r = reg.resolve(&index, &Scope::Range(0..4), &ResolveCtx::default());
        assert!(is_unsupported(r));
    }
    #[test]
    fn duplicate_registration_is_rejected() {
        let mut reg = ResolverRegistry::new();
        reg.register(Arc::new(RangeResolver)).unwrap();
        let err = reg.register(Arc::new(RangeResolver)).unwrap_err();
        assert_eq!(err.0, AccessMethod::Range);
    }
    #[test]
    fn range_subtree_prefix_resolve() {
        let mut index = FakeIndex::new(
            &[AccessMethod::Range, AccessMethod::Subtree, AccessMethod::Prefix],
        );
        index.children.insert(7, EntrySet::from_ranges([10..12, 20..21]));
        let reg = ResolverRegistry::with_defaults();
        let ctx = ResolveCtx::default();
        let r = reg.resolve(&index, &Scope::Range(3..6), &ctx).unwrap();
        assert_eq!(ids(& r), [3, 4, 5]);
        let s = reg.resolve(&index, &Scope::Subtree(NodeId(7)), &ctx).unwrap();
        assert_eq!(ids(& s), [10, 11, 20]);
        let p = reg
            .resolve(&index, &Scope::Prefix(vec![0, 0, 0, 0, 0, 0, 1]), &ctx)
            .unwrap();
        assert_eq!(p.ranges(), & [256..512]);
        let exact = reg.resolve(&index, &Scope::Prefix(vec![0; 8]), &ctx).unwrap();
        assert_eq!(exact.ranges(), & [0..1]);
        let longer = reg.resolve(&index, &Scope::Prefix(vec![0; 9]), &ctx).unwrap();
        assert!(longer.is_empty());
    }
    #[test]
    fn all_and_set_need_no_method() {
        let index = FakeIndex::new(&[]);
        for n in [2, 3, 9] {
            index.apply(EntryId::new(n), b"x");
        }
        let reg = ResolverRegistry::new();
        let ctx = ResolveCtx::default();
        assert_eq!(ids(& reg.resolve(& index, & Scope::All, & ctx).unwrap()), [2, 3, 9]);
        let set = EntrySet::from_ids([EntryId::new(5)]);
        let out = reg.resolve(&index, &Scope::Set(set.clone()), &ctx).unwrap();
        assert_eq!(out, set);
    }
    #[test]
    fn centroid_unions_member_cells_and_respects_k() {
        let mut index = FakeIndex::new(&[AccessMethod::Centroid]);
        index.summary = Some(SummaryRef(Arc::from(vec![0u8; 4])));
        index.cells.insert(1, EntrySet::from_ranges([0..3]));
        index.cells.insert(2, EntrySet::from_ranges([2..5]));
        index.cells.insert(3, EntrySet::from_ranges([100..101]));
        let selector = Arc::new(FixedCells {
            cells: vec![CellId(1), CellId(2), CellId(3)],
            calls: AtomicU32::new(0),
        });
        let ctx = ResolveCtx {
            cells: Some(selector.clone()),
        };
        let reg = ResolverRegistry::with_defaults();
        let scope = Scope::Centroid {
            k: 2,
            radius: None,
        };
        let out = reg.resolve(&index, &scope, &ctx).unwrap();
        assert_eq!(ids(& out), [0, 1, 2, 3, 4]);
        assert_eq!(selector.calls.load(Ordering::Relaxed), 1);
    }
    #[test]
    fn centroid_without_summary_or_selector_fails_loudly() {
        let mut index = FakeIndex::new(&[AccessMethod::Centroid]);
        let reg = ResolverRegistry::with_defaults();
        let scope = Scope::Centroid {
            k: 1,
            radius: None,
        };
        let r = reg.resolve(&index, &scope, &ResolveCtx::default());
        assert!(matches!(r, Err(IndexError::IndexUnavailable { .. })));
        index.summary = Some(SummaryRef(Arc::from(vec![0u8])));
        let r = reg.resolve(&index, &scope, &ResolveCtx::default());
        assert!(is_unsupported(r));
    }
    #[test]
    fn union_and_intersection_of_scopes() {
        let a = EntrySet::from_ranges([0..5, 10..15]);
        let b = EntrySet::from_ranges([3..12]);
        assert_eq!(a.union(& b).ranges(), & [0..15]);
        assert_eq!(a.intersection(& b).ranges(), & [3..5, 10..12]);
        assert!(a.intersection(& EntrySet::new()).is_empty());
    }
}
