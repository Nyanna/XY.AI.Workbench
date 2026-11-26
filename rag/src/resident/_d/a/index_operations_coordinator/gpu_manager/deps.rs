//! Project elements the GPU manager relies on.
pub(super) use super::super::eviction::{
    EvictKey, Eviction, EvictionError, EvictionOutcome, Tier, TrackSpec,
};
pub(super) use super::load_aggregation::{load_aggregated, LoadRequest};
pub(super) use super::scope_resolution::{
    CellSelector, ResolveContext, ResolveError, ScopeResolverRegistry,
};
pub(super) use crate::resident::_d::_c::manager::LazyIndex;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::{Event, Lane};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::scopes_and_entry_sets::{EntrySet, Scope};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::lock;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::arenas::ArenaKind;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::{
    AccessMethod, ByteRange, CellId, EntryId, Index, IndexError, NodeId, Payload,
    PayloadRef, Stamp, StampObserver, Summary,
};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::kernel_layout::{Addr, Slot};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::RegionHandle;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::allocator::Allocator;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::slot_layout::{
    slot_size, EntryRef, SegmentView, SlotAssignment, SlotClass, SlotError, SlotStates,
};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::segments::{SegmentId, SlotRef};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::table_swap::{Epoch, SegmentTable, TableSwap, TableSwapError};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::address_map::{AddressMap, Entry, MapError, MappingStore};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::consistency::IndexWriter;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::segment_arena::{
    Readers, Segment, SegmentArena, SegmentArenaError,
};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::manifest::{Manifest, ManifestError, ManifestStore, SegmentSpec};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::reconcile::{
    load_entry, Reconciler, Reconciliation, Resident, ResidencyDirectory,
};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::a::communication_slot::hit_resolution::{
    SegmentBinding, SegmentDirectory,
};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::a::communication_slot::{
    MappingSnapshot, MappingSource, StructureHold, UpdateApplier, UpdateOutcome,
};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::a::embedding_output::segment_writer::{
    Reservation, SegmentWriter, WriteError,
};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::a::update_queue::{UpdateKind, UpdateQueue};
pub(super) use crate::resident::_d::_c::_b::_a::_9::a::gpu_kernel::{encode_segment_table, SegmentEntry};
