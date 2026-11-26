use super::*;
#[test]
fn new_project_is_empty() {
    let project = Project::new(PathBuf::from("/p"));
    assert_eq!(project.root, PathBuf::from("/p"));
    assert!(project.modules.is_empty());
    assert!(project.imports.is_empty());
    assert!(project.qualifiers.is_empty());
    assert_eq!(project.manifest, Manifest::default());
}
