use super::*;
use crate::resident::l130::l120::l110::l100::a::measurement::bench_workload::ValidityKind;
fn valid() -> Validity {
    Validity {
        kind: ValidityKind::Valid,
        text: "ok".to_string(),
    }
}
fn measured(d: f64) -> DeviationOutcome {
    DeviationOutcome::Measured {
        deviation: d,
        validity: valid(),
    }
}
fn incompatible() -> DeviationOutcome {
    DeviationOutcome::Incompatible {
        validity: Validity {
            kind: ValidityKind::ExceedsMax,
            text: "too far".to_string(),
        },
    }
}
#[test]
fn median_odd_and_even() {
    let odd = MeasurementResult::from_samples(&[3.0, 1.0, 2.0], &measured(0.5)).unwrap();
    assert_eq!(odd.robust_time_ns, 2.0);
    let even = MeasurementResult::from_samples(&[4.0, 1.0, 2.0, 3.0], &measured(0.5))
        .unwrap();
    assert_eq!(even.robust_time_ns, 2.5);
    assert_eq!(even.samples, 4);
}
#[test]
fn single_sample_is_unstable_with_infinite_uncertainty() {
    let r = MeasurementResult::from_samples(&[5.0], &measured(0.1)).unwrap();
    assert_eq!(r.stability, Stability::Unstable);
    assert_eq!(r.rel_uncertainty, f64::INFINITY);
    assert_eq!(r.robust_time_ns, 5.0);
    assert_eq!(r.deviation, Some(0.1));
}
#[test]
fn spread_follows_the_mad_rule() {
    let r = MeasurementResult::from_samples(&[10.0, 10.0, 10.0, 10.0], &measured(0.0))
        .unwrap();
    assert_eq!(r.stability, Stability::Stable);
    assert_eq!(r.rel_uncertainty, 0.0);
    let r = MeasurementResult::from_samples(
            &[10.0, 11.0, 10.0, 9.0, 10.0],
            &measured(0.0),
        )
        .unwrap();
    assert_eq!(r.stability, Stability::Stable);
    let r = MeasurementResult::from_samples(
            &[8.0, 9.0, 10.0, 11.0, 12.0],
            &measured(0.0),
        )
        .unwrap();
    assert_eq!(r.stability, Stability::Unstable);
    let spread = 1.4826 / 10.0;
    assert!((r.rel_uncertainty - spread / 5.0f64.sqrt()).abs() < 1e-12);
}
#[test]
fn incompatible_has_no_deviation() {
    let r = MeasurementResult::from_samples(&[1.0, 1.0], &incompatible()).unwrap();
    assert_eq!(r.deviation, None);
    assert_eq!(r.validity.kind, ValidityKind::ExceedsMax);
}
#[test]
fn invalid_samples_are_rejected() {
    assert_eq!(
        MeasurementResult::from_samples(& [], & measured(0.0)),
        Err(MeasureError::NoSamples)
    );
    for bad in [f64::NAN, f64::INFINITY, -1.0] {
        assert_eq!(
            MeasurementResult::from_samples(& [1.0, bad], & measured(0.0)),
            Err(MeasureError::NoSamples)
        );
    }
}
#[test]
fn zero_median_does_not_divide_by_zero() {
    let r = MeasurementResult::from_samples(&[0.0, 0.0, 0.0], &measured(0.0)).unwrap();
    assert_eq!(r.stability, Stability::Stable);
    let r = MeasurementResult::from_samples(&[0.0, 0.0, 5.0], &measured(0.0)).unwrap();
    assert_eq!(r.robust_time_ns, 0.0);
    assert_eq!(r.rel_uncertainty, 0.0);
}
