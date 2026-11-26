//! Module layout rules for the resident tree.
//!
//! - No `mod.rs` files and no generic file names such as `deps.rs`.
//! - Non-leaf modules hold only module declarations and re-exports.
//! - Directory (module) names are descriptive, not single letters or placeholders.
use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
const GENERIC_NAMES: &[&str] = &[
    "mod",
    "deps",
    "dep",
    "lib",
    "util",
    "utils",
    "common",
    "misc",
    "helpers",
    "helper",
    "stuff",
    "base",
    "shared",
    "internal",
    "inner",
    "tmp",
    "temp",
    "foo",
    "bar",
    "types",
];
fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = fs::read_dir(dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}
fn sources() -> (PathBuf, Vec<PathBuf>) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/resident");
    let mut files = Vec::new();
    collect(&root, &mut files);
    (root, files)
}
fn is_test_path(rel: &Path) -> bool {
    rel.components().any(|c| c.as_os_str() == "test" || c.as_os_str() == "tests")
}
fn strip_comments(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::new();
    let mut i = 0;
    let mut in_str = false;
    while i < chars.len() {
        let c = chars[i];
        if in_str {
            out.push(c);
            if c == '\\' && i + 1 < chars.len() {
                out.push(chars[i + 1]);
                i += 1;
            } else if c == '"' {
                in_str = false;
            }
        } else if c == '"' {
            in_str = true;
            out.push(c);
        } else if c == '/' && chars.get(i + 1) == Some(&'/') {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            continue;
        } else if c == '/' && chars.get(i + 1) == Some(&'*') {
            let mut depth = 1;
            i += 2;
            while i < chars.len() && depth > 0 {
                if chars[i] == '/' && chars.get(i + 1) == Some(&'*') {
                    depth += 1;
                    i += 1;
                } else if chars[i] == '*' && chars.get(i + 1) == Some(&'/') {
                    depth -= 1;
                    i += 1;
                }
                i += 1;
            }
            out.push(' ');
            continue;
        } else {
            out.push(c);
        }
        i += 1;
    }
    out
}
/// Returns the statements of a non-leaf module that are neither module
/// declarations nor re-exports.
fn foreign_statements(text: &str) -> Vec<String> {
    let attribute = Regex::new(r#"#!?\[[^\]]*\]"#).unwrap();
    let allowed = Regex::new(
            r"^(pub(\([^)]*\))?\s+)?mod\s+\w+$|^pub(\([^)]*\))?\s+use\s[\s\S]+$",
        )
        .unwrap();
    let cleaned = attribute.replace_all(&strip_comments(text), " ").into_owned();
    let mut rest = cleaned.trim();
    let mut bad = Vec::new();
    while !rest.is_empty() {
        match rest.find(';') {
            Some(end) => {
                let statement = rest[..end].trim();
                if !allowed.is_match(statement) {
                    bad.push(statement.to_string());
                }
                rest = rest[end + 1..].trim();
            }
            None => {
                bad.push(rest.to_string());
                break;
            }
        }
    }
    bad
}
fn is_non_leaf(path: &Path) -> bool {
    let stem = path.file_stem().unwrap().to_string_lossy();
    stem == "mod" || path.with_extension("").is_dir()
}
#[test]
fn no_mod_rs_or_generic_file_names() {
    let (root, files) = sources();
    let mut violations = Vec::new();
    for path in &files {
        let rel = path.strip_prefix(&root).unwrap();
        if is_test_path(rel) {
            continue;
        }
        let stem = path.file_stem().unwrap().to_string_lossy();
        if stem == "mod" || GENERIC_NAMES.contains(&stem.as_ref()) {
            violations.push(rel.display().to_string());
        }
    }
    assert!(violations.is_empty(), "generic file names:\n{}", violations.join("\n"));
}
#[test]
fn non_leaf_modules_contain_only_declarations_and_reexports() {
    let (root, files) = sources();
    let mut violations = Vec::new();
    for path in &files {
        let rel = path.strip_prefix(&root).unwrap();
        if is_test_path(rel) || !is_non_leaf(path) {
            continue;
        }
        let bad = foreign_statements(&fs::read_to_string(path).unwrap());
        if !bad.is_empty() {
            violations
                .push(format!("{} ({} foreign statements)", rel.display(), bad.len()));
        }
    }
    assert!(
        violations.is_empty(), "non-leaf modules with implementation:\n{}", violations
        .join("\n")
    );
}
#[test]
fn detector_accepts_declarations_and_reexports() {
    let text = "//! doc\npub mod a;\nmod b;\npub(crate) mod c;\npub use self::a::{X, Y};\n\
                #[cfg(test)]\n#[path = \"test/x.rs\"]\nmod tests;\n";
    assert!(foreign_statements(text).is_empty());
}
#[test]
fn detector_flags_implementation() {
    assert_eq!(foreign_statements("pub mod a;\nfn f() { let x = 1; }\n").len(), 2);
    assert_eq!(foreign_statements("use std::fmt;\n").len(), 1);
    assert_eq!(foreign_statements("pub struct S;\n").len(), 1);
    assert_eq!(foreign_statements("mod inline { pub fn f() {} }\n").len(), 1);
}
#[test]
fn detector_ignores_comments() {
    assert!(
        foreign_statements("// fn f() {}\n/* struct S; */\npub mod a;\n").is_empty()
    );
}
