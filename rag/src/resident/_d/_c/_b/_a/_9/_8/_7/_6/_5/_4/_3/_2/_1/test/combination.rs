use super::*;
use super::super::capability_model::{axes, values};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn hash_of(c: &Combination) -> u64 {
    let mut h = DefaultHasher::new();
    c.hash(&mut h);
    h.finish()
}

#[test]
fn text_is_sorted_and_independent_of_insertion_order() {
    let a = Combination::new()
        .with("scoring", "fp16")
        .unwrap()
        .with("batch_size", 4u64)
        .unwrap();
    let b = Combination::new()
        .with("batch_size", "4")
        .unwrap()
        .with("scoring", "fp16")
        .unwrap();
    assert_eq!(a.to_text(), "batch_size=4, scoring=fp16");
    assert_eq!(a, b);
    assert_eq!(hash_of(&a), hash_of(&b));
    assert_eq!(a.to_string(), a.to_text());
    assert_eq!(Combination::new().to_text(), "");
}

#[test]
fn set_replaces_and_iterates_ascending() {
    let mut c = Combination::new();
    c.set("b", "1").unwrap();
    c.set("a", "2").unwrap();
    c.set("b", "3").unwrap();
    let pairs: Vec<(String, String)> = c
        .iter()
        .map(|(a, v)| (a.as_str().to_string(), v.as_str().to_string()))
        .collect();
    assert_eq!(pairs, vec![("a".into(), "2".into()), ("b".into(), "3".into())]);
    assert_eq!(c.value(&Axis::new("b")), Some(&Value::new("3")));
    assert_eq!(c.value(&Axis::new("z")), None);
}

#[test]
fn invalid_names_are_rejected() {
    for bad in ["", "a,b", "a=b", " a", "a "] {
        let mut c = Combination::new();
        assert_eq!(c.set(bad, "v"), Err(CombinationError::InvalidName(bad.to_string())));
        assert_eq!(c.set("a", bad), Err(CombinationError::InvalidName(bad.to_string())));
        assert_eq!(c.iter().count(), 0);
    }
    assert!(Combination::new().set("a b", "c d").is_ok());
}

#[test]
fn round_trip_through_text() {
    let c = Combination::new()
        .with("x y", "1 2")
        .unwrap()
        .with("scoring", "fp32")
        .unwrap();
    assert_eq!(c.to_text().parse::<Combination>().unwrap(), c);
    assert_eq!("".parse::<Combination>().unwrap(), Combination::new());
    let with_lens = c.clone().with_seq_lens(vec![3, 4]);
    assert_eq!(with_lens.to_text(), c.to_text());
    assert!(with_lens.to_text().parse::<Combination>().unwrap().seq_lens.is_empty());
}

#[test]
fn parse_trims_pairs_and_splits_at_first_equals() {
    let c: Combination = "  a=1 ,\tb=2  ".parse().unwrap();
    assert_eq!(c.to_text(), "a=1, b=2");
    assert_eq!(
        "a=b=c".parse::<Combination>(),
        Err(CombinationError::InvalidName("b=c".to_string()))
    );
}

#[test]
fn parse_errors() {
    assert!(matches!("a".parse::<Combination>(), Err(CombinationError::Parse(_))));
    assert!(matches!("=1".parse::<Combination>(), Err(CombinationError::Parse(_))));
    assert!(matches!("a=".parse::<Combination>(), Err(CombinationError::Parse(_))));
    assert!(matches!("a=1,".parse::<Combination>(), Err(CombinationError::Parse(_))));
    assert_eq!(
        "a=1, a=2".parse::<Combination>(),
        Err(CombinationError::DuplicateAxis(Axis::new("a")))
    );
}

#[test]
fn capability_conversion() {
    let set = CapabilitySet::new()
        .with(axes::scoring(), values::fp16())
        .with(axes::batch_size(), "4");
    let c = Combination::from_capability(&set).unwrap();
    assert_eq!(c.value(&axes::scoring()), Some(&values::fp16()));
    assert!(c.seq_lens.is_empty());
    assert_eq!(c.to_capability(), set);

    let multi = set.clone().with(axes::scoring(), values::fp32());
    assert_eq!(
        Combination::from_capability(&multi),
        Err(CombinationError::NotSingleValued { axis: axes::scoring(), count: 2 })
    );
    assert_eq!(
        Combination::from_capability(&CapabilitySet::new()).unwrap(),
        Combination::new()
    );
}

#[test]
fn from_capability_rejects_invalid_names() {
    let set = CapabilitySet::new().with("a,b", "v");
    assert!(matches!(
        Combination::from_capability(&set),
        Err(CombinationError::InvalidName(_))
    ));
}

#[test]
fn from_values_validates_and_keeps_order() {
    let mut map = BTreeMap::new();
    map.insert(Axis::new("b"), Value::new("1"));
    map.insert(Axis::new("a"), Value::new("2"));
    let c = Combination::from_values(map.clone()).unwrap();
    assert_eq!(c.values(), &map);
    assert!(c.seq_lens.is_empty());
    map.insert(Axis::new("c"), Value::new("x,y"));
    assert!(matches!(Combination::from_values(map), Err(CombinationError::InvalidName(_))));
}

#[test]
fn errors_display() {
    let texts = [
        CombinationError::InvalidName("a,b".into()).to_string(),
        CombinationError::DuplicateAxis(Axis::new("a")).to_string(),
        CombinationError::NotSingleValued { axis: Axis::new("a"), count: 2 }.to_string(),
        CombinationError::Parse("x".into()).to_string(),
    ];
    assert!(texts.iter().all(|t| !t.is_empty()));
}
