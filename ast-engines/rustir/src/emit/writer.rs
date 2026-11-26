use crate::ir::{Object, Project};

pub fn render(project: &Project) -> String {
    let modules = project
        .modules
        .iter()
        .map(|module| format!("mod {};\n", module.name));
    let objects = project.objects.iter().map(render_object);
    modules.chain(objects).collect()
}

fn render_object(object: &Object) -> String {
    let mut line = format!("{} {}", object.kind.label(), object.name);
    if !object.dependencies.is_empty() {
        line.push_str(" -> ");
        line.push_str(&object.dependencies.join(", "));
    }
    line.push('\n');
    line
}

#[cfg(test)]
#[path = "test/writer.rs"]
mod tests;
