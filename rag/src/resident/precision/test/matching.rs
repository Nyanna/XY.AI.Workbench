use super::*;
use crate::resident::precision::ProfileConfig;

fn profiles(positions: &[f32]) -> Vec<Profile> {
    positions
        .iter()
        .map(|&p| Profile::new(p, ProfileConfig::new()).unwrap())
        .collect()
}

fn chosen(t: f32, positions: &[f32]) -> f32 {
    match_profile(t, &profiles(positions)).unwrap().position
}

#[test]
fn out_of_range_values_are_clamped() {
    let set = [0.0, 0.5, 1.0];
    assert_eq!(chosen(-0.3, &set), 0.0);
    assert_eq!(chosen(1.7, &set), 1.0);
    assert_eq!(chosen(f32::INFINITY, &set), 1.0);
    assert_eq!(chosen(f32::NEG_INFINITY, &set), 0.0);
}

#[test]
fn nan_is_an_error_even_without_profiles() {
    assert!(matches!(
        match_profile(f32::NAN, &profiles(&[0.5])),
        Err(PrecisionError::InvalidPrecision { .. })
    ));
    assert!(matches!(
        match_profile(f32::NAN, &[]),
        Err(PrecisionError::InvalidPrecision { .. })
    ));
}

#[test]
fn tie_prefers_the_more_precise_profile() {
    for order in [[0.0, 0.25, 0.75], [0.75, 0.25, 0.0], [0.25, 0.0, 0.75]] {
        for _ in 0..3 {
            assert_eq!(chosen(0.5, &order), 0.75);
        }
    }
    assert_eq!(chosen(0.5, &[0.75, 0.25]), 0.75);
}

#[test]
fn nearest_profile_wins() {
    let set = [0.0, 0.3, 1.0];
    assert_eq!(chosen(0.1, &set), 0.0);
    assert_eq!(chosen(0.2, &set), 0.3);
    assert_eq!(chosen(0.9, &set), 1.0);
}

#[test]
fn lone_profile_matches_every_wish() {
    for t in [0.0, 0.3, 1.0, 5.0, -1.0] {
        assert_eq!(chosen(t, &[0.5]), 0.5);
    }
}

#[test]
fn empty_set_has_no_active_profile() {
    assert!(matches!(
        match_profile(0.5, &[]),
        Err(PrecisionError::NoActiveProfile)
    ));
}

#[test]
fn distances_are_not_collapsed_by_f32_rounding() {
    let set = profiles(&[0.1, 0.5]);
    assert_eq!(match_profile(0.3, &set).unwrap().position, 0.5);
    assert_eq!(match_profile(0.299_999_9, &set).unwrap().position, 0.1);
}

#[test]
fn empty_model_is_blocked_after_first_failure() {
    let selector = ProfileSelector::new();
    assert!(!selector.is_blocked("m"));
    assert!(matches!(
        selector.select("m", 0.5, &[]),
        Err(PrecisionError::NoActiveProfile)
    ));
    assert!(selector.is_blocked("m"));

    let set = profiles(&[0.5]);
    assert!(matches!(
        selector.select("m", 0.5, &set),
        Err(PrecisionError::NoActiveProfile)
    ));
    assert!(selector.select("other", 0.5, &set).is_ok());

    assert!(selector.unblock("m"));
    assert!(selector.select("m", 0.5, &set).is_ok());
}

#[test]
fn nan_does_not_block_the_model() {
    let selector = ProfileSelector::new();
    let set = profiles(&[0.5]);
    assert!(selector.select("m", f32::NAN, &set).is_err());
    assert!(!selector.is_blocked("m"));
}
