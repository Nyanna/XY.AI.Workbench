use super::*;

fn conflict(name: &str, files: &[&str]) -> NameConflict {
    NameConflict {
        name: name.into(),
        files: files.iter().map(PathBuf::from).collect(),
    }
}

#[test]
fn yaml_lists_every_name_with_its_files() {
    let yaml = conflicts_yaml(&[conflict("A", &["/a.rs", "/b.rs"]), conflict("B", &["/c.rs"])]);
    assert_eq!(
        yaml,
        "conflicts:\n  - name: \"A\"\n    files:\n      - \"/a.rs\"\n      - \"/b.rs\"\n  - name: \"B\"\n    files:\n      - \"/c.rs\"\n"
    );
}

#[test]
fn special_characters_are_escaped() {
    assert_eq!(quote("a\"b\\c"), "\"a\\\"b\\\\c\"");
}

#[test]
fn report_is_written_into_the_given_directory() {
    let dir = std::env::temp_dir().join(format!("rustir-report-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let target = write_conflicts(&dir, &[conflict("A", &["/a.rs"])]).unwrap();
    assert_eq!(target, dir.join(CONFLICTS_FILE));
    assert!(fs::read_to_string(target).unwrap().starts_with("conflicts:\n"));
}
