use std::sync::atomic::Ordering;

use super::super::fixture::{config, shared, MockBackend};
use super::*;

fn manager_cfg(devices: Option<Vec<u32>>) -> GpuDeviceManagerConfig {
    GpuDeviceManagerConfig {
        devices,
        gpu: config(),
    }
}

fn build(
    backend: &Arc<MockBackend>,
    devices: Option<Vec<u32>>,
) -> Result<GpuDeviceManager, GpuError> {
    GpuDeviceManager::new(
        Arc::clone(backend) as Arc<dyn GpuBackend3>,
        shared(),
        manager_cfg(devices),
    )
}

fn ordinals(manager: &GpuDeviceManager) -> Vec<u32> {
    manager.devices().iter().map(|d| d.ordinal).collect()
}

#[test]
fn all_devices_are_used_in_ascending_order() {
    let backend = MockBackend::new(&[2, 0, 1]);
    let manager = build(&backend, None).unwrap();
    assert_eq!(ordinals(&manager), vec![0, 1, 2]);
    let engines: Vec<u32> = manager.subengines().iter().map(|e| e.info().ordinal).collect();
    assert_eq!(engines, vec![0, 1, 2]);
    assert_eq!(manager.subengine(1).unwrap().name(), "gpu:1");
    assert!(manager.subengine(7).is_none());
}

#[test]
fn one_primary_context_is_retained_per_device() {
    let backend = MockBackend::new(&[0, 1]);
    let manager = build(&backend, None).unwrap();
    assert_eq!(backend.retain_count(0), 1);
    assert_eq!(backend.retain_count(1), 1);
    let _engines = manager.subengines();
    assert_eq!(lock(&backend.retained).len(), 2);
}

#[test]
fn construction_loads_nothing_and_reserves_no_memory() {
    let backend = MockBackend::new(&[0, 1]);
    let manager = build(&backend, None).unwrap();
    assert!(lock(&backend.reserved).is_empty());
    assert!(lock(&backend.streams).is_empty());
    for engine in manager.subengines() {
        assert!(!engine.core().is_active());
        assert!(!engine.vram().arenas_ready());
    }
}

#[test]
fn an_explicit_list_selects_and_deduplicates() {
    let backend = MockBackend::new(&[0, 1, 2]);
    let manager = build(&backend, Some(vec![2, 0, 2])).unwrap();
    assert_eq!(ordinals(&manager), vec![0, 2]);
    assert_eq!(backend.retain_count(0), 1);
    assert_eq!(backend.retain_count(1), 0);
    assert_eq!(backend.retain_count(2), 1);
}

#[test]
fn a_missing_ordinal_retains_nothing() {
    let backend = MockBackend::new(&[0, 1]);
    let err = build(&backend, Some(vec![0, 5])).err().unwrap();
    assert!(matches!(err, GpuError::InvalidConfig(_)), "{err:?}");
    assert!(lock(&backend.retained).is_empty());
}

#[test]
fn enumeration_errors_are_backend_errors() {
    let backend = MockBackend::single();
    backend.fail_devices.store(true, Ordering::SeqCst);
    let err = build(&backend, None).err().unwrap();
    assert!(matches!(err, GpuError::Backend(_)), "{err:?}");
}

#[test]
fn unusable_devices_are_skipped_when_all_devices_are_requested() {
    let backend = MockBackend::new(&[0, 1, 2]);
    lock(&backend.fail_retain).insert(0);
    lock(&backend.no_context).insert(2);
    let manager = build(&backend, None).unwrap();
    assert_eq!(ordinals(&manager), vec![1]);
    assert_eq!(backend.retain_count(0), 1);
    assert_eq!(backend.retain_count(1), 1);
}

#[test]
fn an_explicit_device_that_fails_returns_the_error() {
    let backend = MockBackend::new(&[0, 1]);
    lock(&backend.fail_retain).insert(1);
    let err = build(&backend, Some(vec![0, 1])).err().unwrap();
    assert!(matches!(err, GpuError::Backend(_)), "{err:?}");
    let backend = MockBackend::new(&[0, 1]);
    lock(&backend.no_context).insert(1);
    let err = build(&backend, Some(vec![1])).err().unwrap();
    assert!(matches!(err, GpuError::Capability(DetectError::NoDeviceContext)), "{err:?}");
}

#[test]
fn no_usable_device_is_an_error() {
    let backend = MockBackend::new(&[]);
    assert!(matches!(build(&backend, None).err().unwrap(), GpuError::NoDevice));
    let backend = MockBackend::new(&[0]);
    lock(&backend.fail_retain).insert(0);
    assert!(matches!(build(&backend, None).err().unwrap(), GpuError::NoDevice));
    let backend = MockBackend::new(&[0]);
    assert!(matches!(build(&backend, Some(vec![])).err().unwrap(), GpuError::NoDevice));
}

#[test]
fn the_configuration_is_validated_first() {
    let backend = MockBackend::single();
    let mut cfg = manager_cfg(None);
    cfg.gpu.worker_lanes = 0;
    let err = GpuDeviceManager::new(Arc::clone(&backend) as Arc<dyn GpuBackend3>, shared(), cfg)
        .err()
        .unwrap();
    assert!(matches!(err, GpuError::InvalidConfig(_)), "{err:?}");
    assert!(lock(&backend.retained).is_empty());
}

#[test]
fn every_subengine_carries_its_device_info() {
    let backend = MockBackend::new(&[0, 1]);
    let manager = build(&backend, None).unwrap();
    for (info, engine) in manager.devices().iter().zip(manager.subengines()) {
        assert_eq!(engine.info(), info);
        assert_eq!(engine.compute_units(), info.multiprocessors);
        assert_eq!(engine.device(), DeviceHandle::Gpu(info.ordinal));
    }
}
