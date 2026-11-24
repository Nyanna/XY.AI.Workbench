//! Writing the kernel segment table to VRAM.

use super::deps::*;
use super::types::*;
use super::GpuManager;

impl GpuManager {
    /// Writes the segment table if the published records changed since the last sync.
    /// The caller syncs on a lane before it starts work on that lane.
    pub fn sync_to_vram(&self, lane: &mut Lane) -> Result<VramTable, GpuManagerError> {
        let inner = &*self.inner;
        let mut cache = lock(&inner.sync);
        let (version, entries) = {
            let state = lock(&inner.state);
            let entries: Vec<SegmentEntry> = state
                .records
                .iter()
                .map(|r| SegmentEntry {
                    base: r.device_base.0,
                    lens_addr: 0,
                    stride_bytes: u64::from(r.view.slot_bytes()),
                    first_slot: r.first_slot,
                    slot_count: r.view.slot_count(),
                    pad_len: r.view.class().max_tokens(),
                    _reserved: 0,
                })
                .collect();
            (state.table_version, entries)
        };
        if let Some(table) = *cache {
            if table.version == version {
                return Ok(table);
            }
        }
        let table = if entries.is_empty() {
            VramTable { segments_addr: 0, segment_count: 0, version }
        } else {
            let bytes = encode_segment_table(&entries);
            let segments_addr = inner
                .services
                .write_kernel_table(lane, &bytes)
                .map_err(GpuManagerError::VramSync)?;
            VramTable { segments_addr, segment_count: entries.len() as u32, version }
        };
        *cache = Some(table);
        Ok(table)
    }
}

#[cfg(test)]
#[path = "test/vram_sync.rs"]
mod tests;
