use super::*;

use std::sync::Arc;
use std::thread;

use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::a::resource_requirements::{
    ExecutionEnvironment, ResourceRequirements,
};
use crate::resident::_d::_c::_b::_a::_9::_8::_7::_6::_5::_4::_3::_2::_1::device_lane_event::DeviceHandle;

fn notice(kind: ResourceKind, key: &str) -> IdleNotice {
    IdleNotice {
        key: key.to_string(),
        kind,
        requirements: ResourceRequirements::new(kind, ExecutionEnvironment::Any),
    }
}

fn idle(notices: Vec<IdleNotice>) -> IterationOutcome {
    IterationOutcome {
        idle: notices,
        ..Default::default()
    }
}

fn hit() -> IterationOutcome {
    IterationOutcome {
        claimed: 1,
        ..Default::default()
    }
}

fn cand(item: u32, first_kind: Option<ResourceKind>) -> Candidate<u32> {
    Candidate { item, first_kind }
}

fn load(kind: ResourceKind, capacity: usize) -> LoadReport {
    LoadReport {
        kind,
        device: DeviceHandle::Cpu,
        capacity,
        throughput: 0.0,
    }
}

fn sorted_keys(mut notices: Vec<IdleNotice>) -> Vec<String> {
    notices.sort_by(|a, b| a.key.cmp(&b.key));
    notices.into_iter().map(|n| n.key).collect()
}

#[test]
fn new_control_is_empty() {
    let control = ExpansionControl::new();
    assert!(!control.has_idle());
    assert!(control.snapshot().is_empty());
    assert!(control.underserved().is_empty());
    assert!(control.notifications(ResourceKind::Tokenizer).is_empty());
}

#[test]
fn idle_iteration_inserts_under_reported_kind() {
    let control = ExpansionControl::new();
    let foreign = notice(ResourceKind::Scorer, "a");
    control.report_iteration(ResourceKind::Tokenizer, &idle(vec![foreign]));
    assert_eq!(control.notifications(ResourceKind::Tokenizer).len(), 1);
    assert!(control.notifications(ResourceKind::Scorer).is_empty());
    assert_eq!(control.underserved(), vec![ResourceKind::Tokenizer]);
    assert!(control.has_idle());
}

#[test]
fn same_key_replaces_and_other_keys_stay() {
    let control = ExpansionControl::new();
    let kind = ResourceKind::Encoder;
    control.report_iteration(kind, &idle(vec![notice(kind, "a"), notice(kind, "b")]));
    let mut replacement = notice(kind, "a");
    replacement.requirements = ResourceRequirements::new(kind, ExecutionEnvironment::GpuOnly);
    control.report_iteration(kind, &idle(vec![replacement.clone()]));
    let notices = control.notifications(kind);
    assert_eq!(sorted_keys(notices.clone()), vec!["a", "b"]);
    assert!(notices.contains(&replacement));
}

#[test]
fn duplicate_keys_in_one_outcome_collapse() {
    let control = ExpansionControl::new();
    let kind = ResourceKind::Index;
    control.report_iteration(kind, &idle(vec![notice(kind, "a"), notice(kind, "a")]));
    assert_eq!(control.notifications(kind).len(), 1);
}

#[test]
fn hit_clears_whole_kind_only() {
    let control = ExpansionControl::new();
    let (t, s) = (ResourceKind::Tokenizer, ResourceKind::Scorer);
    control.report_iteration(t, &idle(vec![notice(t, "a"), notice(t, "b")]));
    control.report_iteration(s, &idle(vec![notice(s, "c")]));
    let mut outcome = hit();
    outcome.idle = vec![notice(t, "ignored")];
    control.report_iteration(t, &outcome);
    assert!(control.notifications(t).is_empty());
    assert_eq!(control.underserved(), vec![s]);
}

#[test]
fn empty_idle_without_hit_changes_nothing() {
    let control = ExpansionControl::new();
    let kind = ResourceKind::Tokenizer;
    control.report_iteration(kind, &idle(vec![notice(kind, "a")]));
    control.report_iteration(kind, &IterationOutcome::default());
    assert_eq!(sorted_keys(control.snapshot()), vec!["a"]);
    let fresh = ExpansionControl::new();
    fresh.report_iteration(kind, &IterationOutcome::default());
    assert!(!fresh.has_idle());
}

#[test]
fn inits_and_evictions_are_ignored() {
    let control = ExpansionControl::new();
    let kind = ResourceKind::Tokenizer;
    control.report_iteration(kind, &idle(vec![notice(kind, "a")]));
    let outcome = IterationOutcome {
        inits_started: 3,
        evicted: vec!["a".to_string()],
        ..Default::default()
    };
    control.report_iteration(kind, &outcome);
    assert_eq!(control.notifications(kind).len(), 1);
}

#[test]
fn mark_busy_removes_only_matching_key() {
    let control = ExpansionControl::new();
    let kind = ResourceKind::Scorer;
    control.report_iteration(kind, &idle(vec![notice(kind, "a"), notice(kind, "b")]));
    control.mark_busy(kind, "unknown");
    control.mark_busy(ResourceKind::Tokenizer, "a");
    assert_eq!(control.notifications(kind).len(), 2);
    control.mark_busy(kind, "a");
    assert_eq!(sorted_keys(control.notifications(kind)), vec!["b"]);
    control.mark_busy(kind, "b");
    assert!(!control.has_idle());
    assert!(control.underserved().is_empty());
}

#[test]
fn zero_capacity_load_clears_kind_and_positive_capacity_is_inert() {
    let control = ExpansionControl::new();
    let (t, s) = (ResourceKind::Tokenizer, ResourceKind::Scorer);
    control.report_iteration(t, &idle(vec![notice(t, "a")]));
    control.report_iteration(s, &idle(vec![notice(s, "b")]));
    control.observe_load(&load(t, 4));
    control.observe_load(&load(ResourceKind::Encoder, 4));
    assert_eq!(control.snapshot().len(), 2);
    assert!(control.notifications(ResourceKind::Encoder).is_empty());
    control.observe_load(&load(t, 0));
    assert_eq!(control.underserved(), vec![s]);
    control.observe_load(&load(ResourceKind::Encoder, 0));
    assert_eq!(control.underserved(), vec![s]);
}

#[test]
fn underserved_is_sorted_and_without_duplicates() {
    let control = ExpansionControl::new();
    for kind in [ResourceKind::Scorer, ResourceKind::Tokenizer, ResourceKind::Encoder] {
        control.report_iteration(kind, &idle(vec![notice(kind, "a"), notice(kind, "b")]));
    }
    let kinds = control.underserved();
    let mut expected = kinds.clone();
    expected.sort();
    expected.dedup();
    assert_eq!(kinds, expected);
    assert_eq!(kinds.len(), 3);
}

#[test]
fn select_on_empty_is_none() {
    let control = ExpansionControl::new();
    assert_eq!(control.select::<u32>(&[]), None);
}

#[test]
fn select_without_notifications_takes_oldest() {
    let control = ExpansionControl::new();
    let candidates = [cand(0, Some(ResourceKind::Scorer)), cand(1, None)];
    assert_eq!(control.select(&candidates), Some(0));
}

#[test]
fn select_prefers_first_underserved_candidate() {
    let control = ExpansionControl::new();
    let (t, s) = (ResourceKind::Tokenizer, ResourceKind::Scorer);
    control.report_iteration(s, &idle(vec![notice(s, "a")]));
    control.report_iteration(t, &idle(vec![notice(t, "b")]));
    let candidates = [
        cand(0, Some(ResourceKind::Encoder)),
        cand(1, None),
        cand(2, Some(s)),
        cand(3, Some(t)),
    ];
    assert_eq!(control.select(&candidates), Some(2));
    assert_eq!(control.select(&candidates), Some(2));
}

#[test]
fn select_falls_back_to_oldest_when_nothing_matches() {
    let control = ExpansionControl::new();
    let s = ResourceKind::Scorer;
    control.report_iteration(s, &idle(vec![notice(s, "a")]));
    let candidates = [cand(0, None), cand(1, Some(ResourceKind::Tokenizer))];
    assert_eq!(control.select(&candidates), Some(0));
}

#[test]
fn select_does_not_consume_notifications() {
    let control = ExpansionControl::new();
    let s = ResourceKind::Scorer;
    control.report_iteration(s, &idle(vec![notice(s, "a")]));
    let candidates = [cand(0, None), cand(1, Some(s))];
    for _ in 0..3 {
        assert_eq!(control.select(&candidates), Some(1));
    }
    assert_eq!(control.notifications(s).len(), 1);
}

#[test]
fn poisoned_lock_is_recovered() {
    let control = Arc::new(ExpansionControl::new());
    let kind = ResourceKind::Tokenizer;
    control.report_iteration(kind, &idle(vec![notice(kind, "a")]));
    let poisoner = Arc::clone(&control);
    let _ = thread::spawn(move || {
        let _guard = poisoner.state.lock().unwrap();
        panic!("poison");
    })
    .join();
    assert!(control.state.is_poisoned());
    assert_eq!(control.notifications(kind).len(), 1);
    control.mark_busy(kind, "a");
    assert!(!control.has_idle());
    assert_eq!(control.select(&[cand(0, None)]), Some(0));
}

#[test]
fn concurrent_use_keeps_state_consistent() {
    let control = Arc::new(ExpansionControl::new());
    let kind = ResourceKind::Tokenizer;
    let handles: Vec<_> = (0..4)
        .map(|i| {
            let control = Arc::clone(&control);
            thread::spawn(move || {
                for n in 0..200 {
                    let key = format!("k{}", (i + n) % 5);
                    control.report_iteration(kind, &idle(vec![notice(kind, &key)]));
                    let _ = control.select(&[cand(0, Some(kind))]);
                    if n % 7 == 0 {
                        control.mark_busy(kind, &key);
                    }
                }
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let notices = control.notifications(kind);
    let mut keys = sorted_keys(notices);
    let len = keys.len();
    keys.dedup();
    assert_eq!(keys.len(), len);
    assert!(len <= 5);
}
