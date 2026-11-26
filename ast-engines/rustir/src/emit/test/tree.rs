use super::*;
use crate::ir::Object;
use std::path::PathBuf;
fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir()
        .join(format!("rustir-emit-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    dir
}
fn sample() -> Project {
    let mut project = Project::new(PathBuf::from("/work/demo"));
    let objects = [
        ("Alpha", ObjectKind::Struct, "pub struct Alpha { b: Beta }", vec!["Beta"]),
        ("Beta", ObjectKind::Struct, "struct Beta;", vec![]),
        ("main", ObjectKind::Function, "fn main() {}", vec![]),
    ];
    for (name, kind, body, dependencies) in objects {
        project
            .objects
            .insert(Object {
                name: name.into(),
                kind,
                body: body.into(),
                dependencies: dependencies.into_iter().map(String::from).collect(),
            });
    }
    project
}
#[test]
fn every_object_gets_a_flat_module_file() {
    let dir = scratch("flat");
    write_crate(&sample(), &dir).unwrap();
    let read = |name: &str| fs::read_to_string(dir.join("src").join(name)).unwrap();
    assert_eq!(
        read("alpha.rs"), "use crate::beta::Beta;\n\npub struct Alpha { b: Beta }\n"
    );
    assert_eq!(read("beta.rs"), "pub(crate) struct Beta;\n");
    assert_eq!(read("main_.rs"), "pub fn main() {}\n");
}
#[test]
fn root_files_are_written() {
    let dir = scratch("root");
    write_crate(&sample(), &dir).unwrap();
    let lib = fs::read_to_string(dir.join("src/lib.rs")).unwrap();
    assert!(lib.contains("pub mod alpha;\nmod beta;\npub mod main_;\n"));
    assert!(lib.contains("pub use alpha::Alpha;"));
    assert_eq!(
        fs::read_to_string(dir.join("src/main.rs")).unwrap(), "use demo::main_::main;\n"
    );
    let manifest = fs::read_to_string(dir.join("Cargo.toml")).unwrap();
    assert!(manifest.contains("name = \"demo\"") && manifest.contains("[[bin]]"));
}
#[test]
fn libraries_without_entry_point_have_no_main_file() {
    let dir = scratch("library");
    let mut project = sample();
    project.objects = crate::ir::ObjectTable::new();
    write_crate(&project, &dir).unwrap();
    assert!(! dir.join("src/main.rs").exists());
    assert!(dir.join("src/lib.rs").is_file());
}
#[test]
fn a_foreign_non_empty_directory_is_refused() {
    let dir = scratch("foreign");
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("notes.txt"), "keep").unwrap();
    assert_eq!(write_crate(& sample(), & dir), Err(Error::TargetNotEmpty(dir.clone())));
    assert!(dir.join("notes.txt").is_file());
}
#[test]
fn an_existing_empty_directory_is_accepted() {
    let dir = scratch("empty");
    fs::create_dir_all(&dir).unwrap();
    assert_eq!(write_crate(& sample(), & dir), Ok(()));
}
#[test]
fn a_generated_directory_is_regenerated_without_stale_modules() {
    let dir = scratch("again");
    write_crate(&sample(), &dir).unwrap();
    fs::write(dir.join("src/stale.rs"), "").unwrap();
    write_crate(&sample(), &dir).unwrap();
    assert!(! dir.join("src/stale.rs").exists());
    assert!(dir.join("src/alpha.rs").is_file());
}
#[test]
fn sources_only_requires_an_existing_manifest() {
    let dir = scratch("no-manifest");
    fs::create_dir_all(&dir).unwrap();
    assert_eq!(
        write_sources(& sample(), & dir), Err(Error::NotFound(dir.join("Cargo.toml")))
    );
    assert!(! dir.join("src").exists());
}
#[test]
fn sources_only_replaces_src_and_keeps_everything_else() {
    let dir = scratch("sources-only");
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(dir.join("Cargo.toml"), "[package]\nname = \"mine\"\n").unwrap();
    fs::write(dir.join("notes.txt"), "keep").unwrap();
    fs::write(dir.join("src/stale.rs"), "").unwrap();
    write_sources(&sample(), &dir).unwrap();
    assert_eq!(
        fs::read_to_string(dir.join("Cargo.toml")).unwrap(),
        "[package]\nname = \"mine\"\n"
    );
    assert_eq!(fs::read_to_string(dir.join("notes.txt")).unwrap(), "keep");
    assert!(! dir.join("src/stale.rs").exists());
    assert!(dir.join("src/alpha.rs").is_file() && dir.join("src/lib.rs").is_file());
    assert!(! dir.join("src/main.rs").exists());
}
