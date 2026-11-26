use super::*;
use crate::ir::{Object, ObjectKind};
use std::path::PathBuf;
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("rustir-edges-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}
fn project_with(names: &[(&str, &[&str])]) -> Project {
    let mut project = Project::new(PathBuf::from("/p"));
    for (name, dependencies) in names {
        project
            .objects
            .insert(Object {
                name: name.to_string(),
                kind: ObjectKind::Struct,
                body: String::new(),
                dependencies: dependencies.iter().map(|d| d.to_string()).collect(),
            });
    }
    project
}
#[test]
fn reads_one_dependency_per_line() {
    let edges = read_edges(Path::new("e"), "A\tB\nA\tstd::fmt::Display\nC\t\n").unwrap();
    let dependencies = &edges.dependencies;
    assert_eq!(
        dependencies["A"], vec!["B".to_string(), "std::fmt::Display".to_string()]
    );
    assert!(dependencies["C"].is_empty());
    assert!(edges.aliases.is_empty());
}
#[test]
fn third_column_names_the_alias_and_counts_as_a_dependency() {
    let edges = read_edges(Path::new("e"), "A\tB\tAlias\nA\tC\n").unwrap();
    assert_eq!(edges.dependencies["A"], vec!["B".to_string(), "C".to_string()]);
    assert_eq!(edges.aliases["A"] ["Alias"], "B");
    assert_eq!(edges.aliases["A"].len(), 1);
}
#[test]
fn malformed_line_is_a_parse_error_with_its_number() {
    let result = read_edges(Path::new("e"), "A\tB\nbroken\n");
    match result {
        Err(Error::Parse { message, .. }) => assert!(message.starts_with("line 2:")),
        other => panic!("unexpected {other:?}"),
    }
}
#[test]
fn loaded_edges_replace_provisional_dependencies() {
    let dir = scratch("replace");
    let file = dir.join("edges.txt");
    fs::write(&file, "A\tB\nA\tstd::fmt::Display\nB\t\n").unwrap();
    let mut project = project_with(&[("A", &["Old"]), ("B", &["A"]), ("C", &["A"])]);
    load_edges(&mut project, &file).unwrap();
    let objects = &project.objects;
    assert_eq!(
        objects.get("A").unwrap().dependencies, vec!["B".to_string(), "std::fmt::Display"
        .to_string()]
    );
    assert!(objects.get("B").unwrap().dependencies.is_empty());
    assert_eq!(objects.get("C").unwrap().dependencies, vec!["A".to_string()]);
}
#[test]
fn loaded_aliases_are_kept_for_known_owners_only() {
    let dir = scratch("aliases");
    let file = dir.join("edges.txt");
    fs::write(&file, "A\tB\tAlias\nGhost\tB\tOther\nB\t\n").unwrap();
    let mut project = project_with(&[("A", &[]), ("B", &[])]);
    load_edges(&mut project, &file).unwrap();
    assert_eq!(project.aliases["A"] ["Alias"], "B");
    assert!(! project.aliases.contains_key("Ghost"));
    assert_eq!(project.objects.get("A").unwrap().dependencies, vec!["B".to_string()]);
}
#[test]
fn edges_of_unknown_objects_are_ignored() {
    let dir = scratch("unknown");
    let file = dir.join("edges.txt");
    fs::write(&file, "Ghost\tA\n").unwrap();
    let mut project = project_with(&[("A", &[])]);
    load_edges(&mut project, &file).unwrap();
    assert_eq!(project.objects.len(), 1);
}
#[test]
fn missing_file_is_not_found() {
    let file = scratch("missing").join("absent.txt");
    let mut project = project_with(&[]);
    assert_eq!(load_edges(& mut project, & file), Err(Error::NotFound(file.clone())));
}
