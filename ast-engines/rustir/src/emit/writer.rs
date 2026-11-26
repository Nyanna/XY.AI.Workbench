use crate::ir::Project;

pub fn render(project: &Project) -> String {
    project
        .modules
        .iter()
        .map(|module| format!("mod {};\n", module.name))
        .collect()
}

#[cfg(test)]
#[path = "test/writer.rs"]
mod tests;
