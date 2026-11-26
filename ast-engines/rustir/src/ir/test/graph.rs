use super::*;
use crate::ir::{Object, ObjectKind};

fn table(entries: &[(&str, &[&str])]) -> ObjectTable {
    let mut table = ObjectTable::new();
    for (name, dependencies) in entries {
        table.insert(Object {
            name: name.to_string(),
            kind: ObjectKind::Struct,
            body: String::new(),
            dependencies: dependencies.iter().map(|d| d.to_string()).collect(),
        });
    }
    table
}

fn names(set: &BTreeSet<String>) -> Vec<&str> {
    set.iter().map(String::as_str).collect()
}

#[test]
fn incoming_edges_are_grouped_by_target() {
    let incoming = dependents(&table(&[("A", &["C"]), ("B", &["C", "A"]), ("C", &[])]));
    assert_eq!(names(&incoming["C"]), vec!["A", "B"]);
    assert_eq!(names(&incoming["A"]), vec!["B"]);
}

#[test]
fn objects_without_dependents_are_absent() {
    let incoming = dependents(&table(&[("A", &["B"]), ("B", &[])]));
    assert!(!incoming.contains_key("A"));
}

#[test]
fn self_and_unknown_dependencies_are_ignored() {
    let incoming = dependents(&table(&[("A", &["A", "std::fmt::Display"])]));
    assert!(incoming.is_empty());
}
