//! Dependencies inside the resident tree must never point upward.
//!
//! A module may only depend on modules of its own layer or of lower layers. The layer of a module is the number of its nearest `lNNN` path segment.
//! Test files and `#[cfg(test)]` items are not part of the check.
use regex::Regex;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
type ModPath = Vec<String>;
struct SourceFile {
    rel: String,
    module: ModPath,
    text: String,
}
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
fn is_test_file(rel: &Path) -> bool {
    rel.components()
        .any(|c| {
            let name = c.as_os_str().to_string_lossy();
            name == "test" || name == "tests" || name == "tests.rs"
        })
}
fn module_of(rel: &Path) -> ModPath {
    let parts: Vec<String> = rel
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    let mut module = vec!["resident".to_string()];
    for (index, part) in parts.iter().enumerate() {
        if index + 1 == parts.len() {
            let stem = part.strip_suffix(".rs").unwrap_or(part);
            if stem != "mod" {
                module.push(stem.to_string());
            }
        } else {
            module.push(part.clone());
        }
    }
    module
}
fn load_sources() -> (Vec<SourceFile>, BTreeSet<ModPath>) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/resident");
    let mut paths = Vec::new();
    collect(&root, &mut paths);
    let mut files = Vec::new();
    let mut modules = BTreeSet::new();
    for path in paths {
        let rel = path.strip_prefix(&root).unwrap().to_path_buf();
        if is_test_file(&rel) {
            continue;
        }
        let module = module_of(&rel);
        modules.insert(module.clone());
        files
            .push(SourceFile {
                rel: rel.to_string_lossy().replace('\\', "/"),
                module,
                text: fs::read_to_string(&path).unwrap(),
            });
    }
    (files, modules)
}
fn layer_number(segment: &str) -> Option<u32> {
    let digits = segment.strip_prefix('l')?;
    if digits.len() == 3 && digits.chars().all(|c| c.is_ascii_digit()) {
        digits.parse().ok()
    } else {
        None
    }
}
fn layer_of(module: &[String]) -> u32 {
    module.iter().rev().find_map(|s| layer_number(s)).unwrap_or(u32::MAX)
}
fn blank(out: &mut String, c: char) {
    if c == '\n' {
        out.push('\n');
    } else {
        for _ in 0..c.len_utf8() {
            out.push(' ');
        }
    }
}
fn raw_string_end(chars: &[char], start: usize) -> Option<usize> {
    let mut j = start + 1;
    let mut hashes = 0;
    while chars.get(j) == Some(&'#') {
        hashes += 1;
        j += 1;
    }
    if chars.get(j) != Some(&'"') {
        return None;
    }
    j += 1;
    while j < chars.len() {
        if chars[j] == '"' && (0..hashes).all(|k| chars.get(j + 1 + k) == Some(&'#')) {
            return Some(j + 1 + hashes);
        }
        j += 1;
    }
    Some(chars.len())
}
fn strip_comments_and_literals(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let next = chars.get(i + 1).copied();
        let prev_is_word = i > 0
            && (chars[i - 1].is_alphanumeric() || chars[i - 1] == '_');
        if c == '/' && next == Some('/') {
            while i < chars.len() && chars[i] != '\n' {
                blank(&mut out, chars[i]);
                i += 1;
            }
        } else if c == '/' && next == Some('*') {
            let mut depth = 0usize;
            while i < chars.len() {
                if chars[i] == '/' && chars.get(i + 1) == Some(&'*') {
                    depth += 1;
                    blank(&mut out, '/');
                    blank(&mut out, '*');
                    i += 2;
                } else if chars[i] == '*' && chars.get(i + 1) == Some(&'/') {
                    depth -= 1;
                    blank(&mut out, '*');
                    blank(&mut out, '/');
                    i += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    blank(&mut out, chars[i]);
                    i += 1;
                }
            }
        } else if c == 'r' && !prev_is_word && raw_string_end(&chars, i).is_some() {
            let end = raw_string_end(&chars, i).unwrap();
            while i < end {
                blank(&mut out, chars[i]);
                i += 1;
            }
        } else if c == '"' {
            blank(&mut out, c);
            i += 1;
            while i < chars.len() {
                let d = chars[i];
                blank(&mut out, d);
                i += 1;
                if d == '\\' && i < chars.len() {
                    blank(&mut out, chars[i]);
                    i += 1;
                } else if d == '"' {
                    break;
                }
            }
        } else if c == '\'' {
            let end = if next == Some('\\') {
                (i + 3..chars.len()).find(|&j| chars[j] == '\'').map(|j| j + 1)
            } else if chars.get(i + 2) == Some(&'\'') {
                Some(i + 3)
            } else {
                None
            };
            match end {
                Some(end) => {
                    while i < end {
                        blank(&mut out, chars[i]);
                        i += 1;
                    }
                }
                None => {
                    out.push(c);
                    i += 1;
                }
            }
        } else {
            out.push(c);
            i += 1;
        }
    }
    out
}
fn matching(b: &[u8], start: usize, open: u8, close: u8) -> usize {
    let mut depth = 0i32;
    let mut i = start;
    while i < b.len() {
        if b[i] == open {
            depth += 1;
        } else if b[i] == close {
            depth -= 1;
            if depth == 0 {
                return i + 1;
            }
        }
        i += 1;
    }
    b.len()
}
fn item_end(text: &str, start: usize) -> usize {
    let b = text.as_bytes();
    let mut i = start;
    loop {
        while i < b.len() && b[i].is_ascii_whitespace() {
            i += 1;
        }
        if b[i..].starts_with(b"#[") {
            i = matching(b, i + 1, b'[', b']');
        } else {
            break;
        }
    }
    let is_use = Regex::new(r"^(pub(\s*\([^)]*\))?\s+)?use\b")
        .unwrap()
        .is_match(&text[i..]);
    let mut depth = 0i32;
    while i < b.len() {
        match b[i] {
            b'(' | b'[' => depth += 1,
            b')' | b']' => depth -= 1,
            b';' if depth <= 0 => return i + 1,
            b'{' if depth <= 0 && !is_use => return matching(b, i, b'{', b'}'),
            _ => {}
        }
        i += 1;
    }
    b.len()
}
fn blank_range(bytes: &mut [u8], range: std::ops::Range<usize>) {
    for byte in &mut bytes[range] {
        if *byte != b'\n' {
            *byte = b' ';
        }
    }
}
fn drop_cfg_test_items(text: &str) -> String {
    let attribute = Regex::new(r"#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]").unwrap();
    let mut bytes = text.as_bytes().to_vec();
    let mut from = 0;
    while let Some(found) = attribute.find_at(text, from) {
        let end = item_end(text, found.end());
        blank_range(&mut bytes, found.start()..end);
        from = end.max(found.end());
    }
    String::from_utf8(bytes).unwrap()
}
fn split_uses(text: &str) -> (Vec<(usize, String)>, String) {
    let keyword = Regex::new(r"\buse\b").unwrap();
    let mut rest = text.as_bytes().to_vec();
    let mut uses = Vec::new();
    let mut from = 0;
    while let Some(found) = keyword.find_at(text, from) {
        let Some(len) = text[found.end()..].find(';') else {
            break;
        };
        let end = found.end() + len;
        uses.push((found.start(), text[found.end()..end].to_string()));
        blank_range(&mut rest, found.start()..end + 1);
        from = end + 1;
    }
    (uses, String::from_utf8(rest).unwrap())
}
fn tokenize(body: &str) -> Vec<String> {
    let chars: Vec<char> = body.chars().collect();
    let mut tokens = Vec::new();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_alphanumeric() || c == '_' {
            let begin = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            tokens.push(chars[begin..i].iter().collect());
            continue;
        }
        if c == ':' && chars.get(i + 1) == Some(&':') {
            tokens.push("::".to_string());
            i += 2;
            continue;
        }
        if matches!(c, '{' | '}' | ',' | '*') {
            tokens.push(c.to_string());
        }
        i += 1;
    }
    tokens
}
fn expand(
    tokens: &[String],
    pos: &mut usize,
    prefix: &[String],
    out: &mut Vec<Vec<String>>,
) {
    let mut path = prefix.to_vec();
    loop {
        match tokens.get(*pos).map(String::as_str) {
            Some("::") => *pos += 1,
            Some("as") => *pos += 2,
            Some("*") => {
                *pos += 1;
                break;
            }
            Some("{") => {
                *pos += 1;
                loop {
                    expand(tokens, pos, &path, out);
                    match tokens.get(*pos).map(String::as_str) {
                        Some(",") => *pos += 1,
                        Some("}") => {
                            *pos += 1;
                            break;
                        }
                        _ => break,
                    }
                }
                return;
            }
            Some(",") | Some("}") | None => break,
            Some(ident) => {
                path.push(ident.to_string());
                *pos += 1;
            }
        }
    }
    if path.len() > prefix.len() {
        if path.len() > 1 && path.last().map(String::as_str) == Some("self") {
            path.pop();
        }
        out.push(path);
    }
}
fn expand_use(body: &str) -> Vec<Vec<String>> {
    let tokens = tokenize(body);
    let mut out = Vec::new();
    let mut pos = 0;
    expand(&tokens, &mut pos, &[], &mut out);
    out
}
fn resolve(path: &[String], source: &[String]) -> Option<ModPath> {
    let (head, tail) = path.split_first()?;
    let (mut base, mut tail): (ModPath, &[String]) = match head.as_str() {
        "crate" => (Vec::new(), tail),
        "self" => (source.to_vec(), tail),
        "super" => {
            let mut base = source.to_vec();
            base.pop()?;
            (base, tail)
        }
        _ => return None,
    };
    while tail.first().map(String::as_str) == Some("super") {
        base.pop()?;
        tail = &tail[1..];
    }
    base.extend(tail.iter().cloned());
    (base.first().map(String::as_str) == Some("resident")).then_some(base)
}
fn target_module(full: &[String], modules: &BTreeSet<ModPath>) -> Option<ModPath> {
    (1..=full.len())
        .rev()
        .map(|k| full[..k].to_vec())
        .find(|candidate| modules.contains(candidate))
}
fn violation(source: &[String], target: &[String]) -> Option<&'static str> {
    if layer_of(target) > layer_of(source) {
        return Some("higher layer");
    }
    None
}
fn line_at(text: &str, offset: usize) -> usize {
    text.as_bytes()[..offset].iter().filter(|b| **b == b'\n').count() + 1
}
fn analyze(
    text: &str,
    source: &[String],
    modules: &BTreeSet<ModPath>,
) -> Vec<(usize, ModPath, &'static str)> {
    let cleaned = drop_cfg_test_items(&strip_comments_and_literals(text));
    let (uses, rest) = split_uses(&cleaned);
    let mut paths: Vec<(usize, Vec<String>)> = Vec::new();
    for (offset, body) in uses {
        let line = line_at(&cleaned, offset);
        for path in expand_use(&body) {
            paths.push((line, path));
        }
    }
    let inline = Regex::new(r"\b(?:crate|super|self)(?:\s*::\s*[A-Za-z_][A-Za-z0-9_]*)+")
        .unwrap();
    for found in inline.find_iter(&rest) {
        let path = found.as_str().split("::").map(|s| s.trim().to_string()).collect();
        paths.push((line_at(&rest, found.start()), path));
    }
    let mut found = Vec::new();
    for (line, path) in paths {
        let Some(full) = resolve(&path, source) else {
            continue;
        };
        let Some(target) = target_module(&full, modules) else {
            continue;
        };
        if let Some(reason) = violation(source, &target) {
            found.push((line, target, reason));
        }
    }
    found
}
#[test]
fn dependencies_never_point_upward() {
    let (files, modules) = load_sources();
    assert!(! files.is_empty());
    let mut violations = Vec::new();
    for file in &files {
        for (line, target, reason) in analyze(&file.text, &file.module, &modules) {
            violations
                .push(
                    format!(
                        "{}:{line}: depends on {} ({reason})", file.rel, target
                        .join("::")
                    ),
                );
        }
    }
    violations.sort();
    violations.dedup();
    assert!(violations.is_empty(), "upward dependencies:\n{}", violations.join("\n"));
}
#[test]
fn layer_numbers_decrease_with_depth() {
    let (_, modules) = load_sources();
    let mut violations = Vec::new();
    for module in &modules {
        let numbers: Vec<u32> = module.iter().filter_map(|s| layer_number(s)).collect();
        if numbers.windows(2).any(|w| w[0] <= w[1]) {
            violations.push(module.join("::"));
        }
    }
    assert!(
        violations.is_empty(), "layer numbers must strictly decrease:\n{}", violations
        .join("\n")
    );
}
fn path_of(text: &str) -> ModPath {
    text.split("::").map(str::to_string).collect()
}
fn sample_modules() -> BTreeSet<ModPath> {
    [
        "resident",
        "resident::l130",
        "resident::l130::a",
        "resident::l130::a::cpu",
        "resident::l130::a::sibling",
        "resident::l130::l120",
        "resident::l130::l120::manager",
        "resident::l130::l120::l110",
        "resident::l130::l120::l110::a",
        "resident::l130::l120::l110::a::engine",
        "resident::l130::l120::l110::l100",
        "resident::l130::l120::l110::l100::planner",
    ]
        .iter()
        .map(|s| path_of(s))
        .collect()
}
fn targets(source: &str, text: &str) -> Vec<String> {
    analyze(text, &path_of(source), &sample_modules())
        .into_iter()
        .map(|(_, target, _)| target.join("::"))
        .collect()
}
#[test]
fn use_trees_are_expanded() {
    let paths = expand_use(" crate::a::{b, c::{d, self}, e as f, g::*,} ");
    let flat: Vec<String> = paths.iter().map(|p| p.join("::")).collect();
    assert_eq!(
        flat, vec!["crate::a::b", "crate::a::c::d", "crate::a::c", "crate::a::e",
        "crate::a::g"]
    );
}
#[test]
fn detector_accepts_downward_and_sibling_dependencies() {
    let cpu = "resident::l130::a::cpu";
    assert!(
        targets(cpu, "use crate::resident::l130::l120::manager::LazyIndex;").is_empty()
    );
    assert!(targets(cpu, "use super::sibling::Thing;").is_empty());
    assert!(targets(cpu, "use super::super::l120::manager::Manager;").is_empty());
    let engine = "resident::l130::l120::l110::a::engine";
    assert!(
        targets(engine,
        "use crate::resident::l130::l120::l110::{self, l100::planner::Plan};").is_empty()
    );
}
#[test]
fn detector_flags_upward_dependencies() {
    let manager = "resident::l130::l120::manager";
    assert_eq!(
        targets(manager, "use crate::resident::l130::a::cpu::Thing;"),
        vec!["resident::l130::a::cpu"]
    );
    assert_eq!(
        targets(manager,
        "use crate::resident::l130::{a::cpu::Thing, l120::manager as own};"),
        vec!["resident::l130::a::cpu"]
    );
    assert_eq!(
        targets(manager, "fn f() { let _ = crate::resident::l130::a::cpu::build(); }"),
        vec!["resident::l130::a::cpu"]
    );
    assert_eq!(
        targets("resident::l130::l120::l110::l100::planner",
        "use crate::resident::l130::l120::l110::a::engine::E;"),
        vec!["resident::l130::l120::l110::a::engine"]
    );
}
#[test]
fn detector_accepts_enclosing_modules_of_the_same_layer() {
    let cpu = "resident::l130::a::cpu";
    assert!(targets(cpu, "use super::*;").is_empty());
    assert!(targets(cpu, "use super::Shared;").is_empty());
    assert!(targets(cpu, "use crate::resident::l130::Shared;").is_empty());
}
#[test]
fn detector_ignores_comments_literals_and_test_items() {
    let text = "
// use crate::resident::l130::a::cpu::X;
/* use crate::resident::l130::a::cpu::X; */
const S: &str = \"crate::resident::l130::a::cpu::X\";
#[cfg(test)]
mod tests {
    use crate::resident::l130::a::cpu::X;
}
#[cfg(test)]
#[path = \"test/manager.rs\"]
mod tests;
#[cfg(test)]
use crate::resident::l130::a::cpu::{X, Y};
";
    assert!(targets("resident::l130::l120::manager", text).is_empty());
}
