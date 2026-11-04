use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use super::super::calibrate::{CalibrationTarget, Entry, Stage};
use super::super::measure::{Cfg, Validity};
use super::*;

static COUNTER: AtomicUsize = AtomicUsize::new(0);

fn temp_file(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "xyrag-store-{}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir_all(&dir).unwrap();
    dir.join(name)
}

fn fingerprint(driver: u32) -> Fingerprint {
    let gpu = GpuIdentity { name: "gpu".into(), compute_capability: (7, 5), driver_version: driver };
    Fingerprint::new(
        SubengineKind::Cuda,
        "model",
        Some(gpu),
        CpuFeatures { avx2: true, f16c: true, fma: false, vnni: false },
    )
}

fn config(value: &str) -> ProfileConfig {
    ProfileConfig::new().with("operands", value)
}

fn record(driver: u32) -> CalibrationRecord {
    CalibrationRecord {
        fingerprint: fingerprint(driver),
        profiles: vec![StoredProfile::from(&Profile::new(0.25, config("fp16")).unwrap())],
        unstable: true,
        rejections: vec![RejectedCombo { config: config_map(&config("int8")), reason: "deviation above maximum".into() }],
        pruning: vec![PrunedCombo { config: config_map(&config("bf16")), reason: "dominated".into() }],
    }
}

#[test]
fn fingerprint_without_gpu_has_empty_gpu_fields() {
    let fp = Fingerprint::new(SubengineKind::Cpu, "m", None, CpuFeatures::default());
    assert_eq!((fp.gpu_name, fp.gpu_cc, fp.cuda_driver_version), (None, None, None));
    assert!(fp.cpu_features.is_empty());
    assert_eq!(fp.engine, "cpu");
}

#[test]
fn cpu_features_keep_fixed_order() {
    let all = CpuFeatures { avx2: true, f16c: true, fma: true, vnni: true };
    let fp = Fingerprint::new(SubengineKind::Cpu, "m", None, all);
    assert_eq!(fp.cpu_features, ["avx2", "f16c", "fma", "vnni"]);
}

#[test]
fn current_fingerprint_without_device_is_valid() {
    let fp = Fingerprint::current(SubengineKind::Cpu, "m", None);
    assert_eq!(fp.schema_version, SCHEMA_VERSION);
    assert_eq!(fp.gpu_name, None);
}

#[test]
fn equal_fingerprint_is_calibrated() {
    assert_eq!(state(Some(&record(13030)), &fingerprint(13030)), StoreState::Calibrated);
}

#[test]
fn changed_driver_version_is_stale_and_record_stays_usable() {
    let store = FileCalibrationStore::new(temp_file("c.json"));
    store.save_atomic(&record(13030)).unwrap();
    let resolution = resolve(&store, &fingerprint(13040));
    assert_eq!(resolution.state, StoreState::Stale);
    assert!(resolution.needs_calibration());
    assert_eq!(resolution.record.unwrap().active_profiles().unwrap().len(), 1);
}

#[test]
fn other_schema_version_is_stale() {
    let mut other = fingerprint(13030);
    other.schema_version += 1;
    assert_eq!(state(Some(&record(13030)), &other), StoreState::Stale);
}

#[test]
fn missing_file_is_missing() {
    let store = FileCalibrationStore::new(temp_file("none.json"));
    assert_eq!(store.load().unwrap(), None);
    let resolution = resolve(&store, &fingerprint(1));
    assert_eq!(resolution.state, StoreState::Missing);
    assert!(resolution.record.is_none());
}

#[test]
fn corrupt_file_is_treated_as_missing() {
    let path = temp_file("bad.json");
    fs::write(&path, b"{ not json").unwrap();
    let store = FileCalibrationStore::new(path);
    assert!(matches!(store.load(), Err(PrecisionError::Store { .. })));
    assert_eq!(resolve(&store, &fingerprint(1)).state, StoreState::Missing);
}

#[test]
fn round_trip_keeps_all_fields() {
    let store = FileCalibrationStore::new(temp_file("rt.json"));
    let saved = record(13030);
    store.save_atomic(&saved).unwrap();
    let loaded = store.load().unwrap().unwrap();
    assert_eq!(loaded, saved);
    assert!(loaded.unstable);
    assert_eq!(loaded.rejections.len(), 1);
    assert_eq!(loaded.pruning.len(), 1);
    assert_eq!(resolve(&store, &fingerprint(13030)).state, StoreState::Calibrated);
}

#[test]
fn profile_round_trips_through_stored_form() {
    let profile = Profile::new(0.75, config("fp32").with("scoring", "dot")).unwrap();
    assert_eq!(StoredProfile::from(&profile).to_profile().unwrap(), profile);
}

#[test]
fn out_of_range_stored_position_is_rejected() {
    let stored = StoredProfile { position: 1.5, config: BTreeMap::new() };
    assert!(matches!(stored.to_profile(), Err(PrecisionError::InvalidPrecision { .. })));
}

#[test]
fn unknown_and_missing_fields_are_tolerated() {
    let path = temp_file("fwd.json");
    fs::write(&path, br#"{"unstable": true, "future_field": [1, 2], "fingerprint": {"model_id": "m", "extra": 1}}"#).unwrap();
    let loaded = FileCalibrationStore::new(path).load().unwrap().unwrap();
    assert!(loaded.unstable);
    assert!(loaded.profiles.is_empty());
    assert_eq!(loaded.fingerprint.model_id, "m");
    assert_eq!(loaded.fingerprint.schema_version, 0);
}

#[test]
fn failed_write_leaves_old_record_intact() {
    let store = FileCalibrationStore::new(temp_file("atomic.json"));
    store.save_atomic(&record(1)).unwrap();
    fs::create_dir(store.temp_path()).unwrap();
    assert!(store.save_atomic(&record(2)).is_err());
    assert_eq!(store.load().unwrap().unwrap().fingerprint.cuda_driver_version, Some(1));
}

#[test]
fn successful_save_leaves_no_temp_file() {
    let store = FileCalibrationStore::new(temp_file("clean.json"));
    store.save_atomic(&record(1)).unwrap();
    assert!(!store.temp_path().exists());
}

#[test]
fn save_creates_missing_directories() {
    let path = temp_file("x.json").with_file_name("nested").join("deeper").join("c.json");
    let store = FileCalibrationStore::new(path);
    store.save_atomic(&record(1)).unwrap();
    assert!(store.load().unwrap().is_some());
}

fn entry(value: &str, outcome: Outcome) -> Entry {
    Entry { stage: Stage::Axis, cfg: Cfg::new(config(value), vec![4]), outcome }
}

#[test]
fn record_from_report_collects_rejections_and_pruning() {
    let report = CalibrationReport {
        target: CalibrationTarget { engine: SubengineKind::Cuda, model_id: "model".into() },
        entries: vec![
            entry("int8", Outcome::Rejected(Validity::OverMaxDeviation)),
            entry("bf16", Outcome::Dominated),
            entry("fp16", Outcome::Skipped),
        ],
        front: Vec::new(),
        budget_exhausted: false,
        exhaustive_size: 3,
        elapsed: Duration::ZERO,
    };
    let profile = Profile::new(0.5, config("fp32")).unwrap();
    let rec = CalibrationRecord::from_report(fingerprint(1), &report, &[profile]);
    assert_eq!(rec.rejections.len(), 1);
    assert_eq!(rec.rejections[0].reason, "deviation above maximum");
    assert_eq!(rec.rejections[0].config["operands"], "int8");
    assert_eq!(rec.pruning.iter().map(|p| p.reason.as_str()).collect::<Vec<_>>(), ["dominated", "skipped"]);
    assert_eq!(rec.profiles.len(), 1);
    assert!(!rec.unstable);
}

#[test]
fn background_recalibration_replaces_record() {
    let store = Arc::new(FileCalibrationStore::new(temp_file("bg.json")));
    store.save_atomic(&record(1)).unwrap();
    let pool = LowPriorityPool::new("test-calibration", 1).unwrap();
    let handle = recalibrate_in_background(&pool, store.clone(), || Ok(record(2)));
    assert!(handle.join().unwrap().is_ok());
    assert_eq!(store.load().unwrap().unwrap().fingerprint.cuda_driver_version, Some(2));
}

#[test]
fn failed_calibration_keeps_old_record() {
    let store = Arc::new(FileCalibrationStore::new(temp_file("bgfail.json")));
    store.save_atomic(&record(1)).unwrap();
    let pool = LowPriorityPool::new("test-calibration", 1).unwrap();
    let handle = recalibrate_in_background(&pool, store.clone(), || {
        Err(PrecisionError::CalibrationUnstable { reason: "never stable".into() })
    });
    assert!(handle.join().unwrap().is_err());
    assert_eq!(store.load().unwrap().unwrap().fingerprint.cuda_driver_version, Some(1));
}
