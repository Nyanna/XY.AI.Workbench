//! Placement of entries into slots and creation of segments.
use std::sync::Arc;
use super::deps::*;
use super::state::{entry_ref, Inner, Unpublished};
use super::types::*;
#[derive(Clone, Copy)]
enum Target {
    Published(SegmentId),
    Unpublished(RegionHandle),
}
struct Reserved {
    target: Target,
    writer: Arc<SegmentWriter>,
    view: SegmentView,
    reservation: Reservation,
    in_flight: bool,
}
enum Reserve {
    Present,
    Got(Box<Reserved>),
    NoRoom,
}
/// A slot that holds the entry.
pub(super) struct Placed {
    pub bytes: u64,
    /// The segment if it is published.
    pub segment: Option<SegmentId>,
}
pub(super) enum Placement {
    Placed(Placed),
    AlreadyPresent,
}
fn decode_rows(dtype: DType, bytes: &[u8]) -> Vec<f32> {
    match dtype {
        DType::F16 => {
            bytes
                .chunks_exact(2)
                .map(|c| half::f16::from_le_bytes([c[0], c[1]]).to_f32())
                .collect()
        }
        DType::BF16 => {
            bytes
                .chunks_exact(2)
                .map(|c| half::bf16::from_le_bytes([c[0], c[1]]).to_f32())
                .collect()
        }
        _ => {
            bytes
                .chunks_exact(4)
                .map(|c| f32::from_le_bytes([c[0], c[1], c[2], c[3]]))
                .collect()
        }
    }
}
impl Inner {
    /// Token count of a vector, or the reason it does not fit a slot.
    pub(super) fn validate_vector(&self, len: usize) -> Result<u32, GpuManagerError> {
        if len % self.row_bytes != 0 {
            return Err(GpuManagerError::InvalidVector {
                len,
                row_bytes: self.row_bytes,
            });
        }
        let tokens = u32::try_from(len / self.row_bytes).unwrap_or(u32::MAX);
        let max = self.cfg.class.max_tokens();
        if tokens > max {
            return Err(GpuManagerError::TooManyTokens {
                tokens,
                max,
            });
        }
        Ok(tokens)
    }
    /// Places the entry in a new slot. The caller holds the placement lock.
    ///
    /// With `recheck` an entry that is in flight or waiting for its bind counts as present.
    pub(super) fn place(
        &self,
        id: EntryId,
        vector: &[u8],
        tokens: u32,
        stamp: Stamp,
        recheck: bool,
    ) -> Result<Placement, GpuManagerError> {
        let entry = entry_ref(id);
        if recheck && self.is_waiting(id) {
            return Ok(Placement::AlreadyPresent);
        }
        let reserved = match self.reserve(entry, tokens, recheck)? {
            Reserve::Present => return Ok(Placement::AlreadyPresent),
            Reserve::Got(reserved) => reserved,
            Reserve::NoRoom => {
                self.create_segment()?;
                match self.reserve(entry, tokens, recheck)? {
                    Reserve::Got(reserved) => reserved,
                    Reserve::Present => return Ok(Placement::AlreadyPresent),
                    Reserve::NoRoom => {
                        return Err(GpuManagerError::SlotWrite(WriteError::NoFreeSlot));
                    }
                }
            }
        };
        if let Err(e) = self.write_slot(&reserved, vector, tokens) {
            if reserved.in_flight {
                lock(&self.state).in_flight.remove(&id);
            }
            return Err(e);
        }
        let slot = reserved.reservation.slot;
        let segment = match reserved.target {
            Target::Published(segment) => {
                self.services
                    .updates()
                    .push_bind(entry, SlotRef { segment, slot }, stamp);
                Some(segment)
            }
            Target::Unpublished(region) => {
                let mut state = lock(&self.state);
                if let Some(open) = state
                    .unpublished
                    .iter_mut()
                    .find(|u| u.segment.region == region)
                {
                    open.bindings
                        .push(LoadedBinding {
                            entry,
                            slot,
                            stamp,
                        });
                }
                None
            }
        };
        Ok(
            Placement::Placed(Placed {
                bytes: self.slot_bytes,
                segment,
            }),
        )
    }
    /// Resident entries are not checked here; in flight or a pending Bind is.
    fn is_waiting(&self, id: EntryId) -> bool {
        let binding = matches!(
            self.services.updates().get(id), Some(u) if u.kind == UpdateKind::Bind
        );
        binding || lock(&self.state).in_flight.contains(&id)
    }
    fn reserve(
        &self,
        entry: EntryRef,
        tokens: u32,
        recheck: bool,
    ) -> Result<Reserve, GpuManagerError> {
        let mut state = lock(&self.state);
        if recheck && state.in_flight.contains(&entry.entry) {
            return Ok(Reserve::Present);
        }
        let mut found: Option<Reserved> = None;
        for open in state.unpublished.iter().rev() {
            if open.writer.free_slots() == 0 {
                continue;
            }
            match open.writer.reserve(entry, tokens, None) {
                Ok(reservation) => {
                    found = Some(Reserved {
                        target: Target::Unpublished(open.segment.region),
                        writer: Arc::clone(&open.writer),
                        view: open.view,
                        reservation,
                        in_flight: true,
                    });
                    break;
                }
                Err(WriteError::NoFreeSlot) => {}
                Err(e) => return Err(GpuManagerError::SlotWrite(e)),
            }
        }
        if found.is_none() {
            for record in state.records.iter().rev() {
                let Some(writer) = &record.writer else {
                    continue;
                };
                if writer.free_slots() == 0 {
                    continue;
                }
                match writer.reserve(entry, tokens, None) {
                    Ok(reservation) => {
                        found = Some(Reserved {
                            target: Target::Published(record.id),
                            writer: Arc::clone(writer),
                            view: record.view,
                            reservation,
                            in_flight: false,
                        });
                        break;
                    }
                    Err(WriteError::NoFreeSlot) => {}
                    Err(e) => return Err(GpuManagerError::SlotWrite(e)),
                }
            }
        }
        Ok(
            match found {
                Some(reserved) => {
                    if reserved.in_flight {
                        state.in_flight.insert(entry.entry);
                    }
                    Reserve::Got(Box::new(reserved))
                }
                None => Reserve::NoRoom,
            },
        )
    }
    fn write_slot(
        &self,
        reserved: &Reserved,
        vector: &[u8],
        tokens: u32,
    ) -> Result<(), GpuManagerError> {
        let values = decode_rows(self.cfg.dtype, vector);
        let bytes = reserved
            .view
            .encode_slot(tokens as usize, &values)
            .map_err(GpuManagerError::Slot)?;
        let handle = reserved.view.region();
        let offset = reserved
            .reservation
            .target
            .start
            .checked_sub(handle.offset())
            .ok_or_else(|| GpuManagerError::Memory(
                "slot lies before its segment".into(),
            ))?;
        let event = self
            .services
            .write_visible(&handle, offset, &bytes)
            .map_err(GpuManagerError::Memory)?;
        event.wait().map_err(|e| GpuManagerError::Memory(e.to_string()))?;
        reserved
            .writer
            .commit(&reserved.reservation, &event)
            .map_err(GpuManagerError::SlotWrite)
    }
    /// Allocates a segment and registers it as unpublished. Nothing stays behind on failure.
    fn create_segment(&self) -> Result<(), GpuManagerError> {
        let arena = self.services.arena();
        let segment = arena
            .allocate(self.cfg.segment_bytes)
            .map_err(GpuManagerError::Arena)?;
        match self.build_unpublished(segment) {
            Ok(open) => {
                lock(&self.state).unpublished.push(open);
                Ok(())
            }
            Err(e) => {
                if let Err(release) = arena.release(&segment, Readers::None) {
                    log::warn!(
                        "gpu_manager: segment not released after failed setup: {release}"
                    );
                }
                Err(e)
            }
        }
    }
    fn build_unpublished(
        &self,
        segment: Segment,
    ) -> Result<Unpublished, GpuManagerError> {
        let handle = RegionHandle::new(
                ArenaKind::IndexSegments,
                segment.offset,
                segment.len,
                segment.region.generation(),
            )
            .map_err(|e| GpuManagerError::Memory(e.to_string()))?;
        let device_base = self
            .services
            .device_base(&handle)
            .ok_or_else(|| GpuManagerError::Memory(
                "segment has no device address".into(),
            ))?;
        let slot_count = segment.len / self.slot_bytes;
        if slot_count == 0 {
            return Err(
                GpuManagerError::InvalidConfig("segment is shorter than one slot".into()),
            );
        }
        let slot_count = u32::try_from(slot_count)
            .map_err(|_| GpuManagerError::SegmentLimit)?;
        let first_slot = lock(&self.state).next_first_slot;
        let end = first_slot + u64::from(slot_count);
        let first = u32::try_from(first_slot)
            .map_err(|_| GpuManagerError::SegmentLimit)?;
        if end > u64::from(u32::MAX) {
            return Err(GpuManagerError::SegmentLimit);
        }
        let view = SegmentView::new(
                handle,
                slot_count,
                self.cfg.class,
                self.cfg.dim,
                self.cfg.dtype,
                handle.generation(),
            )
            .map_err(GpuManagerError::Slot)?;
        let writer = Arc::new(
            SegmentWriter::new(view, device_base).map_err(GpuManagerError::SlotWrite)?,
        );
        lock(&self.state).next_first_slot = end;
        Ok(Unpublished {
            segment,
            view,
            device_base,
            states: writer.states(),
            assignment: writer.assignment(),
            writer,
            first_slot: first,
            bindings: Vec::new(),
        })
    }
    /// Reports every unpublished segment with a committed slot to the queue; they are closed
    /// for new slots until published. Returns their number.
    pub(super) fn report_unpublished(&self) -> usize {
        let ready = {
            let mut state = lock(&self.state);
            let (ready, open): (Vec<_>, Vec<_>) = std::mem::take(&mut state.unpublished)
                .into_iter()
                .partition(|u| !u.bindings.is_empty());
            state.unpublished = open;
            for segment in &ready {
                state.first_slots.insert(segment.segment.region, segment.first_slot);
            }
            ready
        };
        let count = ready.len();
        for open in ready {
            self.queue
                .report(LoadedSegment {
                    segment: open.segment,
                    view: open.view,
                    device_base: open.device_base,
                    states: open.states,
                    assignment: open.assignment,
                    bindings: open.bindings,
                    writer: Some(open.writer),
                });
        }
        count
    }
}
#[cfg(test)]
#[path = "test/placement.rs"]
mod tests;
