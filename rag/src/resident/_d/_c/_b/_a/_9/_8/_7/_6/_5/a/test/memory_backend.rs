use super::*;

#[test]
fn rounded_len_rounds_up_to_granule() {
    assert_eq!(rounded_len(1, 1), Ok(ALLOC_GRANULE));
    assert_eq!(rounded_len(ALLOC_GRANULE, 1), Ok(ALLOC_GRANULE));
    assert_eq!(rounded_len(ALLOC_GRANULE + 1, 8), Ok(2 * ALLOC_GRANULE));
    assert_eq!(rounded_len(1000, MAX_ALIGN), Ok(1024));
}

#[test]
fn rounded_len_rejects_invalid_requests() {
    let bad = Err(BackendError::AllocationFailed);
    assert_eq!(rounded_len(0, 1), bad);
    assert_eq!(rounded_len(8, 0), bad);
    assert_eq!(rounded_len(8, 3), bad);
    assert_eq!(rounded_len(8, MAX_ALIGN * 2), bad);
    assert_eq!(rounded_len(u64::MAX, 1), bad);
    assert_eq!(rounded_len(u64::MAX - ALLOC_GRANULE + 2, 1), bad);
}

#[test]
fn rounded_len_handles_largest_valid_size() {
    let max = u64::MAX / ALLOC_GRANULE * ALLOC_GRANULE;
    assert_eq!(rounded_len(max, 1), Ok(max));
}

#[test]
fn display_texts() {
    assert_eq!(BackendError::AllocationFailed.to_string(), "Allokation fehlgeschlagen");
    assert_eq!(BackendError::StaleRegion.to_string(), "stale Region");
    assert_eq!(
        BackendError::DeviceMemoryUnavailable("no context".into()).to_string(),
        "Device-Speicher nicht verfügbar: no context"
    );
    let t = BackendError::Transfer {
        direction: TransferDirection::H2D,
        reason: "size mismatch".into(),
    };
    let text = t.to_string();
    assert!(text.contains("H2D") && text.contains("size mismatch"));
}

#[test]
fn allocator_errors_map_one_to_one() {
    let oom = AllocatorError::OutOfMemory {
        arena: ArenaKind::Weights,
        requested: 5,
        free: 3,
    };
    assert_eq!(
        BackendError::from(oom),
        BackendError::OutOfMemory {
            arena: ArenaKind::Weights,
            requested: 5,
            free: 3
        }
    );
    assert_eq!(
        BackendError::from(AllocatorError::AllocationFailed),
        BackendError::AllocationFailed
    );
    assert_eq!(
        BackendError::from(AllocatorError::StaleRegion),
        BackendError::StaleRegion
    );
}

#[test]
fn wrapped_errors_convert() {
    let access = AccessError::OutOfBounds {
        offset: 1,
        len: 2,
        region_len: 2,
    };
    assert_eq!(
        BackendError::from(access.clone()),
        BackendError::Access(access)
    );
    let adapter = AdapterError::Overflow;
    assert_eq!(
        BackendError::from(adapter.clone()),
        BackendError::Adapter(adapter)
    );
}
