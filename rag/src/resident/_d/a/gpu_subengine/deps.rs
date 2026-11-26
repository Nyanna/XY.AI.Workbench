pub(super) use super::super::index_operations_coordinator::eviction::{
    Eviction, TierBudget,
};
pub(super) use super::super::index_operations_coordinator::gpu_manager::scope_resolution::{
    CellSelector, ScopeResolverRegistry,
};
pub(super) use super::super::index_operations_coordinator::gpu_manager::{
    EnsureReport, GpuManager, GpuManagerConfig, GpuManagerError, GpuServices,
    LoadedQueue, StructureLock,
};
pub(super) use super::super::index_operations_coordinator::index_ops::{
    IndexOpError, IndexOperations, ManagedIndex,
};
pub(super) use crate::resident::_d::_c::_b::a::subengine_core::{
    ComputePath, LaneFactory, PathPolicy, QueueRegistrations, SubengineConfig,
    SubengineCore, SubengineError,
};
pub(super) use crate::resident::_d::_c::_b::_a::a::plan_contract::QueueKind;
pub(super) use crate::resident::_d::_c::_b::_a::a::planner::Subengine;
pub(super) use crate::resident::_d::_c::_b::_a::_9::a::lane_batching::{
    BatchMeta, BatchPayload, BatchRunner, LaneResource, LaneResourceConfig, LaneSetError,
};
pub(super) use crate::resident::_d::_c::_b::_a::_9::a::manager_lifecycle::IndexManager;
pub(super) use crate::resident::_d::_c::_b::_a::_9::a::model_registry::{LoadedEncoder, ModelRegistry};
pub(super) use crate::resident::_d::_c::_b::_a::_9::a::residency_control::{BackendFactory, IndexSpec, ResidencyBackend};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::a::capability_detection::{
    detect_capabilities, CapabilityConfig, DetectError, DetectedCapabilities, GpuProbe,
};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::a::kernel_scoring::KERNEL_SCORE_DOT;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::a::update_queue::UpdateQueue;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::kernel_register::{KernelProvider, KernelRegister, Variant};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::manifest::{ManifestStore, StoreConfig};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::a::reconcile::ResidencyDirectory;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::address_map::MappingStore;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::resource_lifecycle::{
    InitRequest, LifecycleConfig, LifecycleError, LoadTicket, MemoryClass, ResourceKey,
    ResourceRegistry, ResourceState,
};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::a::segment_arena::{SegmentArena, SegmentArenaConfig};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::a::table_swap::{RootSlot, TableSink, TableSwap, TableSwapError};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::allocator::{Allocator, AllocatorError, AllocatorSet};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::{
    HasResourceRequirements, IndexId, ModelId, ResourceKind, ResourceRequirements,
};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::bookkeeping::Bookkeeping;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::kernel_layout::Addr;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::region_handle::RegionHandle;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::weight_source::{
    LoadedWeights, WeightError, WeightFile, WEIGHT_ALIGN,
};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::arenas::{
    fit_check, Arena, ArenaCapacities, ArenaKind, ArenaSet, Budget, BudgetError,
    MemoryQuery, DEFAULT_HEADROOM,
};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::index_contract::EntryId;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::memory_access::AccessRegion;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::model_descriptor::{DeviceClass, ModelDescriptor};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::capability_model::{axes, values, CapabilitySet};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::{
    DeviceHandle, DeviceKind, Event, Lane, LaneConfig, LaneKind, Priority, StreamBackend,
};
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::scopes_and_entry_sets::EntrySet;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::sync_util::lock;
pub(super) use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::target_env::ComputeCapability;
