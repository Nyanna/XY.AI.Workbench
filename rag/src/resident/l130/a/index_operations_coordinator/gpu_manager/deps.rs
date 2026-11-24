//! Project elements the GPU manager relies on.
use crate::resident::l130::l120::l110::l100::l090 as l090;
use l090::l080 as l080;
use l080::l070 as l070;
use l070::l060 as l060;
use l060::l050 as l050;
use l050::l040 as l040;
use l040::l030 as l030;
use l030::l020 as l020;
use l020::l010 as l010;
pub(super) use super::super::eviction::{
    EvictKey, Eviction, EvictionError, EvictionOutcome, Tier, TrackSpec,
};
pub(super) use super::load_aggregation::{load_aggregated, LoadRequest};
pub(super) use super::scope_resolution::{
    CellSelector, ResolveContext, ResolveError, ScopeResolverRegistry,
};
pub(super) use crate::resident::l130::l120::manager::LazyIndex;
pub(super) use l010::device_lane_event::{Event, Lane};
pub(super) use l010::layout::DType;
pub(super) use l010::scopes_and_entry_sets::{EntrySet, Scope};
pub(super) use l010::sync_util::lock;
pub(super) use l020::a::arenas::ArenaKind;
pub(super) use l020::a::index_contract::{
    AccessMethod, ByteRange, CellId, EntryId, Index, IndexError, NodeId, Payload,
    PayloadRef, Stamp, StampObserver, Summary,
};
pub(super) use l030::a::kernel_layout::{Addr, Slot};
pub(super) use l030::a::region_handle::RegionHandle;
pub(super) use l040::a::allocator::Allocator;
pub(super) use l040::a::slot_layout::{
    slot_size, EntryRef, SegmentView, SlotAssignment, SlotClass, SlotError, SlotStates,
};
pub(super) use l050::a::segments::{SegmentId, SlotRef};
pub(super) use l050::a::table_swap::{Epoch, SegmentTable, TableSwap, TableSwapError};
pub(super) use l060::a::address_map::{AddressMap, Entry, MapError, MappingStore};
pub(super) use l060::a::consistency::IndexWriter;
pub(super) use l060::a::segment_arena::{
    Readers, Segment, SegmentArena, SegmentArenaError,
};
pub(super) use l070::a::manifest::{Manifest, ManifestError, ManifestStore, SegmentSpec};
pub(super) use l070::a::reconcile::{
    load_entry, Reconciler, Reconciliation, Resident, ResidencyDirectory,
};
pub(super) use l080::a::communication_slot::hit_resolution::{
    SegmentBinding, SegmentDirectory,
};
pub(super) use l080::a::communication_slot::{
    MappingSnapshot, MappingSource, StructureHold, UpdateApplier, UpdateOutcome,
};
pub(super) use l080::a::embedding_output::segment_writer::{
    Reservation, SegmentWriter, WriteError,
};
pub(super) use l080::a::update_queue::{UpdateKind, UpdateQueue};
pub(super) use l090::a::gpu_kernel::{encode_segment_table, SegmentEntry};
