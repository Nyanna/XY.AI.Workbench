use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub root: PathBuf,
    pub modules: Vec<Module>,
}

impl Project {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            modules: Vec::new(),
        }
    }
}

#[cfg(test)]
#[path = "test/model.rs"]
mod tests;
