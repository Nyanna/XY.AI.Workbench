//! Scope resolution and loading of entry sets into slots.

use std::time::Instant;

use super::deps::*;
use super::placement::Placement;
use super::types::*;
use super::GpuManager;

impl GpuManager {
    /// Resolves the scope through the resolver registry; there is no fallback to all entries.
    pub fn resolve_scope(&self, scope: &Scope) -> Result<EntrySet, GpuManagerError> {
        let inner = &*self.inner;
        let index = inner.index()?;
        let ctx = ResolveContext {
            index: Some(&*index),
            cell_selector: inner.cell_selector.as_deref(),
        };
        inner.resolver.resolve(scope, &ctx).map_err(GpuManagerError::Resolve)
    }

    /// Places the missing members of `set` into slots. Placed segments are reported to the
    /// `LoadedQueue`; publishing them is up to the coordinator.
    pub fn ensure(&self, set: &EntrySet) -> Result<EnsureReport, GpuManagerError> {
        let inner = &*self.inner;
        let index = inner.index()?;
        if index.dtype() != inner.cfg.dtype {
            return Err(GpuManagerError::DTypeMismatch {
                index: index.dtype(),
                configured: inner.cfg.dtype,
            });
        }
        let (present, missing) = inner.classify(set.entry_ids());
        let mut report = EnsureReport { already_resident: present, ..EnsureReport::default() };
        let loaded = if missing.is_empty() {
            Vec::new()
        } else {
            let request = LoadRequest {
                index: (),
                scope: Scope::Set(EntrySet::from_entry_ids(missing.iter().copied())),
            };
            let source = std::sync::Arc::clone(&index);
            let lookup = move |_: &()| Some(std::sync::Arc::clone(&source));
            let mut loads = load_aggregated(
                vec![request],
                &inner.resolver,
                &lookup,
                inner.cell_selector.as_deref(),
            );
            match loads.pop() {
                Some(load) => load.result.map_err(GpuManagerError::Resolve)?,
                None => Vec::new(),
            }
        };
        let mut exhausted = None;
        {
            let _place = lock(&inner.place_lock);
            for (id, entry) in loaded {
                let Some(entry) = entry else {
                    report.absent_in_index.push(id);
                    continue;
                };
                let vector: &[u8] = &entry.payload.vector;
                let tokens = match inner.validate_vector(vector.len()) {
                    Ok(tokens) => tokens,
                    Err(e) => {
                        report.failed.push((id, e));
                        continue;
                    }
                };
                match inner.place(id, vector, tokens, entry.stamp, true) {
                    Ok(Placement::Placed(placed)) => {
                        report.loaded.push(id);
                        report.bytes_moved += placed.bytes;
                    }
                    Ok(Placement::AlreadyPresent) => report.already_resident.push(id),
                    Err(e @ GpuManagerError::Arena(_)) => {
                        exhausted = Some(e);
                        break;
                    }
                    Err(e) => report.failed.push((id, e)),
                }
            }
            report.segments_created = inner.report_unpublished();
        }
        report.already_resident.sort();
        report.already_resident.dedup();
        self.touch_published(Instant::now());
        log::info!(
            "gpu_manager ensure: requested={} loaded={} bytes_moved={} segments_created={}",
            set.len(),
            report.loaded.len(),
            report.bytes_moved,
            report.segments_created
        );
        match exhausted {
            Some(e) => Err(e),
            None => Ok(report),
        }
    }

    fn touch_published(&self, now: Instant) {
        let ids: Vec<u64> = lock(&self.inner.state).records.iter().map(|r| r.id.0).collect();
        let eviction = self.inner.services.eviction();
        for id in ids {
            eviction.touch(EvictKey(id), now);
        }
    }
}

#[cfg(test)]
#[path = "test/ensure.rs"]
mod tests;
