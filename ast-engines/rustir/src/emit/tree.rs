use std::fs;
use std::path::Path;
use crate::emit::layout::Layout;
use crate::emit::module::module_source;
use crate::emit::root::{
    cargo_source, lib_source, main_source, package_name, Declared, GENERATED_MARK,
};
use crate::error::{Error, Result};
use crate::ir::{dependents, ObjectKind, Project};
pub fn write_crate(project: &Project, target: &Path) -> Result<()> {
    prepare_target(target)?;
    let package = package_name(project);
    let entry = write_modules(project, &target.join("src"))?;
    if let Some(module) = &entry {
        fs::write(target.join("src/main.rs"), main_source(&package, module))?;
    }
    let manifest = cargo_source(project, &package, entry.is_some());
    fs::write(target.join("Cargo.toml"), manifest)?;
    Ok(())
}
pub fn write_sources(project: &Project, target: &Path) -> Result<()> {
    let manifest = target.join("Cargo.toml");
    if !manifest.is_file() {
        return Err(Error::NotFound(manifest));
    }
    let src = target.join("src");
    if src.is_dir() {
        fs::remove_dir_all(&src)?;
    }
    write_modules(project, &src)?;
    Ok(())
}
fn write_modules(project: &Project, src: &Path) -> Result<Option<String>> {
    let layout = Layout::new(&project.objects);
    let incoming = dependents(&project.objects);
    fs::create_dir_all(src)?;
    let mut declared = Vec::new();
    let mut entry = None;
    for object in project.objects.iter() {
        let Some(module) = layout.module(&object.name) else {
            continue;
        };
        let is_entry = object.name == "main" && object.kind == ObjectKind::Function;
        let rendered = module_source(project, &layout, &incoming, object, is_entry);
        fs::write(src.join(format!("{module}.rs")), rendered.text)?;
        if is_entry {
            entry = Some(module.to_string());
        }
        declared
            .push(Declared {
                name: object.name.clone(),
                module: module.to_string(),
                kind: object.kind,
                public: rendered.public,
            });
    }
    fs::write(src.join("lib.rs"), lib_source(&declared, &layout))?;
    Ok(entry)
}
fn prepare_target(target: &Path) -> Result<()> {
    if target.exists() {
        let occupied = !target.is_dir() || fs::read_dir(target)?.next().is_some();
        if occupied && !is_generated(target) {
            return Err(Error::TargetNotEmpty(target.to_path_buf()));
        }
        let src = target.join("src");
        if src.is_dir() {
            fs::remove_dir_all(&src)?;
        }
    }
    fs::create_dir_all(target)?;
    Ok(())
}
fn is_generated(target: &Path) -> bool {
    fs::read_to_string(target.join("Cargo.toml"))
        .is_ok_and(|text| text.lines().next() == Some(GENERATED_MARK))
}
#[cfg(test)]
#[path = "test/tree.rs"]
mod tests;
