use super::*;
use crate::ir::ObjectKind;
fn object(name: &str, body: &str, dependencies: &[&str]) -> Object {
    Object {
        name: name.into(),
        kind: ObjectKind::Struct,
        body: body.into(),
        dependencies: dependencies.iter().map(|d| d.to_string()).collect(),
    }
}
#[test]
fn insert_keeps_the_first_object() {
    let mut table = ObjectTable::new();
    assert!(table.insert(object("A", "first", & [])));
    assert!(! table.insert(object("A", "second", & [])));
    assert_eq!(table.len(), 1);
    assert_eq!(table.get("A").unwrap().body, "first");
}
#[test]
fn distinct_names_are_all_kept() {
    let mut table = ObjectTable::new();
    table.insert(object("A", "", &[]));
    table.insert(object("B", "", &[]));
    assert_eq!(table.iter().count(), 2);
    assert!(table.contains("B"));
    assert!(! table.is_empty());
}
#[test]
fn resolve_drops_unknown_and_self_and_duplicates() {
    let mut table = ObjectTable::new();
    table.insert(object("A", "", &["B", "A", "Missing", "B"]));
    table.insert(object("B", "", &[]));
    table.resolve_dependencies();
    assert_eq!(table.get("A").unwrap().dependencies, vec!["B".to_string()]);
}
#[test]
fn attach_appends_body_and_dependencies_to_the_owner() {
    let mut table = ObjectTable::new();
    table.insert(object("A", "struct A;", &[]));
    assert!(table.attach("A", "impl A {}", ["B".to_string()]));
    let owner = table.get("A").unwrap();
    assert_eq!(owner.body, "struct A;\n\nimpl A {}");
    assert_eq!(owner.dependencies, vec!["B".to_string()]);
}
#[test]
fn attach_to_unknown_owner_is_rejected() {
    let mut table = ObjectTable::new();
    assert!(! table.attach("Unknown", "impl X {}", []));
    assert!(table.is_empty());
}
#[test]
fn set_dependencies_replaces_sorts_and_drops_self() {
    let mut table = ObjectTable::new();
    table.insert(object("A", "", &["Old"]));
    assert!(
        table.set_dependencies("A", vec!["z::Z".into(), "B".into(), "A".into(), "B"
        .into()])
    );
    assert_eq!(
        table.get("A").unwrap().dependencies, vec!["B".to_string(), "z::Z".to_string()]
    );
    assert!(! table.set_dependencies("Unknown", vec![]));
}
