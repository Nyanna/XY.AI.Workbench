use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::{
    Payload, PayloadRef, Stamp, StampObserver, StampSignal,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
const ALL_METHODS: [AccessMethod; 4] = [
    AccessMethod::Centroid,
    AccessMethod::Prefix,
    AccessMethod::Subtree,
    AccessMethod::Range,
];
struct FakeIndex {
    methods: Vec<AccessMethod>,
    entries: BTreeMap<u64, u64>,
    tree: BTreeMap<u64, Vec<u64>>,
    cells: BTreeMap<u64, Vec<u64>>,
    signal: Arc<StampSignal>,
    reads: Mutex<Vec<u64>>,
    touches: AtomicUsize,
}
impl FakeIndex {
    fn new(methods: &[AccessMethod]) -> Self {
        Self {
            methods: methods.to_vec(),
            entries: BTreeMap::new(),
            tree: BTreeMap::new(),
            cells: BTreeMap::new(),
            signal: StampSignal::new(Stamp(1)),
            reads: Mutex::new(Vec::new()),
            touches: AtomicUsize::new(0),
        }
    }
    fn with_entries(mut self, ids: &[u64]) -> Self {
        self.entries = ids.iter().map(|&i| (i, i + 100)).collect();
        self
    }
    fn touches(&self) -> usize {
        self.touches.load(Ordering::SeqCst)
    }
    fn touch(&self) {
        self.touches.fetch_add(1, Ordering::SeqCst);
    }
}
impl Index for FakeIndex {
    fn dtype(&self) -> DType {
        DType::F32
    }
    fn access_methods(&self) -> &[AccessMethod] {
        &self.methods
    }
    fn stamp(&self, id: EntryId) -> Option<Stamp> {
        self.touch();
        self.entries.get(&id.0).map(|&s| Stamp(s))
    }
    fn stamps(&self) -> Vec<(EntryId, Stamp)> {
        self.touch();
        self.entries.iter().map(|(&i, &s)| (EntryId(i), Stamp(s))).collect()
    }
    fn read(&self, id: EntryId) -> Payload {
        self.touch();
        self.reads.lock().unwrap().push(id.0);
        Payload::default()
    }
    fn write(
        &self,
        _id: EntryId,
        _payload: PayloadRef<'_>,
    ) -> Result<Stamp, IndexError> {
        Ok(Stamp(0))
    }
    fn delete(&self, _id: EntryId) -> Result<Option<Stamp>, IndexError> {
        Ok(None)
    }
    fn subscribe(&self) -> StampObserver {
        self.signal.observer()
    }
    fn children(&self, node: NodeId) -> Result<Vec<NodeId>, IndexError> {
        self.touch();
        self.tree
            .get(&node.0)
            .map(|c| c.iter().map(|&n| NodeId(n)).collect())
            .ok_or(IndexError::NotFound)
    }
    fn cell_entries(&self, cell: CellId) -> Result<Vec<EntryId>, IndexError> {
        self.touch();
        self.cells
            .get(&cell.0)
            .map(|c| c.iter().map(|&n| EntryId(n)).collect())
            .ok_or(IndexError::NotFound)
    }
}
struct FakeSelector {
    cells: Vec<u64>,
    error: Option<IndexError>,
    calls: Mutex<Vec<(u32, Option<f32>)>>,
}
impl FakeSelector {
    fn new(cells: &[u64]) -> Self {
        Self {
            cells: cells.to_vec(),
            error: None,
            calls: Mutex::new(Vec::new()),
        }
    }
    fn calls(&self) -> Vec<(u32, Option<f32>)> {
        self.calls.lock().unwrap().clone()
    }
}
impl CellSelector for FakeSelector {
    fn select(
        &self,
        _index: &dyn Index,
        count: u32,
        radius: Option<f32>,
    ) -> Result<Vec<CellId>, IndexError> {
        self.calls.lock().unwrap().push((count, radius));
        match &self.error {
            Some(e) => Err(e.clone()),
            None => Ok(self.cells.iter().map(|&c| CellId(c)).collect()),
        }
    }
}
fn ctx<'a>(
    index: &'a FakeIndex,
    selector: Option<&'a FakeSelector>,
) -> ResolveContext<'a> {
    ResolveContext {
        index: Some(index),
        cell_selector: selector.map(|s| s as &dyn CellSelector),
    }
}
fn ids(set: &EntrySet) -> Vec<u64> {
    set.iter().collect()
}
fn unsupported_err(method: &str) -> ResolveError {
    ResolveError::Unsupported {
        method: method.to_string(),
    }
}
fn tree_index() -> FakeIndex {
    let mut index = FakeIndex::new(&ALL_METHODS).with_entries(&[1, 2, 3]);
    index.tree.insert(10, vec![3, 5, 9]);
    index.tree.insert(11, vec![]);
    index.cells.insert(1, vec![1, 2]);
    index.cells.insert(2, vec![2, 3]);
    index.cells.insert(3, vec![40, 41]);
    index.cells.insert(4, vec![50]);
    index
}
#[test]
fn types_are_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<ScopeResolverRegistry>();
    check::<ResolveError>();
    check::<RangeResolver>();
    check::<SubtreeResolver>();
    check::<PrefixResolver>();
    check::<CentroidResolver>();
}
#[test]
fn method_names() {
    assert_eq!(method_name(AccessMethod::Centroid), "centroid");
    assert_eq!(method_name(AccessMethod::Prefix), "prefix");
    assert_eq!(method_name(AccessMethod::Subtree), "subtree");
    assert_eq!(method_name(AccessMethod::Range), "range");
}
#[test]
fn error_display_is_descriptive() {
    assert!(unsupported_err("prefix").to_string().contains("prefix"));
    assert!(! ResolveError::IndexUnavailable.to_string().is_empty());
    assert!(
        ResolveError::DuplicateResolver { kind : ScopeKind::Range } .to_string()
        .contains("range")
    );
    assert!(
        ResolveError::Index(IndexError::NotFound).to_string().contains(&
        IndexError::NotFound.to_string())
    );
}
#[test]
fn set_needs_neither_index_nor_resolver() {
    let registry = ScopeResolverRegistry::new();
    let set = EntrySet::from_ids([4, 8, u64::MAX]);
    let out = registry
        .resolve(&Scope::Set(set.clone()), &ResolveContext::default())
        .unwrap();
    assert_eq!(out, set);
}
#[test]
fn non_set_scopes_require_index() {
    let registry = ScopeResolverRegistry::with_defaults();
    let scopes = [
        Scope::All,
        Scope::Range { start: 0, end: 5 },
        Scope::Subtree { root: 1 },
        Scope::Prefix { bytes: vec![1] },
        Scope::Centroid {
            count: 1,
            radius: None,
        },
    ];
    for scope in scopes {
        assert_eq!(
            registry.resolve(& scope, & ResolveContext::default()),
            Err(ResolveError::IndexUnavailable), "{scope:?}"
        );
    }
}
#[test]
fn all_lists_stamps_without_reading_and_needs_no_method() {
    let index = FakeIndex::new(&[]).with_entries(&[7, 2, 9]);
    let registry = ScopeResolverRegistry::new();
    let out = registry.resolve(&Scope::All, &ctx(&index, None)).unwrap();
    assert_eq!(ids(& out), vec![2, 7, 9]);
    assert!(index.reads.lock().unwrap().is_empty());
}
#[test]
fn all_on_empty_index_is_empty() {
    let index = FakeIndex::new(&[]);
    let out = ScopeResolverRegistry::new()
        .resolve(&Scope::All, &ctx(&index, None))
        .unwrap();
    assert!(out.is_empty());
}
#[test]
fn range_is_half_open_and_unfiltered() {
    let index = FakeIndex::new(&[AccessMethod::Range]).with_entries(&[1]);
    let registry = ScopeResolverRegistry::default();
    let out = registry
        .resolve(&Scope::Range { start: 3, end: 6 }, &ctx(&index, None))
        .unwrap();
    assert_eq!(ids(& out), vec![3, 4, 5]);
    assert_eq!(index.touches(), 0);
}
#[test]
fn range_empty_and_reversed_yield_empty() {
    let index = FakeIndex::new(&[AccessMethod::Range]);
    let registry = ScopeResolverRegistry::default();
    for (start, end) in [(5, 5), (9, 2), (u64::MAX, u64::MAX), (u64::MAX, 0)] {
        let out = registry
            .resolve(&Scope::Range { start, end }, &ctx(&index, None))
            .unwrap();
        assert!(out.is_empty(), "{start}..{end}");
    }
}
#[test]
fn range_requires_declared_method() {
    let index = FakeIndex::new(&[AccessMethod::Prefix]);
    let out = ScopeResolverRegistry::default()
        .resolve(&Scope::Range { start: 0, end: 4 }, &ctx(&index, None));
    assert_eq!(out, Err(unsupported_err("range")));
}
#[test]
fn subtree_returns_children_without_root() {
    let index = tree_index();
    let out = ScopeResolverRegistry::default()
        .resolve(&Scope::Subtree { root: 10 }, &ctx(&index, None))
        .unwrap();
    assert_eq!(ids(& out), vec![3, 5, 9]);
    let leaf = ScopeResolverRegistry::default()
        .resolve(&Scope::Subtree { root: 11 }, &ctx(&index, None))
        .unwrap();
    assert!(leaf.is_empty());
}
#[test]
fn subtree_passes_index_errors_through() {
    let index = tree_index();
    let out = ScopeResolverRegistry::default()
        .resolve(&Scope::Subtree { root: 99 }, &ctx(&index, None));
    assert_eq!(out, Err(ResolveError::Index(IndexError::NotFound)));
}
#[test]
fn subtree_requires_declared_method() {
    let index = FakeIndex::new(&[AccessMethod::Range]);
    let out = ScopeResolverRegistry::default()
        .resolve(&Scope::Subtree { root: 1 }, &ctx(&index, None));
    assert_eq!(out, Err(unsupported_err("subtree")));
}
fn prefix(bytes: &[u8]) -> EntrySet {
    let index = FakeIndex::new(&[AccessMethod::Prefix]);
    let out = ScopeResolverRegistry::default()
        .resolve(
            &Scope::Prefix {
                bytes: bytes.to_vec(),
            },
            &ctx(&index, None),
        )
        .unwrap();
    assert_eq!(index.touches(), 0);
    out
}
#[test]
fn prefix_single_byte() {
    let set = prefix(&[0x12]);
    let (lo, hi) = (0x1200_0000_0000_0000u64, 0x12FF_FFFF_FFFF_FFFFu64);
    assert_eq!(set, EntrySet::from_ranges([lo..hi + 1]));
    assert_eq!(set.len(), 1u128 << 56);
    assert!(set.contains(lo) && set.contains(hi));
    assert!(! set.contains(lo - 1) && ! set.contains(hi + 1));
}
#[test]
fn prefix_multi_byte() {
    let set = prefix(&[0x01, 0x02]);
    let lo = 0x0102u64 << 48;
    assert_eq!(set.len(), 1u128 << 48);
    assert!(set.contains(lo) && set.contains(lo | 0xFFFF_FFFF_FFFF));
    assert!(! set.contains(lo - 1) && ! set.contains(lo + (1 << 48)));
}
#[test]
fn prefix_top_byte_includes_u64_max() {
    let set = prefix(&[0xFF]);
    assert!(set.contains(u64::MAX));
    assert!(set.contains(0xFF00_0000_0000_0000));
    assert!(! set.contains(0xFEFF_FFFF_FFFF_FFFF));
    assert_eq!(set.len(), 1u128 << 56);
}
#[test]
fn prefix_full_length_is_single_id() {
    assert_eq!(ids(& prefix(& [0xFF; 8])), vec![u64::MAX]);
    assert_eq!(ids(& prefix(& [0x00; 8])), vec![0]);
    assert_eq!(ids(& prefix(& [0, 0, 0, 0, 0, 0, 1, 2])), vec![0x0102]);
}
#[test]
fn prefix_empty_matches_everything_and_long_matches_nothing() {
    let all = prefix(&[]);
    assert_eq!(all.len(), 1u128 << 64);
    assert!(all.contains(0) && all.contains(u64::MAX));
    assert!(prefix(& [0; 9]).is_empty());
    assert!(prefix(& [0xFF; 12]).is_empty());
}
#[test]
fn prefix_requires_declared_method() {
    let index = FakeIndex::new(&[AccessMethod::Range]);
    let out = ScopeResolverRegistry::default()
        .resolve(&Scope::Prefix { bytes: vec![1] }, &ctx(&index, None));
    assert_eq!(out, Err(unsupported_err("prefix")));
}
#[test]
fn centroid_unions_cells_of_selector() {
    let index = tree_index();
    let selector = FakeSelector::new(&[1, 2]);
    let out = ScopeResolverRegistry::default()
        .resolve(
            &Scope::Centroid {
                count: 2,
                radius: Some(0.5),
            },
            &ctx(&index, Some(&selector)),
        )
        .unwrap();
    assert_eq!(ids(& out), vec![1, 2, 3]);
    assert_eq!(selector.calls(), vec![(2, Some(0.5))]);
}
#[test]
fn centroid_passes_absent_radius_unchanged() {
    let index = tree_index();
    let selector = FakeSelector::new(&[1]);
    ScopeResolverRegistry::default()
        .resolve(
            &Scope::Centroid {
                count: 7,
                radius: None,
            },
            &ctx(&index, Some(&selector)),
        )
        .unwrap();
    assert_eq!(selector.calls(), vec![(7, None)]);
}
#[test]
fn centroid_zero_count_skips_selector() {
    let index = tree_index();
    let selector = FakeSelector::new(&[1]);
    let out = ScopeResolverRegistry::default()
        .resolve(
            &Scope::Centroid {
                count: 0,
                radius: None,
            },
            &ctx(&index, Some(&selector)),
        )
        .unwrap();
    assert!(out.is_empty());
    assert!(selector.calls().is_empty());
}
#[test]
fn centroid_dedups_in_order_and_truncates_to_count() {
    let index = tree_index();
    let selector = FakeSelector::new(&[3, 3, 1, 3, 2, 4]);
    let out = ScopeResolverRegistry::default()
        .resolve(
            &Scope::Centroid {
                count: 2,
                radius: None,
            },
            &ctx(&index, Some(&selector)),
        )
        .unwrap();
    assert_eq!(ids(& out), vec![1, 2, 40, 41]);
}
#[test]
fn centroid_truncation_does_not_touch_surplus_cells() {
    let mut index = tree_index();
    index.cells.remove(&4);
    let selector = FakeSelector::new(&[1, 4]);
    let out = ScopeResolverRegistry::default()
        .resolve(
            &Scope::Centroid {
                count: 1,
                radius: None,
            },
            &ctx(&index, Some(&selector)),
        );
    assert_eq!(ids(& out.unwrap()), vec![1, 2]);
}
#[test]
fn centroid_missing_cell_fails_without_partial_result() {
    let mut index = tree_index();
    index.cells.remove(&2);
    let selector = FakeSelector::new(&[1, 2]);
    let out = ScopeResolverRegistry::default()
        .resolve(
            &Scope::Centroid {
                count: 2,
                radius: None,
            },
            &ctx(&index, Some(&selector)),
        );
    assert_eq!(out, Err(ResolveError::Index(IndexError::NotFound)));
}
#[test]
fn centroid_selector_error_is_wrapped() {
    let index = tree_index();
    let mut selector = FakeSelector::new(&[]);
    selector.error = Some(IndexError::Backend("boom".into()));
    let out = ScopeResolverRegistry::default()
        .resolve(
            &Scope::Centroid {
                count: 1,
                radius: None,
            },
            &ctx(&index, Some(&selector)),
        );
    assert_eq!(out, Err(ResolveError::Index(IndexError::Backend("boom".into()))));
}
#[test]
fn centroid_check_order() {
    let scope = Scope::Centroid {
        count: 1,
        radius: None,
    };
    let registry = ScopeResolverRegistry::default();
    let selector = FakeSelector::new(&[1]);
    let undeclared = FakeIndex::new(&[AccessMethod::Range]);
    assert_eq!(
        registry.resolve(& scope, & ctx(& undeclared, Some(& selector))),
        Err(unsupported_err("centroid"))
    );
    assert!(selector.calls().is_empty());
    let declared = tree_index();
    assert_eq!(
        registry.resolve(& scope, & ctx(& declared, None)),
        Err(unsupported_err("centroid"))
    );
    let no_index = ResolveContext {
        index: None,
        cell_selector: Some(&selector),
    };
    assert_eq!(
        registry.resolve(& scope, & no_index), Err(ResolveError::IndexUnavailable)
    );
}
#[test]
fn resolvers_reject_foreign_scopes_regardless_of_index() {
    let index = tree_index();
    let selector = FakeSelector::new(&[1]);
    let contexts = [ResolveContext::default(), ctx(&index, Some(&selector))];
    let foreign = [
        Scope::All,
        Scope::Set(EntrySet::empty()),
        Scope::Range { start: 0, end: 1 },
        Scope::Subtree { root: 10 },
        Scope::Prefix { bytes: vec![1] },
        Scope::Centroid {
            count: 1,
            radius: None,
        },
    ];
    let resolvers: [Arc<dyn ScopeResolver>; 4] = [
        Arc::new(RangeResolver),
        Arc::new(SubtreeResolver),
        Arc::new(PrefixResolver),
        Arc::new(CentroidResolver),
    ];
    for resolver in &resolvers {
        for scope in foreign.iter().filter(|s| s.kind() != resolver.kind()) {
            for c in &contexts {
                assert_eq!(
                    resolver.resolve(scope, c), Err(unsupported_err(method_name(resolver
                    .access_method()))), "{:?} / {scope:?}", resolver.kind()
                );
            }
        }
    }
    assert_eq!(index.touches(), 0);
    assert!(selector.calls().is_empty());
}
#[test]
fn resolvers_report_kind_and_method() {
    assert_eq!(RangeResolver.kind(), ScopeKind::Range);
    assert_eq!(RangeResolver.access_method(), AccessMethod::Range);
    assert_eq!(SubtreeResolver.kind(), ScopeKind::Subtree);
    assert_eq!(SubtreeResolver.access_method(), AccessMethod::Subtree);
    assert_eq!(PrefixResolver.kind(), ScopeKind::Prefix);
    assert_eq!(PrefixResolver.access_method(), AccessMethod::Prefix);
    assert_eq!(CentroidResolver.kind(), ScopeKind::Centroid);
    assert_eq!(CentroidResolver.access_method(), AccessMethod::Centroid);
}
#[test]
fn empty_registry_rejects_without_fallback() {
    let index = tree_index();
    let registry = ScopeResolverRegistry::new();
    let cases = [
        (Scope::Range { start: 0, end: 4 }, "range"),
        (Scope::Subtree { root: 10 }, "subtree"),
        (Scope::Prefix { bytes: vec![1] }, "prefix"),
        (
            Scope::Centroid {
                count: 1,
                radius: None,
            },
            "centroid",
        ),
    ];
    for (scope, name) in cases {
        assert_eq!(
            registry.resolve(& scope, & ctx(& index, None)), Err(unsupported_err(name))
        );
    }
}
#[test]
fn register_rejects_duplicates_and_keeps_state() {
    let mut registry = ScopeResolverRegistry::with_defaults();
    assert_eq!(
        registry.register(Arc::new(RangeResolver)), Err(ResolveError::DuplicateResolver {
        kind : ScopeKind::Range })
    );
    let index = tree_index();
    let out = registry
        .resolve(&Scope::Range { start: 1, end: 3 }, &ctx(&index, None))
        .unwrap();
    assert_eq!(ids(& out), vec![1, 2]);
}
#[test]
fn register_accepts_each_kind_once_on_empty_registry() {
    let mut registry = ScopeResolverRegistry::new();
    registry.register(Arc::new(PrefixResolver)).unwrap();
    assert_eq!(
        registry.register(Arc::new(PrefixResolver)), Err(ResolveError::DuplicateResolver
        { kind : ScopeKind::Prefix })
    );
    registry.register(Arc::new(RangeResolver)).unwrap();
}
struct ClaimsKind(ScopeKind);
impl ScopeResolver for ClaimsKind {
    fn kind(&self) -> ScopeKind {
        self.0
    }
    fn access_method(&self) -> AccessMethod {
        AccessMethod::Range
    }
    fn resolve(
        &self,
        _scope: &Scope,
        _ctx: &ResolveContext<'_>,
    ) -> Result<EntrySet, ResolveError> {
        Ok(EntrySet::from_ids([42]))
    }
}
#[test]
fn all_and_set_are_always_taken() {
    let mut registry = ScopeResolverRegistry::new();
    for kind in [ScopeKind::All, ScopeKind::Set] {
        assert_eq!(
            registry.register(Arc::new(ClaimsKind(kind))),
            Err(ResolveError::DuplicateResolver { kind })
        );
    }
}
#[test]
fn custom_resolver_is_dispatched_after_registration() {
    let mut registry = ScopeResolverRegistry::new();
    registry.register(Arc::new(ClaimsKind(ScopeKind::Subtree))).unwrap();
    let index = FakeIndex::new(&[AccessMethod::Range]);
    let out = registry.resolve(&Scope::Subtree { root: 1 }, &ctx(&index, None)).unwrap();
    assert_eq!(ids(& out), vec![42]);
}
#[test]
fn default_registry_has_standard_resolvers() {
    let registry = ScopeResolverRegistry::default();
    let index = tree_index();
    assert!(
        registry.resolve(& Scope::Range { start : 0, end : 2 }, & ctx(& index, None))
        .is_ok()
    );
    assert!(
        registry.resolve(& Scope::Prefix { bytes : vec![0] }, & ctx(& index, None))
        .is_ok()
    );
    assert!(
        registry.resolve(& Scope::Subtree { root : 10 }, & ctx(& index, None)).is_ok()
    );
}
#[test]
fn registry_is_shareable_across_threads() {
    let registry = Arc::new(ScopeResolverRegistry::default());
    let index = Arc::new(tree_index());
    let handles: Vec<_> = (0..4u64)
        .map(|t| {
            let (registry, index) = (registry.clone(), index.clone());
            std::thread::spawn(move || {
                let c = ResolveContext {
                    index: Some(&*index),
                    cell_selector: None,
                };
                let out = registry
                    .resolve(
                        &Scope::Range {
                            start: t,
                            end: t + 3,
                        },
                        &c,
                    )
                    .unwrap();
                out.len()
            })
        })
        .collect();
    for h in handles {
        assert_eq!(h.join().unwrap(), 3);
    }
}
#[test]
fn load_candidates_visits_exactly_the_set_ascending() {
    let index = FakeIndex::new(&[]).with_entries(&[1, 2, 3, 4, 5, 6]);
    let candidates = EntrySet::from_ids([5, 2, 4, 99]);
    let mut seen = Vec::new();
    load_candidates(
        &index,
        &candidates,
        |id, loaded| {
            seen.push((id.0, loaded.map(|l| l.stamp)));
        },
    );
    assert_eq!(
        seen, vec![(2, Some(Stamp(102))), (4, Some(Stamp(104))), (5, Some(Stamp(105))),
        (99, None),]
    );
    assert_eq!(* index.reads.lock().unwrap(), vec![2, 4, 5]);
}
#[test]
fn load_candidates_with_empty_set_does_not_touch_index() {
    let index = FakeIndex::new(&[]).with_entries(&[1, 2]);
    let mut calls = 0;
    load_candidates(&index, &EntrySet::empty(), |_, _| calls += 1);
    assert_eq!(calls, 0);
    assert_eq!(index.touches(), 0);
}
