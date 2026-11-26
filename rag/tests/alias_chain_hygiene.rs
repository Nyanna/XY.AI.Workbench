//! `use` declarations must not form alias chains: an alias must point at a direct path,
//! never at another alias; paths must not run through alias modules (`lNNN`) and must not
//! start at a bare module segment (`_N`).
use regex::Regex;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
fn collect(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()))
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
fn strip_comments(text: &str) -> String {
    let block = Regex::new(r"(?s)/\*.*?\*/").unwrap();
    let line = Regex::new(r"(?m)//.*$").unwrap();
    let text = block.replace_all(text, "");
    line.replace_all(&text, "").into_owned()
}
fn line_at(text: &str, offset: usize) -> usize {
    text[..offset].bytes().filter(|b| *b == b'\n').count() + 1
}
fn is_bare_module_segment(segment: &str) -> bool {
    let mut chars = segment.chars();
    chars.next() == Some('_') && chars.clone().count() >= 1
        && chars.all(|c| c.is_ascii_alphanumeric())
}
fn alias_chain_violations(text: &str) -> Vec<(usize, String)> {
    let text = strip_comments(text);
    let use_re = Regex::new(r"(?m)(?:^|[;{}\s])use\s+([^;]+);").unwrap();
    let alias_re = Regex::new(r"\bas\s+([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    let root_re = Regex::new(r"^\s*(?:::)?\{?\s*([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    let uses: Vec<(usize, String)> = use_re
        .captures_iter(&text)
        .map(|c| {
            let m = c.get(1).unwrap();
            (m.start(), m.as_str().to_string())
        })
        .collect();
    let mut aliases = BTreeSet::new();
    for (_, body) in &uses {
        for c in alias_re.captures_iter(body) {
            let name = c[1].to_string();
            if name != "_" {
                aliases.insert(name);
            }
        }
    }
    let segment_re = Regex::new(r"[A-Za-z_][A-Za-z0-9_]*").unwrap();
    let alias_segment_re = Regex::new(r"^l[0-9]{3}$").unwrap();
    let mut found = Vec::new();
    for (offset, body) in &uses {
        let Some(root) = root_re.captures(body).map(|c| c[1].to_string()) else {
            continue;
        };
        let compact = body.split_whitespace().collect::<Vec<_>>().join(" ");
        let line = line_at(&text, *offset);
        let path_part = body.split(" as ").next().unwrap_or(body);
        let through_alias = segment_re
            .find_iter(path_part)
            .skip(1)
            .any(|m| alias_segment_re.is_match(m.as_str()));
        if is_bare_module_segment(&root) {
            found
                .push((line, format!("starts at bare module `{root}`: use {compact};")));
        } else if aliases.contains(&root) && alias_re.is_match(body) {
            found
                .push((
                    line,
                    format!("alias defined through alias `{root}`: use {compact};"),
                ));
        } else if through_alias {
            found
                .push((
                    line,
                    format!("path runs through an alias module: use {compact};"),
                ));
        }
    }
    found
}
#[test]
fn no_alias_chains_in_use_paths() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut files = Vec::new();
    collect(&root, &mut files);
    assert!(! files.is_empty(), "no sources found under {}", root.display());
    let mut report = Vec::new();
    for file in &files {
        let text = fs::read_to_string(file).unwrap();
        for (line, message) in alias_chain_violations(&text) {
            let rel = file.strip_prefix(&root).unwrap_or(file);
            report.push(format!("{}:{line}: {message}", rel.display()));
        }
    }
    assert!(
        report.is_empty(), "{} alias chain violation(s):\n{}", report.len(), report
        .join("\n")
    );
}
#[test]
fn detector_flags_alias_chains() {
    let src = "use crate::a::b as l090;\nuse l090::l080 as l080;\nuse l080::l070::a::E;\nuse l080::a::F;\n";
    let found = alias_chain_violations(src);
    assert_eq!(found.iter().map(| f | f.0).collect::< Vec < _ >> (), vec![2, 3]);
}
#[test]
fn detector_flags_bare_module_roots() {
    let src = "use _9::l080 as l080;\n";
    assert_eq!(alias_chain_violations(src).len(), 1);
}
#[test]
fn detector_accepts_direct_paths_and_single_aliases() {
    let src = "use std::sync::Mutex as StdMutex;\n\
               use crate::a::b as l090;\n\
               use super::c::D;\n\
               use self::e::F;\n\
               use serde::de::{Deserializer, Error as _};\n";
    assert!(alias_chain_violations(src).is_empty());
}
#[test]
fn detector_ignores_comments() {
    let src = "// use l090::x;\nuse crate::a::b as l090;\n/* use l090::y; */\n";
    assert!(alias_chain_violations(src).is_empty());
}
