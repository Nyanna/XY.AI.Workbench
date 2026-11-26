use crate::error::Result;
use crate::ir::Project;

pub trait Transform {
    fn name(&self) -> &str;
    fn apply(&self, project: &mut Project) -> Result<()>;
}

pub fn apply_all(project: &mut Project, transforms: &[Box<dyn Transform>]) -> Result<()> {
    for transform in transforms {
        transform.apply(project)?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/pipeline.rs"]
mod tests;
