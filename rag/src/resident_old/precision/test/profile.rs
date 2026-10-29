use super::*;
#[test]
fn position_must_be_in_unit_interval() {
    for p in [0.0, 0.5, 1.0] {
        assert!(Profile::new(p, ProfileConfig::new()).is_ok());
    }
    for p in [-0.1, 1.1, f32::NAN, f32::INFINITY] {
        assert!(
            matches!(Profile::new(p, ProfileConfig::new()),
            Err(PrecisionError::InvalidPrecision { .. }))
        );
    }
}
#[test]
fn config_holds_one_value_per_axis() {
    let cfg = ProfileConfig::new()
        .with("operands", "fp16")
        .with("operands", "fp32")
        .with("fusion", "on");
    assert_eq!(cfg.len(), 2);
    assert_eq!(cfg.get(& AxisId::new("operands")), Some(& ValueId::new("fp32")));
}
use std::time::Duration;
use crate::resident::precision::measure::Cfg;
fn point(name: &str, ms: f64, deviation: f64, uncertainty: f64) -> ParetoPoint {
    ParetoPoint {
        cfg: Cfg::new(ProfileConfig::new().with("path", name), vec![8]),
        time: Duration::from_secs_f64(ms / 1e3),
        deviation,
        stable: true,
        uncertainty,
    }
}
fn names(profiles: &[Profile]) -> Vec<String> {
    profiles
        .iter()
        .map(|p| p.config.get(&AxisId::new("path")).unwrap().to_string())
        .collect()
}
fn threshold(t: f64) -> ProfilingConfig {
    ProfilingConfig {
        spread_threshold: t,
    }
}
#[test]
fn empty_front_yields_no_profile() {
    assert!(derive_profiles(& [], & ProfilingConfig::default()).is_empty());
}
#[test]
fn single_point_sits_in_the_middle() {
    let p = derive_profiles(
        &[point("f32", 2.0, 0.0, 0.01)],
        &ProfilingConfig::default(),
    );
    assert_eq!(p.len(), 1);
    assert_eq!(p[0].position, Profile::SINGLE_POSITION);
}
#[test]
fn threshold_controls_profile_count() {
    let front = [
        point("a", 1.0, 0.4, 0.0),
        point("b", 2.0, 0.3, 0.0),
        point("c", 4.0, 0.2, 0.0),
        point("d", 8.0, 0.0, 0.0),
    ];
    assert_eq!(derive_profiles(& front, & threshold(0.01)).len(), 4);
    assert_eq!(derive_profiles(& front, & threshold(0.35)).len(), 2);
    assert_eq!(derive_profiles(& front, & threshold(10.0)).len(), 1);
}
#[test]
fn close_points_merge_and_precise_end_is_kept() {
    let front = [
        point("int8", 1.18, 5.5e-4, 0.01),
        point("f16", 1.92, 4.8e-5, 0.01),
        point("f32", 2.00, 0.0, 0.01),
    ];
    let p = derive_profiles(&front, &ProfilingConfig::default());
    assert_eq!(names(& p), vec!["int8", "f32"]);
    assert_eq!(p[0].position, 0.0);
    assert_eq!(p[1].position, 1.0);
    assert_eq!(names(& derive_profiles(& front, & threshold(0.05))).len(), 3);
}
#[test]
fn cluster_representative_is_the_fastest_point() {
    let front = [
        point("a", 1.0, 0.9, 0.0),
        point("b", 1.1, 0.8, 0.0),
        point("c", 5.0, 0.1, 0.0),
        point("d", 10.0, 0.0, 0.0),
    ];
    let p = derive_profiles(&front, &threshold(0.15));
    assert_eq!(names(& p), vec!["a", "c", "d"]);
}
#[test]
fn dense_chain_collapses_to_one_profile() {
    let front: Vec<_> = (0..11)
        .map(|i| point(
            &format!("p{i}"),
            1.0 + i as f64 * 0.1,
            1.0 - i as f64 * 0.1,
            0.0,
        ))
        .collect();
    let p = derive_profiles(&front, &threshold(0.2));
    assert_eq!(p.len(), 1);
    assert_eq!(names(& p), vec!["p10"]);
}
#[test]
fn positions_follow_distance_not_rank() {
    let front = [
        point("a", 1.0, 1.0, 0.0),
        point("b", 2.0, 0.9, 0.0),
        point("c", 1000.0, 0.0, 0.0),
    ];
    let p = derive_profiles(&front, &threshold(0.001));
    assert_eq!(p.len(), 3);
    assert!(p[1].position < 0.5);
}
#[test]
fn point_within_noise_of_a_more_precise_one_is_dropped() {
    let front = [point("f16", 6.78, 4.8e-5, 0.02), point("f32", 6.79, 0.0, 0.02)];
    let p = derive_profiles(&front, &threshold(0.001));
    assert_eq!(names(& p), vec!["f32"]);
    assert_eq!(p[0].position, Profile::SINGLE_POSITION);
}
#[test]
fn significantly_faster_point_survives_noise_check() {
    let front = [point("fast", 1.0, 1e-3, 0.02), point("ref", 2.0, 0.0, 0.02)];
    let p = derive_profiles(&front, &threshold(0.001));
    assert_eq!(names(& p), vec!["fast", "ref"]);
}
#[test]
fn identical_coordinates_do_not_break_positions() {
    let front = [point("a", 1.0, 0.5, 0.0), point("b", 2.0, 0.0, 0.0)];
    let same_time = [point("a", 1.0, 0.5, 0.0), point("b", 1.0, 0.0, 0.0)];
    for f in [&front[..], &same_time[..]] {
        let p = derive_profiles(f, &threshold(0.001));
        assert!(p.iter().all(| x | (0.0..= 1.0).contains(& x.position)));
    }
}
#[test]
fn non_finite_points_are_ignored() {
    let front = [point("nan", 1.0, f64::NAN, 0.0), point("ok", 2.0, 0.0, 0.0)];
    assert_eq!(
        names(& derive_profiles(& front, & ProfilingConfig::default())), vec!["ok"]
    );
}
