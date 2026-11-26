use std::path::Path;

use crate::error::{Error, Result};
use crate::ir::Project;

pub fn parse_project(root: &Path) -> Result<Project> {
    let manifest = root.join("Cargo.toml");
    if !manifest.is_file() {
        return Err(Error::NotFound(manifest));
    }
    Ok(Project::new(root.to_path_buf()))
}

#[cfg(test)]
#[path = "test/project.rs"]
mod tests;
