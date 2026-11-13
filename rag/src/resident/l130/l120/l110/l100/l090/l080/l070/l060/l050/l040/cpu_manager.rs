use std::collections::{HashMap, HashSet};
use std::fmt;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
use std::time::{Duration, Instant};
use log::warn;
use super::l030::l020::a::index_contract::{
    AccessMethod, ByteRange, EntryId, Index, Payload,
};
const MSG_NOT_ATTACHED: &str = "Index nicht angehängt";
const MSG_TASK_ABORTED: &str = "Lese-Task abgebrochen";
/// Lazily activated index: the slot is filled once by the owner at a later point.
pub type LazyIndex = Arc<OnceLock<Arc<dyn Index>>>;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Advice {
    WillNeed,
    Sequential,
    Random,
    Cold,
}
/// Hint sink (e.g. the mmap region of the index). Applies the hint to `[offset, offset+len)`.
pub trait MemoryAdvisor: Send + Sync {
    fn advise(&self, offset: u64, len: u64, advice: Advice) -> Result<(), String>;
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferError(pub String);
impl fmt::Display for TransferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for TransferError {}
/// Copy of the entry bytes (not shared). A missing entry yields both parts empty.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct OwnedPayload {
    pub vector: Vec<u8>,
    pub meta: Vec<u8>,
}
impl From<Payload> for OwnedPayload {
    fn from(payload: Payload) -> Self {
        OwnedPayload {
            vector: payload.vector.to_vec(),
            meta: payload.meta.to_vec(),
        }
    }
}
type Span = (u64, u64);
#[derive(Debug, Clone, PartialEq, Eq)]
enum HintPlan {
    None,
    Dense(Span),
    Scattered(Vec<Span>),
}
fn plan_hints(ranges: &[Option<Span>]) -> HintPlan {
    let mut distinct: Vec<Span> = Vec::new();
    let mut seen: HashSet<Span> = HashSet::new();
    for &(offset, len) in ranges.iter().flatten() {
        if len > 0 && seen.insert((offset, len)) {
            distinct.push((offset, len));
        }
    }
    if distinct.is_empty() {
        return HintPlan::None;
    }
    let start = distinct.iter().map(|r| r.0).min().unwrap_or(0);
    let end = distinct.iter().map(|r| r.0.saturating_add(r.1)).max().unwrap_or(start);
    let span_len = end - start;
    let mut sorted = distinct.clone();
    sorted.sort_unstable();
    let mut covered: u128 = 0;
    let mut run: Option<(u64, u64)> = None;
    for (offset, len) in sorted {
        let range_end = offset.saturating_add(len);
        run = match run {
            Some((run_start, run_end)) if offset <= run_end => {
                Some((run_start, run_end.max(range_end)))
            }
            Some((run_start, run_end)) => {
                covered += u128::from(run_end - run_start);
                Some((offset, range_end))
            }
            None => Some((offset, range_end)),
        };
    }
    if let Some((run_start, run_end)) = run {
        covered += u128::from(run_end - run_start);
    }
    if covered * 2 >= u128::from(span_len) {
        HintPlan::Dense((start, span_len))
    } else {
        HintPlan::Scattered(distinct)
    }
}
fn locate_hints(index: &dyn Index, ids: &[EntryId]) -> Vec<Option<Span>> {
    let none = vec![None; ids.len()];
    let (Some(min), Some(max)) = (ids.iter().min().copied(), ids.iter().max().copied())
    else {
        return none;
    };
    if !index.access_methods().contains(&AccessMethod::Range) {
        return none;
    }
    let ranges = match index.byte_ranges(min, max) {
        Ok(ranges) => ranges,
        Err(err) => {
            warn!("byte range lookup failed: {err}");
            return none;
        }
    };
    ids.iter()
        .map(|id| {
            let pos = ranges.partition_point(|r| r.first.0 <= id.0);
            let range: &ByteRange = ranges.get(pos.checked_sub(1)?)?;
            (id.0 < range.first.0.saturating_add(range.count))
                .then_some((range.offset, range.len))
        })
        .collect()
}
struct Inner {
    index: LazyIndex,
    advisor: Option<Arc<dyn MemoryAdvisor>>,
    idle_after: Duration,
    tracked: Mutex<HashMap<Span, Instant>>,
}
impl Inner {
    fn tracked(&self) -> MutexGuard<'_, HashMap<Span, Instant>> {
        self.tracked.lock().unwrap_or_else(|e| e.into_inner())
    }
    fn hint(&self, offset: u64, len: u64, advice: Advice) {
        let Some(advisor) = &self.advisor else {
            return;
        };
        if len == 0 {
            return;
        }
        if let Err(err) = advisor.advise(offset, len, advice) {
            warn!("memory advice {advice:?} for [{offset}, +{len}) failed: {err}");
        }
        self.tracked().insert((offset, len), Instant::now());
    }
    fn apply(&self, plan: &HintPlan) {
        match plan {
            HintPlan::None => {}
            HintPlan::Dense((offset, len)) => {
                self.hint(*offset, *len, Advice::Sequential)
            }
            HintPlan::Scattered(spans) => {
                for &(offset, len) in spans {
                    self.hint(offset, len, Advice::Random);
                }
            }
        }
    }
    fn read_all(index: &dyn Index, ids: &[EntryId]) -> Vec<OwnedPayload> {
        ids.iter().map(|&id| index.read(id).into()).collect()
    }
}
fn not_attached() -> TransferError {
    TransferError(MSG_NOT_ATTACHED.to_string())
}
fn join_failed<E>(_: E) -> TransferError {
    TransferError(MSG_TASK_ABORTED.to_string())
}
/// Keeps file-backed index data resident and reads entries through the blocking pool.
pub struct CpuManager {
    inner: Arc<Inner>,
}
impl CpuManager {
    /// `advisor = None`: hints are no-ops. `idle_after`: idle threshold for cold marking.
    pub fn new(
        index: LazyIndex,
        advisor: Option<Arc<dyn MemoryAdvisor>>,
        idle_after: Duration,
    ) -> CpuManager {
        CpuManager {
            inner: Arc::new(Inner {
                index,
                advisor,
                idle_after,
                tracked: Mutex::new(HashMap::new()),
            }),
        }
    }
    fn index(&self) -> Result<Arc<dyn Index>, TransferError> {
        self.inner.index.get().cloned().ok_or_else(not_attached)
    }
    /// Synchronous pass-through to the index on the calling thread; `None` if not attached.
    pub fn read_direct(&self, id: EntryId) -> Option<Payload> {
        self.inner.index.get().map(|index| index.read(id))
    }
    /// Hints `WillNeed` for `range` and tracks it as used now.
    pub fn secure_residency(&self, range: ByteRange) {
        self.inner.hint(range.offset, range.len, Advice::WillNeed);
    }
    /// Marks tracked ranges idle for at least `idle_after` as cold and stops tracking them.
    pub fn mark_idle_cold(&self, now: Instant) -> usize {
        let Some(advisor) = &self.inner.advisor else {
            return 0;
        };
        let idle: Vec<Span> = {
            let mut tracked = self.inner.tracked();
            let idle: Vec<Span> = tracked
                .iter()
                .filter(|(_, used)| {
                    now.saturating_duration_since(**used) >= self.inner.idle_after
                })
                .map(|(span, _)| *span)
                .collect();
            for span in &idle {
                tracked.remove(span);
            }
            idle
        };
        for &(offset, len) in &idle {
            if let Err(err) = advisor.advise(offset, len, Advice::Cold) {
                warn!("cold advice for [{offset}, +{len}) failed: {err}");
            }
        }
        idle.len()
    }
    /// Reads a single entry; `hint` is the entry's hint range, if known.
    pub async fn read_entry(
        &self,
        id: EntryId,
        hint: Option<ByteRange>,
    ) -> Result<OwnedPayload, TransferError> {
        let index = self.index()?;
        if let Some(range) = hint {
            self.inner.hint(range.offset, range.len, Advice::WillNeed);
        }
        tokio::task::spawn_blocking(move || OwnedPayload::from(index.read(id)))
            .await
            .map_err(join_failed)
    }
    /// Reads several entries in input order with an access pattern derived from their hints.
    pub async fn read_entries(
        &self,
        entries: &[(EntryId, Option<ByteRange>)],
    ) -> Result<Vec<OwnedPayload>, TransferError> {
        if entries.is_empty() {
            return Ok(Vec::new());
        }
        let index = self.index()?;
        let spans: Vec<Option<Span>> = entries
            .iter()
            .map(|(_, hint)| hint.map(|r| (r.offset, r.len)))
            .collect();
        self.inner.apply(&plan_hints(&spans));
        let ids: Vec<EntryId> = entries.iter().map(|(id, _)| *id).collect();
        tokio::task::spawn_blocking(move || Inner::read_all(index.as_ref(), &ids))
            .await
            .map_err(join_failed)
    }
    /// Like `read_entries`, but the hint ranges are requested from the index.
    pub async fn read_entries_auto(
        &self,
        ids: &[EntryId],
    ) -> Result<Vec<OwnedPayload>, TransferError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let index = self.index()?;
        let inner = Arc::clone(&self.inner);
        let ids = ids.to_vec();
        tokio::task::spawn_blocking(move || {
                let spans = locate_hints(index.as_ref(), &ids);
                inner.apply(&plan_hints(&spans));
                Inner::read_all(index.as_ref(), &ids)
            })
            .await
            .map_err(join_failed)
    }
}
#[cfg(test)]
#[path = "test/cpu_manager.rs"]
mod tests;
