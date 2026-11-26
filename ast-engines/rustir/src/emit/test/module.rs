use super::*;
use crate::ir::{dependents, ObjectKind};
use std::path::PathBuf;

fn object(name: &str, kind: ObjectKind, body: &str, dependencies: &[&str]) -> Object {
    Object {
        name: name.into(),
        kind,
        body: body.into(),
        dependencies: dependencies.iter().map(|d| d.to_string()).collect(),
    }
}

fn render_one(objects: Vec<Object>, name: &str, entry: bool) -> ModuleSource {
    let mut project = Project::new(PathBuf::from("/p"));
    for object in objects {
        project.objects.insert(object);
    }
    project
        .imports
        .insert("Alpha".into(), vec!["std::fmt".into()]);
    project.qualifiers.insert("ir".into());
    let layout = Layout::new(&project.objects);
    let incoming = dependents(&project.objects);
    let target = project.objects.get(name).unwrap().clone();
    module_source(&project, &layout, &incoming, &target, entry)
}

fn pair() -> Vec<Object> {
    vec![
        object("Alpha", ObjectKind::Struct, "struct Alpha { b: Beta }", &["Beta"]),
        object("Beta", ObjectKind::Struct, "struct Beta { n: u8 }", &[]),
    ]
}

#[test]
fn dependents_make_an_object_crate_visible() {
    let beta = render_one(pair(), "Beta", false);
    assert_eq!(beta.text, "pub(crate) struct Beta { pub(crate) n: u8 }\n");
    assert!(!beta.public);
}

#[test]
fn objects_without_dependents_keep_their_visibility() {
    let alpha = render_one(pair(), "Alpha", false);
    assert_eq!(alpha.text, "use crate::beta::Beta;\nuse std::fmt;\n\nstruct Alpha { b: Beta }\n");
}

#[test]
fn public_objects_are_reported_public() {
    let objects = vec![object("Alpha", ObjectKind::Struct, "pub struct Alpha;", &[])];
    assert!(render_one(objects, "Alpha", false).public);
}

#[test]
fn the_entry_object_is_made_public() {
    let objects = vec![object("main", ObjectKind::Function, "fn main() {}", &[])];
    let rendered = render_one(objects, "main", true);
    assert_eq!(rendered.text, "pub fn main() {}\n");
    assert!(rendered.public);
}

#[test]
fn paths_into_the_original_tree_are_rewritten() {
    let objects = vec![
        object("Alpha", ObjectKind::Function, "fn alpha() -> crate::ir::Beta { todo!() }", &["Beta"]),
        object("Beta", ObjectKind::Struct, "struct Beta;", &[]),
    ];
    let alpha = render_one(objects, "Alpha", false);
    assert!(alpha.text.contains("-> crate::beta::Beta {"));
}

#[test]
fn unparsable_bodies_are_written_unchanged() {
    let objects = vec![object("Alpha", ObjectKind::Struct, "struct {", &[])];
    let rendered = render_one(objects, "Alpha", false);
    assert_eq!(rendered.text, "use std::fmt;\n\nstruct {\n");
    assert!(!rendered.public);
}
