use super::*;
use crate::ir::ObjectKind;
use std::path::PathBuf;

fn object(name: &str, kind: ObjectKind, dependencies: &[&str]) -> Object {
    Object {
        name: name.into(),
        kind,
        body: String::new(),
        dependencies: dependencies.iter().map(|d| d.to_string()).collect(),
    }
}

fn project(objects: Vec<Object>, imports: &[(&str, &[&str])]) -> Project {
    let mut project = Project::new(PathBuf::from("/p"));
    for object in objects {
        project.objects.insert(object);
    }
    for (owner, paths) in imports {
        project
            .imports
            .insert(owner.to_string(), paths.iter().map(|p| p.to_string()).collect());
    }
    project
}

fn block(project: &Project, name: &str) -> String {
    let layout = Layout::new(&project.objects);
    use_block(project, &layout, project.objects.get(name).unwrap())
}

#[test]
fn local_names_follow_aliases_and_skip_globs() {
    assert_eq!(local_name("std::fmt::Display"), Some("Display"));
    assert_eq!(local_name("std::io::Result as IoResult"), Some("IoResult"));
    assert_eq!(local_name("std::fmt::Write as _"), None);
    assert_eq!(local_name("std::io::prelude::*"), None);
}

#[test]
fn internal_dependencies_become_module_imports() {
    let project = project(
        vec![
            object("Alpha", ObjectKind::Struct, &["Beta", "Gamma"]),
            object("Beta", ObjectKind::Struct, &[]),
            object("Gamma", ObjectKind::Function, &[]),
        ],
        &[],
    );
    assert_eq!(block(&project, "Alpha"), "use crate::beta::Beta;\nuse crate::gamma::Gamma;\n\n");
}

#[test]
fn unknown_self_and_macro_dependencies_are_skipped() {
    let project = project(
        vec![
            object("Alpha", ObjectKind::Struct, &["Alpha", "std::fmt::Display", "shout"]),
            object("shout", ObjectKind::Macro, &[]),
        ],
        &[],
    );
    assert_eq!(block(&project, "Alpha"), "");
}

#[test]
fn external_imports_follow_the_internal_ones_sorted() {
    let project = project(
        vec![object("Alpha", ObjectKind::Struct, &["Beta"]), object("Beta", ObjectKind::Struct, &[])],
        &[("Alpha", &["std::path::PathBuf", "std::fmt"])],
    );
    assert_eq!(
        block(&project, "Alpha"),
        "use crate::beta::Beta;\nuse std::fmt;\nuse std::path::PathBuf;\n\n"
    );
}

#[test]
fn internal_names_win_over_external_imports() {
    let project = project(
        vec![object("Alpha", ObjectKind::Struct, &["Result"]), object("Result", ObjectKind::TypeAlias, &[])],
        &[("Alpha", &["std::fmt::Result", "std::fmt::Write as _"])],
    );
    assert_eq!(
        block(&project, "Alpha"),
        "use crate::result::Result;\nuse std::fmt::Write as _;\n\n"
    );
}

#[test]
fn an_import_named_like_the_object_itself_is_dropped() {
    let project = project(
        vec![object("Alpha", ObjectKind::Struct, &[])],
        &[("Alpha", &["other::Alpha"])],
    );
    assert_eq!(block(&project, "Alpha"), "");
}
