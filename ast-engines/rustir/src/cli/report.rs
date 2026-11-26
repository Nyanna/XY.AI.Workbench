use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{NameConflict, Result, CONFLICTS_FILE};

fn quote(text: &str) -> String {
    let mut quoted = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => quoted.push_str("\\\""),
            '\\' => quoted.push_str("\\\\"),
            '\n' => quoted.push_str("\\n"),
            other => quoted.push(other),
        }
    }
    quoted.push('"');
    quoted
}

pub fn conflicts_yaml(conflicts: &[NameConflict]) -> String {
    let mut yaml = String::from("conflicts:\n");
    for conflict in conflicts {
        yaml.push_str(&format!("  - name: {}\n    files:\n", quote(&conflict.name)));
        for file in &conflict.files {
            yaml.push_str(&format!("      - {}\n", quote(&file.display().to_string())));
        }
    }
    yaml
}

pub fn write_conflicts(dir: &Path, conflicts: &[NameConflict]) -> Result<PathBuf> {
    let target = dir.join(CONFLICTS_FILE);
    fs::write(&target, conflicts_yaml(conflicts))?;
    Ok(target)
}

#[cfg(test)]
#[path = "test/report.rs"]
mod tests;
