use super::*;

fn invalid(cfg: &GpuConfig) -> String {
    match cfg.validate() {
        Err(GpuError::InvalidConfig(text)) => text,
        other => panic!("expected InvalidConfig, got {other:?}"),
    }
}

fn lifecycle(ram: u64, vram: u64) -> LifecycleConfig {
    LifecycleConfig::new(Duration::from_secs(ram), Duration::from_secs(vram), Duration::from_secs(5))
        .unwrap()
}

#[test]
fn defaults_are_valid_and_fp16_first() {
    let cfg = GpuConfig::default();
    assert!(cfg.validate().is_ok());
    assert_eq!(cfg.default_dtype, DType::F16);
    assert_eq!(cfg.headroom, DEFAULT_HEADROOM);
    assert_eq!(cfg.generations, 1);
    assert_eq!(cfg.worker_lanes, 2);
    assert_eq!(cfg.upload_batch, 256);
    assert_eq!(cfg.lane_workspace_bytes, 0);
    assert_eq!(cfg.lifecycle.ram_ttl(), Duration::from_secs(60));
    assert_eq!(cfg.lifecycle.vram_ttl(), Duration::from_secs(600));
    assert_eq!(cfg.lifecycle.backoff(), Duration::from_secs(5));
    assert_eq!(cfg.ram_init_deadline, Duration::from_secs(30));
    assert_eq!(cfg.vram_init_deadline, Duration::from_secs(300));
    assert_eq!(cfg.arenas, ArenaCapacities::default());
}

#[test]
fn only_f16_and_f32_are_accepted_as_default_dtype() {
    let mut cfg = GpuConfig::default();
    cfg.default_dtype = DType::F32;
    assert!(cfg.validate().is_ok());
    for dtype in [DType::BF16, DType::U8, DType::U32, DType::I64] {
        cfg.default_dtype = dtype;
        assert!(invalid(&cfg).contains("dtype"), "{dtype:?}");
    }
}

#[test]
fn lanes_and_batches_must_be_positive() {
    let mut cfg = GpuConfig::default();
    cfg.worker_lanes = 0;
    assert!(invalid(&cfg).contains("worker_lanes"));
    cfg.worker_lanes = 1;
    assert!(cfg.validate().is_ok());
    cfg.upload_batch = 0;
    assert!(invalid(&cfg).contains("upload_batch"));
    cfg.upload_batch = 1;
    assert!(cfg.validate().is_ok());
}

#[test]
fn vram_ttl_must_be_at_least_twice_the_ram_ttl() {
    let mut cfg = GpuConfig::default();
    cfg.lifecycle = lifecycle(60, 119);
    assert!(invalid(&cfg).contains("vram_ttl"));
    cfg.lifecycle = lifecycle(60, 120);
    assert!(cfg.validate().is_ok());
}

#[test]
fn vram_deadline_must_be_at_least_twice_the_ram_deadline() {
    let mut cfg = GpuConfig::default();
    cfg.ram_init_deadline = Duration::from_secs(30);
    cfg.vram_init_deadline = Duration::from_secs(59);
    assert!(invalid(&cfg).contains("vram_init_deadline"));
    cfg.vram_init_deadline = Duration::from_secs(60);
    assert!(cfg.validate().is_ok());
}

#[test]
fn the_first_violation_wins() {
    let mut cfg = GpuConfig::default();
    cfg.default_dtype = DType::BF16;
    cfg.worker_lanes = 0;
    cfg.upload_batch = 0;
    cfg.lifecycle = lifecycle(60, 61);
    cfg.vram_init_deadline = Duration::from_secs(1);
    assert!(invalid(&cfg).contains("dtype"));
    cfg.default_dtype = DType::F16;
    assert!(invalid(&cfg).contains("worker_lanes"));
    cfg.worker_lanes = 1;
    assert!(invalid(&cfg).contains("upload_batch"));
    cfg.upload_batch = 1;
    assert!(invalid(&cfg).contains("vram_ttl"));
    cfg.lifecycle = lifecycle(60, 600);
    assert!(invalid(&cfg).contains("vram_init_deadline"));
}

#[test]
fn device_manager_config_defaults_to_all_devices() {
    let cfg = GpuDeviceManagerConfig::default();
    assert!(cfg.devices.is_none());
    assert!(cfg.gpu.validate().is_ok());
}

#[test]
fn default_kernel_names_map_the_scorer_only() {
    let names = default_kernel_names();
    assert_eq!(names.len(), 1);
    assert_eq!(names.get(&ResourceKind::Scorer).map(String::as_str), Some("score_dot"));
}
