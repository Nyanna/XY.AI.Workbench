use std::collections::{BTreeMap, BTreeSet};

use crate::emit::edits::{apply_edits, Source};
use crate::emit::imports::use_block;
use crate::emit::layout::Layout;
use crate::emit::paths::path_edits;
use crate::emit::visibility::{is_public, visibility_edits, Exposure};
use crate::ir::{Object, Project};

pub struct ModuleSource {
    pub text: String,
    pub public: bool,
}

pub fn module_source(
    project: &Project,
    layout: &Layout,
    incoming: &BTreeMap<String, BTreeSet<String>>,
    object: &Object,
    entry: bool,
) -> ModuleSource {
    let header = use_block(project, layout, object);
    let Ok(file) = syn::parse_file(&object.body) else {
        return ModuleSource {
            text: format!("{header}{}\n", object.body),
            public: false,
        };
    };
    let exposure = if entry {
        Exposure::Public
    } else if incoming.contains_key(&object.name) {
        Exposure::Crate
    } else {
        Exposure::Keep
    };
    let source = Source::new(&object.body);
    let mut edits = visibility_edits(&file, &source, exposure);
    edits.extend(path_edits(&file, &source, layout, &project.qualifiers));
    ModuleSource {
        text: format!("{header}{}\n", apply_edits(&object.body, edits)),
        public: is_public(&file) || exposure == Exposure::Public,
    }
}

#[cfg(test)]
#[path = "test/module.rs"]
mod tests;
