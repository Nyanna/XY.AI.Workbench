use super::*;
use super::super::bench_workload::{Validity, ValidityKind};
use std::sync::Mutex;
struct Script {
    deviation: Mutex<Option<Result<DeviationOutcome, MeasureError>>>,
    time: Mutex<Option<Result<TimeProbe, MeasureError>>>,
    seen: Mutex<Vec<Vec<usize>>>,
}
impl Script {
    fn new(
        deviation: Result<DeviationOutcome, MeasureError>,
        time: Result<TimeProbe, MeasureError>,
    ) -> Arc<Script> {
        Arc::new(Script {
            deviation: Mutex::new(Some(deviation)),
            time: Mutex::new(Some(time)),
            seen: Mutex::new(Vec::new()),
        })
    }
}
impl Measure for Script {
    fn deviation(&self, combo: &Combination) -> Result<DeviationOutcome, MeasureError> {
        self.seen.lock().unwrap().push(combo.seq_lens.clone());
        self.deviation.lock().unwrap().take().expect("scripted once")
    }
    fn time_probe(&self, combo: &Combination) -> Result<TimeProbe, MeasureError> {
        self.seen.lock().unwrap().push(combo.seq_lens.clone());
        self.time.lock().unwrap().take().expect("scripted once")
    }
}
fn validity(kind: ValidityKind) -> Validity {
    Validity {
        kind,
        text: String::new(),
    }
}
fn backend(script: &Arc<Script>) -> MeasureBackend {
    let measure: Arc<dyn Measure> = script.clone();
    MeasureBackend::new(measure)
}
fn no_time() -> Result<TimeProbe, MeasureError> {
    Ok(TimeProbe::Interrupted)
}
fn no_deviation() -> Result<DeviationOutcome, MeasureError> {
    Ok(DeviationOutcome::Measured {
        deviation: 0.0,
        validity: validity(ValidityKind::Valid),
    })
}
fn deviation_error(error: MeasureError) -> ProbeError {
    let script = Script::new(Err(error), no_time());
    backend(&script).deviation(&Combination::new(), &[]).unwrap_err()
}
#[test]
fn measured_deviation_is_passed_through_with_the_sequence_lengths() {
    let script = Script::new(
        Ok(DeviationOutcome::Measured {
            deviation: 0.25,
            validity: validity(ValidityKind::Valid),
        }),
        no_time(),
    );
    let value = backend(&script).deviation(&Combination::new(), &[16, 64]).unwrap();
    assert_eq!(value, 0.25);
    assert_eq!(* script.seen.lock().unwrap(), vec![vec![16, 64]]);
}
#[test]
fn incompatible_deviation_is_rejected() {
    let validity = validity(ValidityKind::NonFinite);
    let script = Script::new(
        Ok(DeviationOutcome::Incompatible {
            validity,
        }),
        no_time(),
    );
    let error = backend(&script).deviation(&Combination::new(), &[]).unwrap_err();
    assert!(matches!(error, ProbeError::Rejected(_)));
}
#[test]
fn time_is_converted_from_nanoseconds() {
    let script = Script::new(no_deviation(), Ok(TimeProbe::Time(2_500.0)));
    let t = backend(&script)
        .time(
            &Combination::new(),
            &[8],
            Duration::from_millis(10),
            Duration::from_millis(5),
        )
        .unwrap();
    assert_eq!(t, Duration::from_nanos(2_500));
    assert_eq!(* script.seen.lock().unwrap(), vec![vec![8]]);
}
#[test]
fn interrupted_probe_maps_to_interrupted() {
    let script = Script::new(no_deviation(), Ok(TimeProbe::Interrupted));
    let error = backend(&script)
        .time(&Combination::new(), &[], Duration::ZERO, Duration::ZERO)
        .unwrap_err();
    assert_eq!(error, ProbeError::Interrupted);
}
#[test]
fn non_finite_and_negative_times_fail() {
    for bad in [f64::NAN, f64::INFINITY, -1.0] {
        let script = Script::new(no_deviation(), Ok(TimeProbe::Time(bad)));
        let error = backend(&script)
            .time(&Combination::new(), &[], Duration::ZERO, Duration::ZERO)
            .unwrap_err();
        assert!(matches!(error, ProbeError::Failed(_)));
    }
}
#[test]
fn measure_errors_map_to_probe_errors() {
    assert_eq!(
        deviation_error(MeasureError::Capability("no gpu".into())),
        ProbeError::Unavailable
    );
    assert_eq!(
        deviation_error(MeasureError::Config("bad".into())), ProbeError::Fatal("bad"
        .into())
    );
    assert!(matches!(deviation_error(MeasureError::NoSamples), ProbeError::Failed(_)));
    assert!(
        matches!(deviation_error(MeasureError::Compute("x".into())),
        ProbeError::Failed(_))
    );
}
#[test]
fn time_errors_map_like_deviation_errors() {
    let script = Script::new(
        no_deviation(),
        Err(MeasureError::Capability("no gpu".into())),
    );
    let error = backend(&script)
        .time(&Combination::new(), &[], Duration::ZERO, Duration::ZERO)
        .unwrap_err();
    assert_eq!(error, ProbeError::Unavailable);
}
#[test]
fn measure_config_follows_the_calibration_config() {
    let config = CalibrationConfig {
        time_window: Duration::from_millis(20),
        chunk_budget: Duration::from_millis(3),
        max_deviation: 0.125,
        ..CalibrationConfig::default()
    };
    let measure = MeasureConfig::for_calibration(&config);
    assert_eq!(measure.window_ns, 20_000_000);
    assert_eq!(measure.chunk_budget_ns, 3_000_000);
    assert_eq!(measure.max_deviation, 0.125);
    assert_eq!(measure.top_k, DEFAULT_TOP_K);
}
