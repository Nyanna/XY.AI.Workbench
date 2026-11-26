use super::*;
use crate::ir::{Module, Object, ObjectKind};
use std::path::PathBuf;

fn object(name: &str, kind: ObjectKind, dependencies: &[&str]) -> Object {
    Object {
        name: name.into(),
        kind,
        body: String::new(),
        dependencies: dependencies.iter().map(|d| d.to_string()).collect(),
    }
}

#[test]
fn empty_project_renders_nothing() {
    assert_eq!(render(&Project::new(PathBuf::from("/p"))), "");
}

#[test]
fn renders_one_line_per_module() {
    let mut project = Project::new(PathBuf::from("/p"));
    for name in ["a", "b"] {
        project.modules.push(Module { name: name.into() });
    }
    assert_eq!(render(&project), "mod a;\nmod b;\n");
}

#[test]
fn renders_objects_with_dependencies() {
    let mut project = Project::new(PathBuf::from("/p"));
    project.objects.insert(object("A", ObjectKind::Struct, &["B", "C"]));
    project.objects.insert(object("B", ObjectKind::Function, &[]));
    assert_eq!(render(&project), "struct A -> B, C\nfn B\n");
}
