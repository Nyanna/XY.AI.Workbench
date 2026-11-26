use super::*;
use crate::ir::Module;
use std::path::PathBuf;

#[test]
fn empty_project_renders_nothing() {
    assert_eq!(render(&Project::new(PathBuf::from("/p"))), "");
}

#[test]
fn renders_one_line_per_module() {
    let mut project = Project::new(PathBuf::from("/p"));
    for name in ["a", "b"] {
        project.modules.push(Module { name: name.into() });
    }
    assert_eq!(render(&project), "mod a;\nmod b;\n");
}
