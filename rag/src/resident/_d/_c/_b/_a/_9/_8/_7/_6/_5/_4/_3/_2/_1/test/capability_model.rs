use super::*;

fn n(s: &str) -> Name {
    Name::new(s)
}

#[test]
fn name_basics() {
    assert_eq!(Name::from_number(0).as_str(), "0");
    assert_eq!(Name::from(128u64), Name::from("128".to_string()));
    assert!(n("10") < n("9"));
    assert_eq!(n("").as_str(), "");
    assert_eq!(n("x").to_string(), "x");
}

#[test]
fn add_and_query() {
    let mut s = CapabilitySet::new();
    assert!(s.is_empty());
    assert!(s.add("a", "1"));
    assert!(!s.add("a", "1"));
    assert!(s.contains(&n("a"), &n("1")));
    assert!(!s.contains(&n("b"), &n("1")));
    assert!(s.values(&n("b")).is_none());
    assert!(!s.is_empty());
}

#[test]
fn order_independence() {
    let a = CapabilitySet::new().with("x", "2").with("x", "1").with("a", "z");
    let b = CapabilitySet::new().with("a", "z").with("x", "1").with("x", "2");
    assert_eq!(a, b);
    assert_eq!(serde_json::to_string(&a).unwrap(), serde_json::to_string(&b).unwrap());
    let pairs: Vec<_> = a.iter().map(|(x, y)| format!("{x}{y}")).collect();
    assert_eq!(pairs, ["az", "x1", "x2"]);
}

#[test]
fn satisfies_rules() {
    let s = CapabilitySet::new().with("a", "1").with("a", "2");
    assert!(s.satisfies(&CapabilitySet::new()));
    assert!(CapabilitySet::new().satisfies(&CapabilitySet::new()));
    assert!(s.satisfies(&CapabilitySet::new().with("a", "1")));
    assert!(!s.satisfies(&CapabilitySet::new().with("a", "3")));
    assert!(!s.satisfies(&CapabilitySet::new().with("b", "1")));
}

#[test]
fn reference_sets() {
    let gpu = gpu_reference();
    let cpu = cpu_reference();
    assert_eq!(gpu, gpu_reference());
    assert!(cpu.is_within(&cpu));
    assert!(!cpu.is_within(&gpu));
    assert!(CapabilitySet::new().with(axes::scoring(), values::int8_dp4a()).is_within(&gpu));
    assert!(cpu.values(&axes::batch_size()).is_none());
    assert!(gpu.values(&axes::density()).is_none());
    assert_eq!(gpu.values(&axes::scoring()).unwrap().len(), 3);
    assert_eq!(cpu.values(&axes::scoring()).unwrap().len(), 3);
    assert!(cpu.values(&axes::accumulator()).is_none());
}

#[test]
fn serde_round_trip_and_errors() {
    let s = gpu_reference();
    let json = serde_json::to_string(&s).unwrap();
    assert_eq!(serde_json::from_str::<CapabilitySet>(&json).unwrap(), s);

    let merged: CapabilitySet = serde_json::from_str(r#"{"a":["1","1"]}"#).unwrap();
    assert_eq!(merged.values(&n("a")).unwrap().len(), 1);

    let err = serde_json::from_str::<CapabilitySet>(r#"{"a":[]}"#).unwrap_err();
    assert!(err.to_string().contains("empty value set"));
}

#[test]
fn send_sync() {
    fn check<T: Send + Sync>() {}
    check::<Name>();
    check::<CapabilitySet>();
}
