use super::*;

#[derive(Debug, PartialEq)]
struct Correlation(u64);

#[derive(Debug, PartialEq)]
struct Label(&'static str);

#[test]
fn stores_one_value_per_type() {
    let mut ext = Extensions::new();
    ext.insert(Correlation(1)).insert(Label("a"));
    assert_eq!(ext.get::<Correlation>(), Some(&Correlation(1)));
    assert_eq!(ext.get::<Label>(), Some(&Label("a")));
    ext.insert(Correlation(2));
    assert_eq!(ext.get::<Correlation>(), Some(&Correlation(2)));
    assert_eq!(ext.len(), 2);
}

#[test]
fn missing_type_is_none() {
    let ext = Extensions::new();
    assert!(ext.is_empty());
    assert!(ext.get::<Label>().is_none());
}

#[test]
fn clone_is_independent_for_later_changes() {
    let mut original = Extensions::new();
    original.insert(Correlation(7));
    let mut copy = original.clone();
    copy.insert(Correlation(8));
    assert!(copy.remove::<Correlation>());
    assert_eq!(original.get::<Correlation>(), Some(&Correlation(7)));
    assert!(copy.get::<Correlation>().is_none());
}

#[test]
fn extensions_are_send_and_sync() {
    fn assert_traits<T: Send + Sync>() {}
    assert_traits::<Extensions>();
}
