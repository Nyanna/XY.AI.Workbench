use super::*;
use crate::error::Error;
use crate::ir::Module;
use std::path::PathBuf;

struct AddModule(&'static str);

impl Transform for AddModule {
    fn name(&self) -> &str {
        "add-module"
    }

    fn apply(&self, project: &mut Project) -> Result<()> {
        project.modules.push(Module {
            name: self.0.to_string(),
        });
        Ok(())
    }
}

struct Failing;

impl Transform for Failing {
    fn name(&self) -> &str {
        "failing"
    }

    fn apply(&self, _project: &mut Project) -> Result<()> {
        Err(Error::Io("fail".into()))
    }
}

fn project() -> Project {
    Project::new(PathBuf::from("/p"))
}

#[test]
fn applies_in_order() {
    let mut project = project();
    let transforms: Vec<Box<dyn Transform>> =
        vec![Box::new(AddModule("a")), Box::new(AddModule("b"))];
    apply_all(&mut project, &transforms).unwrap();
    let names: Vec<&str> = project.modules.iter().map(|m| m.name.as_str()).collect();
    assert_eq!(names, ["a", "b"]);
}

#[test]
fn stops_at_first_error() {
    let mut project = project();
    let transforms: Vec<Box<dyn Transform>> = vec![Box::new(Failing), Box::new(AddModule("a"))];
    assert_eq!(
        apply_all(&mut project, &transforms),
        Err(Error::Io("fail".into()))
    );
    assert!(project.modules.is_empty());
}

#[test]
fn exposes_name() {
    assert_eq!(AddModule("a").name(), "add-module");
}
