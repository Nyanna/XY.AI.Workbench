use super::*;

#[test]
fn single_definitions_are_not_duplicates() {
    let mut conflicts = Conflicts::new();
    conflicts.record("A", Path::new("a.rs"));
    conflicts.record("B", Path::new("b.rs"));
    assert!(conflicts.duplicates().is_empty());
}

#[test]
fn duplicates_list_every_location_in_order() {
    let mut conflicts = Conflicts::new();
    conflicts.record("A", Path::new("one.rs"));
    conflicts.record("B", Path::new("b.rs"));
    conflicts.record("A", Path::new("two.rs"));
    conflicts.record("A", Path::new("three.rs"));
    assert_eq!(
        conflicts.duplicates(),
        vec![NameConflict {
            name: "A".into(),
            files: vec![
                PathBuf::from("one.rs"),
                PathBuf::from("two.rs"),
                PathBuf::from("three.rs")
            ],
        }]
    );
}
