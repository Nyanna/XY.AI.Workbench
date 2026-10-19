//! Public signatures of the resident engine must not expose Candle tensors or raw pointers.
//! The Candle adapter and the CUDA backends are the only places allowed to touch them.
use std::fs;
use std::path::{Path, PathBuf};
const ALLOWED: &[&str] = &[
    "model/candle_adapter.rs",
    "kernel/cuda.rs",
    "kernel/maxsim.rs",
];
fn sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}
fn public_signatures(text: &str) -> Vec<String> {
    let mut sigs = Vec::new();
    let mut lines = text.lines();
    while let Some(line) = lines.next() {
        let t = line.trim_start();
        if !(t.starts_with("pub fn ") || t.starts_with("pub unsafe fn ")
            || t.starts_with("pub const fn "))
        {
            continue;
        }
        let mut sig = t.to_string();
        while !sig.contains('{') && !sig.trim_end().ends_with(';') {
            match lines.next() {
                Some(next) => {
                    sig.push(' ');
                    sig.push_str(next.trim());
                }
                None => break,
            }
        }
        sigs.push(sig);
    }
    sigs
}
fn has_word(sig: &str, word: &str) -> bool {
    let is_ident = |c: char| c.is_alphanumeric() || c == '_';
    sig.match_indices(word)
        .any(|(i, _)| {
            let before = sig[..i].chars().next_back();
            let after = sig[i + word.len()..].chars().next();
            !before.is_some_and(is_ident) && !after.is_some_and(is_ident)
        })
}
fn leaks(sig: &str) -> bool {
    has_word(sig, "Tensor") || sig.contains("*mut ") || sig.contains("*const ")
        || has_word(sig, "CudaSlice") || has_word(sig, "CUdeviceptr")
}
#[test]
fn public_api_is_free_of_tensors_and_pointers() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/resident");
    let mut files = Vec::new();
    sources(&root, &mut files);
    assert!(! files.is_empty());
    let mut violations = Vec::new();
    for file in files {
        let rel = file.strip_prefix(&root).unwrap().to_string_lossy().replace('\\', "/");
        if ALLOWED.contains(&rel.as_str()) {
            continue;
        }
        let text = fs::read_to_string(&file).unwrap();
        let body = text.split("#[cfg(test)]").next().unwrap_or(&text);
        for sig in public_signatures(body) {
            if leaks(&sig) {
                violations.push(format!("{rel}: {sig}"));
            }
        }
    }
    assert!(violations.is_empty(), "leaking signatures:\n{}", violations.join("\n"));
}
#[test]
fn detector_flags_leaks() {
    let sigs = public_signatures(
        "pub fn a(t: &Tensor) -> u8 {\n}\npub fn b(\n    p: *const u8,\n) {\n}\npub fn c() {}",
    );
    let flagged: Vec<_> = sigs.iter().map(|s| leaks(s)).collect();
    assert_eq!(flagged, vec![true, true, false]);
}
