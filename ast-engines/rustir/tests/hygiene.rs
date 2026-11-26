use std::fs;
use std::path::{Path, PathBuf};

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

fn all_sources() -> Vec<PathBuf> {
    let mut files = Vec::new();
    rust_files(&src_dir(), &mut files);
    files.sort();
    files
}

fn file_name(path: &Path) -> &str {
    path.file_name().unwrap().to_str().unwrap()
}

fn in_test_dir(path: &Path) -> bool {
    path.parent().is_some_and(|p| file_name(p) == "test")
}

fn is_non_leaf(path: &Path) -> bool {
    matches!(file_name(path), "mod.rs" | "lib.rs")
}

fn is_leaf(path: &Path) -> bool {
    !in_test_dir(path) && !is_non_leaf(path) && file_name(path) != "main.rs"
}

fn layer(module: &str) -> Option<u8> {
    match module {
        "error" => Some(0),
        "ir" => Some(1),
        "parser" | "transform" | "emit" => Some(2),
        "cli" => Some(3),
        _ => None,
    }
}

fn top_module(path: &Path) -> Option<String> {
    let relative = path.strip_prefix(src_dir()).ok()?;
    let first = relative.components().next()?.as_os_str().to_str()?;
    let name = first.strip_suffix(".rs").unwrap_or(first);
    layer(name).map(|_| name.to_string())
}

#[test]
fn sources_are_found() {
    assert!(all_sources().len() > 5);
}

#[test]
fn non_leaf_modules_only_declare_and_reexport() {
    for path in all_sources().iter().filter(|p| is_non_leaf(p)) {
        let text = fs::read_to_string(path).unwrap();
        for line in text.lines().map(str::trim).filter(|l| !l.is_empty()) {
            let allowed = line.starts_with("//")
                || ((line.starts_with("mod ")
                    || line.starts_with("pub mod ")
                    || line.starts_with("use ")
                    || line.starts_with("pub use "))
                    && line.ends_with(';'));
            assert!(allowed, "{}: not a declaration or re-export: {line}", path.display());
        }
    }
}

#[test]
fn leaf_modules_declare_their_test_module() {
    for path in all_sources().iter().filter(|p| is_leaf(p)) {
        let name = file_name(path).trim_end_matches(".rs");
        let text = fs::read_to_string(path).unwrap();
        let block = format!("#[cfg(test)]\n#[path = \"test/{name}.rs\"]\nmod tests;\n");
        assert!(text.ends_with(&block), "{}: missing trailing test block", path.display());
        let test_file = path.parent().unwrap().join("test").join(format!("{name}.rs"));
        assert!(test_file.is_file(), "missing {}", test_file.display());
    }
}

#[test]
fn test_files_belong_to_a_leaf_module() {
    for path in all_sources().iter().filter(|p| in_test_dir(p)) {
        let leaf = path.parent().unwrap().parent().unwrap().join(file_name(path));
        assert!(leaf.is_file(), "orphan test file {}", path.display());
    }
}

#[test]
fn dependencies_point_only_to_lower_layers() {
    for path in all_sources() {
        let Some(own) = top_module(&path) else { continue };
        let own_layer = layer(&own).unwrap();
        let text = fs::read_to_string(&path).unwrap();
        assert!(!text.contains("super::super"), "{}: super::super path", path.display());
        for (name, other_layer) in ["error", "ir", "parser", "transform", "emit", "cli"]
            .iter()
            .map(|n| (*n, layer(n).unwrap()))
        {
            if name == own || !text.contains(&format!("crate::{name}")) {
                continue;
            }
            assert!(
                other_layer < own_layer,
                "{}: {own} must not depend on {name}",
                path.display()
            );
        }
    }
}

#[test]
fn no_use_aliases() {
    for path in all_sources() {
        let text = fs::read_to_string(&path).unwrap();
        for line in text.lines().map(str::trim) {
            let is_use = line.starts_with("use ") || line.starts_with("pub use ");
            assert!(!(is_use && line.contains(" as ")), "{}: alias in {line}", path.display());
        }
    }
}

#[test]
fn tests_do_not_depend_on_logging() {
    let forbidden = ["log::", "tracing::", "env_logger", "testing_logger", "tracing_test"];
    let mut files: Vec<PathBuf> = all_sources().into_iter().filter(|p| in_test_dir(p)).collect();
    rust_files(&Path::new(env!("CARGO_MANIFEST_DIR")).join("tests"), &mut files);
    for path in files.iter().filter(|p| file_name(p) != "hygiene.rs") {
        let text = fs::read_to_string(path).unwrap();
        for token in forbidden {
            assert!(!text.contains(token), "{}: uses {token}", path.display());
        }
    }
}
