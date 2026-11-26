use super::*;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::a::calibration_run::{
    BudgetStatus, Counters, FrontPoint, ProbeError, ReportEntry, SkipReason, Summary3,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::a::model_descriptor::E5_SMALL_MODEL_ID;
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::layout::DType;
use std::sync::Condvar;
use std::time::Duration;
struct TempDir(PathBuf);
impl TempDir {
    fn new(tag: &str) -> TempDir {
        static N: AtomicU64 = AtomicU64::new(0);
        let p = std::env::temp_dir()
            .join(
                format!(
                    "calib_store_{tag}_{}_{}", std::process::id(), N.fetch_add(1,
                    Ordering::Relaxed)
                ),
            );
        let _ = fs::remove_dir_all(&p);
        TempDir(p)
    }
}
impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
        let _ = fs::remove_file(&self.0);
    }
}
struct Gate {
    open: Mutex<bool>,
    cv: Condvar,
}
impl Gate {
    fn new() -> Arc<Gate> {
        Arc::new(Gate {
            open: Mutex::new(false),
            cv: Condvar::new(),
        })
    }
    fn release(&self) {
        *lock(&self.open) = true;
        self.cv.notify_all();
    }
    fn pass(&self) {
        let g = lock(&self.open);
        let _ = self.cv.wait_timeout_while(g, Duration::from_secs(30), |o| !*o);
    }
}
#[derive(Default)]
struct TestBackend {
    gate: Option<Arc<Gate>>,
    fail: bool,
    panic: bool,
}
impl CalibrationBackend for TestBackend {
    fn deviation(&self, _: &Combination, _: &[usize]) -> Result<f64, ProbeError> {
        Ok(0.01)
    }
    fn time(
        &self,
        c: &Combination,
        _: &[usize],
        _: Duration,
        _: Duration,
    ) -> Result<Duration, ProbeError> {
        if self.panic {
            panic!("backend panic");
        }
        if self.fail {
            return Err(ProbeError::Fatal("boom".into()));
        }
        if let Some(g) = &self.gate {
            g.pass();
        }
        let fast = c.value(&axes::batch_size()).map(|v| v.as_str()) == Some("8");
        Ok(Duration::from_micros(if fast { 100 } else { 200 }))
    }
}
fn fingerprint(engine: &str) -> Fingerprint {
    Fingerprint {
        schema_version: SCHEMA_VERSION3,
        subengine: "cpu".into(),
        cpu_features: "avx2".into(),
        model_id: E5_SMALL_MODEL_ID.into(),
        engine_version: engine.into(),
        ..Fingerprint::default()
    }
}
fn caps() -> CapabilitySet {
    CapabilitySet::new()
        .with(axes::batch_size(), "1")
        .with(axes::batch_size(), "8")
        .with(axes::tile_size(), "16")
}
fn request(engine: &str, backend: TestBackend) -> CalibrationRequest {
    CalibrationRequest {
        fingerprint: fingerprint(engine),
        capabilities: caps(),
        model: ModelDescriptor::e5_small_repo_default(DType::F32),
        backend: Arc::new(backend),
        config: CalibrationConfig {
            backoff: Duration::ZERO,
            ..CalibrationConfig::default()
        },
    }
}
fn store(dir: &TempDir) -> CalibrationStore {
    let pool = Arc::new(BackgroundPool::new("calib-test", 1).unwrap());
    CalibrationStore::new(dir.0.clone(), pool)
}
fn cfg(pairs: &[(&str, &str)]) -> BTreeMap<Axis, Value> {
    pairs.iter().map(|(a, v)| (Name::new(a), Name::new(v))).collect()
}
fn comb(pairs: &[(&str, &str)]) -> Combination {
    Combination::from_values(cfg(pairs)).unwrap()
}
fn record(engine: &str) -> CalibrationRecord {
    CalibrationRecord {
        fingerprint: fingerprint(engine),
        profiles: vec![
            PrecisionProfile { position : 0.0, config : cfg(& [("batch_size", "8")]), },
            PrecisionProfile { position : 1.0, config : cfg(& [("batch_size", "1")]), },
        ],
        unstable: false,
        discarded: vec![],
        pruned: vec![],
    }
}
fn finish<T: Send + 'static>(f: JobFuture<T>) -> T {
    match f.wait_timeout(Duration::from_secs(60)) {
        Ok(r) => r.expect("job panicked"),
        Err(_) => panic!("job timed out"),
    }
}
fn leftover_tmp_files(dir: &Path) -> usize {
    fs::read_dir(dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter(|e| e.file_name().to_string_lossy().contains(".tmp."))
                .count()
        })
        .unwrap_or(0)
}
#[test]
fn store_is_send_and_sync() {
    fn check<T: Send + Sync>() {}
    check::<CalibrationStore>();
    check::<CalibrationRequest>();
}
#[test]
fn encode_has_specified_layout() {
    let mut r = record("1.0");
    r.unstable = true;
    r.discarded
        .push(Annotated {
            combination: comb(&[("batch_size", "1"), ("tile_size", "16")]),
            reason: "non-finite deviation".into(),
        });
    r.pruned
        .push(Annotated {
            combination: comb(&[]),
            reason: "dominated".into(),
        });
    let text = encode_record(&r);
    let expected = format!(
        "# kalibrierung\nschema_version=1\nsubengine=cpu\ngpu_name=\ncompute_capability=\n\
         driver_version=\ncpu_features=avx2\nmodel_id=intfloat/multilingual-e5-small\n\
         engine_version=1.0\nunstable=true\nprofile=0|batch_size=8\nprofile=1|batch_size=1\n\
         discarded=batch_size=1;tile_size=16|non-finite deviation\npruned=|dominated\n"
    );
    assert_eq!(text, expected);
}
#[test]
fn roundtrip_with_special_characters() {
    let mut r = record("v|1;2=3%4\n\r5");
    r.fingerprint.gpu_name = "GTX 1660 | SUPER = é".into();
    r.unstable = true;
    r.profiles = vec![
        PrecisionProfile { position : 0.0, config : cfg(& [("a|b", "c;d"), ("e=f",
        "g%h")]), }, PrecisionProfile { position : 1.0 / 3.0, config : cfg(& []), },
        PrecisionProfile { position : 1.0, config : cfg(& [("x", "")]), },
    ];
    r.discarded
        .push(Annotated {
            combination: comb(&[("k|", "v;")]),
            reason: "failed: a|b=c\nd".into(),
        });
    r.pruned
        .push(Annotated {
            combination: comb(&[]),
            reason: "dominated".into(),
        });
    let text = encode_record(&r);
    let back = decode_record(&text).expect("decodes");
    assert_eq!(back, r);
    assert_eq!(encode_record(& back), text);
}
#[test]
fn unescape_keeps_invalid_sequences_literal() {
    assert_eq!(unescape("%41"), "A");
    assert_eq!(unescape("%7c"), "|");
    assert_eq!(unescape("100%"), "100%");
    assert_eq!(unescape("%4"), "%4");
    assert_eq!(unescape("%zz1"), "%zz1");
}
#[test]
fn decode_is_lenient() {
    let text = "# comment\n\ngarbage line\nunknown=1\nschema_version=1\nschema_version=2\n\
                subengine=cpu\nmodel_id=a%2Fb\n\
                profile=0.9|operands=fp32\nprofile=0.1|operands=int8\nprofile=nopipe\n\
                profile=NaN|a=b\nprofile=inf|a=b\nprofile=1.5|a=b\nprofile=-0.1|a=b\n\
                profile=0.5|novalue\nprofile=abc|a=b\nprofile=0.5|a=b;\n\
                discarded=bad\ndiscarded=batch=1|failed%3D x\npruned=|dominated\npruned=a|x\n";
    let r = decode_record(text).unwrap();
    assert_eq!(r.fingerprint.schema_version, 2);
    assert_eq!(r.fingerprint.model_id, "a/b");
    assert_eq!(r.fingerprint.engine_version, "");
    assert!(! r.unstable);
    let positions: Vec<f64> = r.profiles.iter().map(|p| p.position).collect();
    assert_eq!(positions, vec![0.1, 0.9]);
    assert_eq!(r.discarded.len(), 1);
    assert_eq!(r.discarded[0].combination, comb(& [("batch", "1")]));
    assert_eq!(r.discarded[0].reason, "failed= x");
    assert_eq!(r.pruned.len(), 1);
    assert_eq!(r.pruned[0].combination, comb(& []));
}
#[test]
fn decode_without_known_keys_is_none() {
    assert!(decode_record("").is_none());
    assert!(decode_record("# kalibrierung\nfoo=bar\njust text\n").is_none());
    assert!(decode_record("unstable=true\n").is_some());
}
#[test]
fn equal_positions_keep_file_order() {
    let r = decode_record("profile=0.5|a=1\nprofile=0.2|a=2\nprofile=0.5|a=3\n")
        .unwrap();
    let a: Vec<&str> = r
        .profiles
        .iter()
        .map(|p| p.config.values().next().unwrap().as_str())
        .collect();
    assert_eq!(a, vec!["2", "1", "3"]);
}
#[test]
fn path_for_encodes_model_id() {
    let t = TempDir::new("path");
    let s = store(&t);
    let p = s.path_for("cpu", "intfloat/multilingual-e5-small");
    assert_eq!(
        p.file_name().unwrap().to_str().unwrap(),
        "cpu__intfloat%2Fmultilingual-e5-small.calib.txt"
    );
    assert_eq!(p.parent().unwrap(), t.0.as_path());
    let q = s.path_for("gpu", "a b.é_-");
    assert_eq!(
        q.file_name().unwrap().to_str().unwrap(), "gpu__a%20b.%C3%A9_-.calib.txt"
    );
}
#[test]
fn new_and_load_create_nothing() {
    let t = TempDir::new("nothing");
    let s = store(&t);
    assert_eq!(s.dir(), t.0.as_path());
    assert_eq!(s.load("cpu", "m"), Err(PersistError::NotFound));
    assert!(! t.0.exists());
}
#[test]
fn save_then_load_roundtrips_and_creates_directories() {
    let t = TempDir::new("save");
    let s = CalibrationStore::new(
        t.0.join("nested").join("dir"),
        Arc::new(BackgroundPool::new("calib-test", 1).unwrap()),
    );
    let r = record("1");
    s.save(&r).unwrap();
    assert_eq!(s.load("cpu", E5_SMALL_MODEL_ID).unwrap(), r);
    let mut r2 = r.clone();
    r2.unstable = true;
    s.save(&r2).unwrap();
    assert_eq!(s.load("cpu", E5_SMALL_MODEL_ID).unwrap(), r2);
    assert_eq!(leftover_tmp_files(s.dir()), 0);
}
#[test]
fn load_reports_unreadable_content() {
    let t = TempDir::new("unreadable");
    let s = store(&t);
    fs::create_dir_all(&t.0).unwrap();
    fs::write(s.path_for("cpu", "a"), "nothing useful\n").unwrap();
    assert!(matches!(s.load("cpu", "a"), Err(PersistError::Unreadable(_))));
    fs::write(s.path_for("cpu", "b"), [0xff, 0xfe, b'=', 0x80]).unwrap();
    assert!(matches!(s.load("cpu", "b"), Err(PersistError::Unreadable(_))));
    fs::create_dir(s.path_for("cpu", "c")).unwrap();
    assert!(matches!(s.load("cpu", "c"), Err(PersistError::Unreadable(_))));
}
#[test]
fn failed_save_keeps_target_and_removes_temp_file() {
    let t = TempDir::new("failsave");
    let s = store(&t);
    fs::create_dir_all(&t.0).unwrap();
    let target = s.path_for("cpu", E5_SMALL_MODEL_ID);
    fs::create_dir(&target).unwrap();
    assert!(matches!(s.save(& record("1")), Err(PersistError::Io(_))));
    assert!(target.is_dir());
    assert_eq!(leftover_tmp_files(& t.0), 0);
}
#[test]
fn save_fails_when_directory_is_a_file() {
    let t = TempDir::new("filedir");
    fs::write(&t.0, "x").unwrap();
    let s = store(&t);
    assert!(matches!(s.save(& record("1")), Err(PersistError::Io(_))));
}
#[test]
fn concurrent_saves_never_corrupt() {
    let t = TempDir::new("concurrent");
    let s = Arc::new(store(&t));
    let readers_ok = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let mut handles = Vec::new();
    for i in 0..6 {
        let s = Arc::clone(&s);
        let ok = Arc::clone(&readers_ok);
        handles
            .push(
                std::thread::spawn(move || {
                    for j in 0..15 {
                        let r = record(&format!("{i}-{j}"));
                        s.save(&r).unwrap();
                        match s.load("cpu", E5_SMALL_MODEL_ID) {
                            Ok(l) if l.profiles.len() == 2 => {}
                            _ => ok.store(false, Ordering::Relaxed),
                        }
                    }
                }),
            );
    }
    for h in handles {
        h.join().unwrap();
    }
    assert!(readers_ok.load(Ordering::Relaxed));
    assert_eq!(leftover_tmp_files(& t.0), 0);
}
struct FakeDriver(Option<String>, Option<(u32, u32)>, Option<String>);
impl DriverInfo for FakeDriver {
    fn gpu_name(&self) -> Option<String> {
        self.0.clone()
    }
    fn compute_capability(&self) -> Option<(u32, u32)> {
        self.1
    }
    fn driver_version(&self) -> Option<String> {
        self.2.clone()
    }
}
#[test]
fn cpu_features_text_uses_fixed_order() {
    let f = CpuFeatures {
        avx2: true,
        f16c: false,
        fma: true,
        vnni: true,
    };
    assert_eq!(cpu_features_text(f), "avx2,fma,vnni");
    assert_eq!(cpu_features_text(CpuFeatures::default()), "");
}
#[test]
fn detect_fingerprint_cpu_ignores_driver() {
    let d = FakeDriver(Some("X".into()), Some((7, 5)), Some("1".into()));
    let fp = Fingerprint::detect("cpu", "m", "e1", Some(&d));
    assert_eq!(fp.schema_version, SCHEMA_VERSION3);
    assert_eq!((fp.gpu_name.as_str(), fp.compute_capability.as_str()), ("", ""));
    assert_eq!(fp.driver_version, "");
    assert_eq!(fp.cpu_features, cpu_features_text(detect_cpu_features2()));
    assert_eq!((fp.model_id.as_str(), fp.engine_version.as_str()), ("m", "e1"));
}
#[test]
fn detect_fingerprint_gpu_uses_driver_and_unknown() {
    let d = FakeDriver(Some("GTX 1660 SUPER".into()), Some((7, 5)), None);
    let fp = Fingerprint::detect("gpu", "m", "e1", Some(&d));
    assert_eq!(fp.gpu_name, "GTX 1660 SUPER");
    assert_eq!(fp.compute_capability, "7.5");
    assert_eq!(fp.driver_version, "unknown");
    let none = Fingerprint::detect("gpu", "m", "e1", None);
    assert_eq!(
        (none.gpu_name.as_str(), none.compute_capability.as_str(), none.driver_version
        .as_str()), ("unknown", "unknown", "unknown")
    );
    assert_eq!(none.cpu_features, cpu_features_text(detect_cpu_features2()));
}
#[test]
fn fallback_prefers_precision_and_first_value() {
    let c = CapabilitySet::new()
        .with(axes::operands(), values::int8())
        .with(axes::operands(), values::fp16())
        .with(axes::operands(), values::fp32())
        .with(axes::accumulator(), values::fp16())
        .with(axes::accumulator(), values::fp16_storage_fp32_compute())
        .with(axes::scoring(), values::int8_vnni())
        .with(axes::scoring(), values::int8())
        .with(axes::scoring(), "zzz")
        .with(axes::batch_size(), "16")
        .with(axes::batch_size(), "4")
        .with(axes::batch_size(), "8")
        .with(axes::fusion(), "b")
        .with(axes::fusion(), "a")
        .with(axes::fusion(), "10");
    let m = most_precise_combination(&c).values().clone();
    assert_eq!(m[& axes::operands()], values::fp32());
    assert_eq!(m[& axes::accumulator()], values::fp16_storage_fp32_compute());
    assert_eq!(m[& axes::scoring()], values::int8());
    assert_eq!(m[& axes::batch_size()].as_str(), "4");
    assert_eq!(m[& axes::fusion()].as_str(), "10");
    assert_eq!(m.len(), 5);
    for (axis, value) in &m {
        assert!(c.contains(axis, value));
    }
}
#[test]
fn fallback_numeric_precision_ties_take_first_and_empty_is_empty() {
    let c = CapabilitySet::new().with(axes::scoring(), "2").with(axes::scoring(), "1");
    assert_eq!(most_precise_combination(& c), comb(& [("scoring", "1")]));
    assert_eq!(most_precise_combination(& CapabilitySet::new()), comb(& []));
}
fn measured(stable: bool) -> Outcome {
    Outcome::Measured {
        time: Duration::from_micros(100),
        deviation: 0.0,
        stable,
        samples: 3,
        uncertainty: 0.0,
    }
}
fn entry(stage: Stage2, c: Combination, outcome: Outcome) -> ReportEntry {
    ReportEntry {
        stage,
        combination: c,
        outcome,
    }
}
fn report(entries: Vec<ReportEntry>, front: Vec<FrontPoint>) -> CalibrationReport {
    CalibrationReport {
        target: Target2 {
            subengine: "cpu".into(),
            model_id: "m".into(),
        },
        entries,
        product_size: 0,
        budget: BudgetStatus {
            limit: None,
            elapsed: Duration::ZERO,
            exhausted: false,
        },
        runtime: Duration::ZERO,
        summary: Summary3 {
            front,
            candidate_count: 0,
        },
        counters: Counters::default(),
        log: vec![],
    }
}
fn front() -> Vec<FrontPoint> {
    vec![
        FrontPoint { combination : comb(& [("batch_size", "8")]), time :
        Duration::from_micros(100), deviation : 0.05, }, FrontPoint { combination :
        comb(& [("batch_size", "1")]), time : Duration::from_micros(400), deviation :
        0.0, },
    ]
}
#[test]
fn report_conversion_collects_annotations() {
    let entries = vec![
        entry(Stage2::Axis, comb(& [("a", "1")]), measured(false)), entry(Stage2::Axis,
                                                                          comb(& [("a", "2")]), Outcome::Dominated), entry(Stage2::Verification, comb(&
        [("a", "3")]), Outcome::Discarded(DiscardReason::Failed("x".into())),),
        entry(Stage2::Full, comb(& [("a", "4")]),
              Outcome::Discarded(DiscardReason::NonFinite),), entry(Stage2::Full, comb(& [("a",
                                                                                           "5")]), Outcome::Discarded(DiscardReason::ExceedsMax),), entry(Stage2::Full,
                                                                                                                                                          comb(& [("a", "6")]), Outcome::Discarded(DiscardReason::Other("custom"
        .into())),), entry(Stage2::Full, comb(& [("a", "7")]),
                           Outcome::Skipped(SkipReason::Budget),), entry(Stage2::Full, comb(& [("a", "8")]),
                                                                         Outcome::Dominated),
    ];
    let fp = fingerprint("9");
    let r = record_from_report(fp.clone(), &report(entries, front())).unwrap();
    assert_eq!(r.fingerprint, fp);
    assert!(! r.unstable, "unstable axis-stage entry must not count");
    let reasons: Vec<&str> = r.discarded.iter().map(|a| a.reason.as_str()).collect();
    assert_eq!(
        reasons, vec!["failed: x", "non-finite deviation", "exceeds max deviation",
        "custom"]
    );
    assert_eq!(r.discarded[0].combination, comb(& [("a", "3")]));
    let pruned: Vec<(&Combination, &str)> = r
        .pruned
        .iter()
        .map(|a| (&a.combination, a.reason.as_str()))
        .collect();
    assert_eq!(
        pruned, vec![(& comb(& [("a", "2")]), "dominated"), (& comb(& [("a", "8")]),
        "dominated")]
    );
    assert!(! r.profiles.is_empty());
    assert!(r.profiles.windows(2).all(| w | w[0].position <= w[1].position));
    for p in &r.profiles {
        assert!(front().iter().any(| f | * f.combination.values() == p.config));
    }
}
#[test]
fn report_conversion_flags_unstable_in_verification_and_full() {
    for stage in [Stage2::Verification, Stage2::Full] {
        let e = vec![entry(stage, comb(& [("a", "1")]), measured(false))];
        assert!(
            record_from_report(fingerprint("1"), & report(e, front())).unwrap().unstable
        );
    }
    let e = vec![entry(Stage2::Full, comb(& [("a", "1")]), measured(true))];
    assert!(
        ! record_from_report(fingerprint("1"), & report(e, front())).unwrap().unstable
    );
}
#[test]
fn report_without_front_yields_no_profiles() {
    assert_eq!(
        record_from_report(fingerprint("1"), & report(vec![], vec![])),
        Err(PersistError::NoProfiles)
    );
}
#[test]
fn calibrate_record_runs_calibration() {
    let r = calibrate_record(&request("1", TestBackend::default())).unwrap();
    assert_eq!(r.fingerprint, fingerprint("1"));
    assert!(! r.profiles.is_empty());
}
#[test]
fn calibrate_record_maps_errors() {
    let mut req = request("1", TestBackend::default());
    req.capabilities = CapabilitySet::new();
    assert_eq!(
        calibrate_record(& req),
        Err(PersistError::Calibration(CalibrationError::Capability))
    );
    let req = request(
        "1",
        TestBackend {
            fail: true,
            ..TestBackend::default()
        },
    );
    assert_eq!(
        calibrate_record(& req),
        Err(PersistError::Calibration(CalibrationError::Backend("boom".into())))
    );
}
#[test]
fn resolve_missing_follows_policy_and_writes_nothing() {
    let t = TempDir::new("missing");
    let s = store(&t);
    let req = request("1", TestBackend::default());
    match s.resolve(&req, MissingPolicy::ExplicitCalibrate) {
        Resolution3::Missing(MissingResolution::CalibrateCommand) => {}
        _ => panic!("expected calibrate command"),
    }
    match s.resolve(&req, MissingPolicy::MostPreciseValid) {
        Resolution3::Missing(MissingResolution::Fallback(c)) => {
            assert_eq!(c, comb(& [("batch_size", "1"), ("tile_size", "16")]));
        }
        _ => panic!("expected fallback"),
    }
    assert!(! t.0.exists());
    assert!(lock(& s.in_flight).is_empty());
}
#[test]
fn resolve_unreadable_file_is_missing() {
    let t = TempDir::new("unreadable_resolve");
    let s = store(&t);
    fs::create_dir_all(&t.0).unwrap();
    let req = request("1", TestBackend::default());
    let path = s.path_for("cpu", E5_SMALL_MODEL_ID);
    fs::write(&path, "garbage").unwrap();
    assert!(
        matches!(s.resolve(& req, MissingPolicy::ExplicitCalibrate),
        Resolution3::Missing(MissingResolution::CalibrateCommand))
    );
    assert_eq!(fs::read_to_string(& path).unwrap(), "garbage");
}
#[test]
fn resolve_calibrated_starts_nothing() {
    let t = TempDir::new("calibrated");
    let s = store(&t);
    let r = record("1");
    s.save(&r).unwrap();
    let req = request("1", TestBackend::default());
    match s.resolve(&req, MissingPolicy::ExplicitCalibrate) {
        Resolution3::Calibrated { record } => assert_eq!(record, r),
        _ => panic!("expected calibrated"),
    }
    assert!(lock(& s.in_flight).is_empty());
}
#[test]
fn resolve_stale_recalibrates_in_background() {
    let t = TempDir::new("stale");
    let s = store(&t);
    let old = record("old");
    s.save(&old).unwrap();
    let req = request("new", TestBackend::default());
    let Resolution3::Stale { record, recalibration } = s
        .resolve(&req, MissingPolicy::ExplicitCalibrate) else {
        panic!("expected stale");
    };
    assert_eq!(record, old);
    let new = finish(recalibration.expect("job started")).unwrap();
    assert_eq!(new.fingerprint, fingerprint("new"));
    assert_eq!(s.load("cpu", E5_SMALL_MODEL_ID).unwrap(), new);
    assert!(
        matches!(s.resolve(& req, MissingPolicy::ExplicitCalibrate),
        Resolution3::Calibrated { .. })
    );
}
#[test]
fn defaulted_fingerprint_fields_count_as_stale() {
    let t = TempDir::new("defaulted");
    let s = store(&t);
    fs::create_dir_all(&t.0).unwrap();
    fs::write(
            s.path_for("cpu", E5_SMALL_MODEL_ID),
            format!(
                "subengine=cpu\nmodel_id={E5_SMALL_MODEL_ID}\nprofile=0|batch_size=8\n"
            ),
        )
        .unwrap();
    let req = request("1", TestBackend::default());
    let Resolution3::Stale { recalibration, .. } = s
        .resolve(&req, MissingPolicy::ExplicitCalibrate) else {
        panic!("expected stale");
    };
    assert!(finish(recalibration.unwrap()).is_ok());
}
#[test]
fn stale_with_running_job_returns_no_second_future() {
    let t = TempDir::new("inflight");
    let s = store(&t);
    let old = record("old");
    s.save(&old).unwrap();
    let gate = Gate::new();
    let slow = request(
        "new",
        TestBackend {
            gate: Some(Arc::clone(&gate)),
            ..TestBackend::default()
        },
    );
    let first = s.request_calibration(slow.clone()).unwrap().expect("started");
    assert!(s.request_calibration(slow.clone()).unwrap().is_none());
    let Resolution3::Stale { recalibration, .. } = s
        .resolve(&slow, MissingPolicy::ExplicitCalibrate) else {
        panic!("expected stale");
    };
    assert!(recalibration.is_none());
    assert_eq!(s.load("cpu", E5_SMALL_MODEL_ID).unwrap(), old);
    gate.release();
    assert!(finish(first).is_ok());
    assert!(lock(& s.in_flight).is_empty());
    assert!(s.request_calibration(slow).unwrap().is_some());
}
#[test]
fn other_keys_run_independently() {
    let t = TempDir::new("keys");
    let s = store(&t);
    let gate = Gate::new();
    let slow = request(
        "1",
        TestBackend {
            gate: Some(Arc::clone(&gate)),
            ..TestBackend::default()
        },
    );
    let first = s.request_calibration(slow.clone()).unwrap().unwrap();
    let mut other = slow.clone();
    other.fingerprint.subengine = "gpu".into();
    let second = s.request_calibration(other).unwrap();
    assert!(second.is_some());
    gate.release();
    let _ = finish(first);
    let _ = finish(second.unwrap());
}
#[test]
fn failed_run_keeps_old_file_and_frees_key() {
    let t = TempDir::new("failrun");
    let s = store(&t);
    let old = record("old");
    s.save(&old).unwrap();
    let req = request(
        "new",
        TestBackend {
            fail: true,
            ..TestBackend::default()
        },
    );
    let f = s.request_calibration(req.clone()).unwrap().unwrap();
    assert_eq!(
        finish(f), Err(PersistError::Calibration(CalibrationError::Backend("boom"
        .into())))
    );
    assert_eq!(s.load("cpu", E5_SMALL_MODEL_ID).unwrap(), old);
    assert!(s.request_calibration(req).unwrap().is_some());
}
#[test]
fn panicking_run_surfaces_on_future_and_frees_key() {
    let t = TempDir::new("panicrun");
    let s = store(&t);
    let req = request(
        "new",
        TestBackend {
            panic: true,
            ..TestBackend::default()
        },
    );
    let f = s.request_calibration(req.clone()).unwrap().unwrap();
    assert!(
        matches!(f.wait_timeout(Duration::from_secs(60)), Ok(Err(crate
        ::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::background_pool::JobError::Panicked(_))))
    );
    assert!(lock(& s.in_flight).is_empty());
    assert!(s.request_calibration(req).unwrap().is_some());
}
#[test]
fn shut_down_pool_rejects_requests() {
    let t = TempDir::new("shutdown");
    let pool = Arc::new(BackgroundPool::new("calib-test", 1).unwrap());
    let s = CalibrationStore::new(t.0.clone(), Arc::clone(&pool));
    s.save(&record("old")).unwrap();
    pool.shutdown();
    let req = request("new", TestBackend::default());
    assert!(matches!(s.request_calibration(req.clone()), Err(PersistError::Pool(_))));
    assert!(lock(& s.in_flight).is_empty());
    let Resolution3::Stale { record: stale, recalibration } = s
        .resolve(&req, MissingPolicy::ExplicitCalibrate) else {
        panic!("expected stale");
    };
    assert!(recalibration.is_none());
    assert_eq!(stale, self::record("old"));
}
