use std::path::PathBuf;

use crate::ir::ObjectTable;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectKind {
    Struct,
    Enum,
    Union,
    Trait,
    TypeAlias,
    Function,
    Const,
    Static,
    Macro,
}

impl ObjectKind {
    pub fn label(self) -> &'static str {
        match self {
            ObjectKind::Struct => "struct",
            ObjectKind::Enum => "enum",
            ObjectKind::Union => "union",
            ObjectKind::Trait => "trait",
            ObjectKind::TypeAlias => "type",
            ObjectKind::Function => "fn",
            ObjectKind::Const => "const",
            ObjectKind::Static => "static",
            ObjectKind::Macro => "macro",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object {
    pub name: String,
    pub kind: ObjectKind,
    pub body: String,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Project {
    pub root: PathBuf,
    pub modules: Vec<Module>,
    pub objects: ObjectTable,
}

impl Project {
    pub fn new(root: PathBuf) -> Self {
        Self {
            root,
            modules: Vec::new(),
            objects: ObjectTable::new(),
        }
    }
}

#[cfg(test)]
#[path = "test/model.rs"]
mod tests;
