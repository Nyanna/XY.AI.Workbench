use super::*;
use crate::ir::{Object, ObjectKind};

fn flatten(source: &str) -> Vec<(Option<String>, String)> {
    let item: syn::ItemUse = syn::parse_str(source).unwrap();
    flatten_use(Path::new("x.rs"), &item.tree)
        .into_iter()
        .map(|import| (import.local, import.path))
        .collect()
}

fn entry(local: Option<&str>, path: &str) -> (Option<String>, String) {
    (local.map(str::to_string), path.to_string())
}

fn import(file: &str, local: Option<&str>, path: &str) -> Import {
    Import {
        file: PathBuf::from(file),
        local: local.map(str::to_string),
        path: path.to_string(),
    }
}

fn origin(file: &str, names: &[&str]) -> (PathBuf, BTreeSet<String>) {
    (PathBuf::from(file), names.iter().map(|n| n.to_string()).collect())
}

fn select(imports: &[Import], origins: &[(PathBuf, BTreeSet<String>)]) -> Vec<String> {
    let mut objects = ObjectTable::new();
    objects.insert(Object {
        name: "Local".into(),
        kind: ObjectKind::Struct,
        body: String::new(),
        dependencies: Vec::new(),
    });
    let qualifiers: BTreeSet<String> = ["ir".to_string()].into_iter().collect();
    select_imports(imports, origins, &qualifiers, &objects)
}

#[test]
fn simple_path_names_its_last_segment() {
    assert_eq!(flatten("use std::fmt::Display;"), vec![entry(Some("Display"), "std::fmt::Display")]);
}

#[test]
fn groups_expand_and_self_names_the_parent() {
    assert_eq!(
        flatten("use std::io::{self, Write};"),
        vec![entry(Some("io"), "std::io"), entry(Some("Write"), "std::io::Write")]
    );
}

#[test]
fn renames_keep_the_alias_as_local_name() {
    assert_eq!(flatten("use std::io::Result as IoResult;"), vec![entry(Some("IoResult"), "std::io::Result as IoResult")]);
}

#[test]
fn anonymous_imports_have_no_local_name() {
    assert_eq!(flatten("use std::fmt::Write as _;"), vec![entry(None, "std::fmt::Write as _")]);
    assert_eq!(flatten("use std::io::prelude::*;"), vec![entry(None, "std::io::prelude::*")]);
}

#[test]
fn referenced_imports_come_first() {
    let imports = [import("a.rs", Some("Unused"), "std::a::Unused"), import("a.rs", Some("Used"), "std::b::Used")];
    let selected = select(&imports, &[origin("a.rs", &["Used"])]);
    assert_eq!(selected, vec!["std::b::Used", "std::a::Unused"]);
}

#[test]
fn imports_of_other_files_are_not_selected() {
    let imports = [import("b.rs", Some("Used"), "std::b::Used")];
    assert!(select(&imports, &[origin("a.rs", &["Used"])]).is_empty());
}

#[test]
fn internal_roots_are_not_selected() {
    let imports = [
        import("a.rs", Some("X"), "crate::x::X"),
        import("a.rs", Some("Y"), "ir::Y"),
        import("a.rs", None, "Local::*"),
        import("a.rs", Some("Z"), "super::Z"),
    ];
    assert!(select(&imports, &[origin("a.rs", &["X", "Y", "Z"])]).is_empty());
}

#[test]
fn unreferenced_imports_do_not_shadow_referenced_names() {
    let imports = [import("a.rs", Some("Result"), "std::fmt::Result"), import("b.rs", Some("Result"), "std::io::Result")];
    let origins = [origin("a.rs", &[]), origin("b.rs", &["Result"])];
    assert_eq!(select(&imports, &origins), vec!["std::io::Result"]);
}

#[test]
fn globs_follow_referenced_imports_and_duplicates_collapse() {
    let imports = [
        import("a.rs", None, "std::io::prelude::*"),
        import("a.rs", Some("Used"), "std::b::Used"),
        import("b.rs", Some("Used"), "std::b::Used"),
    ];
    let origins = [origin("a.rs", &["Used"]), origin("b.rs", &["Used"])];
    assert_eq!(select(&imports, &origins), vec!["std::b::Used", "std::io::prelude::*"]);
}
