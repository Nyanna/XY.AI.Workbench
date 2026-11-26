use std::fs;
use std::path::Path;

use crate::error::Result;
use crate::ir::Manifest;

pub fn read_manifest(root: &Path) -> Result<Manifest> {
    let file = root.join("Cargo.toml");
    if !file.is_file() {
        return Ok(Manifest::default());
    }
    Ok(scan_manifest(&fs::read_to_string(file)?))
}

pub fn scan_manifest(text: &str) -> Manifest {
    let mut manifest = Manifest::default();
    let mut section = String::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(header) = trimmed.strip_prefix('[') {
            let name = header.trim_start_matches('[').split(']').next();
            section = name.unwrap_or_default().trim().to_string();
        }
        if section == "dependencies" || section.starts_with("dependencies.") {
            manifest.dependencies.push_str(line);
            manifest.dependencies.push('\n');
        } else if section == "package" {
            if let Some(name) = quoted_value(trimmed, "name") {
                manifest.package = Some(name);
            }
            if let Some(edition) = quoted_value(trimmed, "edition") {
                manifest.edition = Some(edition);
            }
        }
    }
    manifest
}

fn quoted_value(line: &str, key: &str) -> Option<String> {
    let rest = line.strip_prefix(key)?.trim_start().strip_prefix('=')?.trim();
    let rest = rest.strip_prefix('"')?;
    rest.split('"').next().map(str::to_string)
}

#[cfg(test)]
#[path = "test/manifest.rs"]
mod tests;
