use std::fs;
use std::path::{Path, PathBuf};
use crate::error::Result;
pub fn source_files(root: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    walk_dir(root, &mut files)?;
    files.sort();
    Ok(files)
}
fn walk_dir(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            if !is_skipped_dir(&path) {
                walk_dir(&path, out)?;
            }
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            out.push(path);
        }
    }
    Ok(())
}
fn is_skipped_dir(path: &Path) -> bool {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
    name.starts_with('.') || matches!(name, "test" | "tests" | "target")
        || path.join("Cargo.toml").is_file()
}
#[cfg(test)]
#[path = "test/files.rs"]
mod tests;
