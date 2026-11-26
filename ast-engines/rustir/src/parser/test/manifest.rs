use super::*;

const SAMPLE: &str = "[package]\nname = \"demo-app\"\nversion = \"0.1.0\"\nedition = \"2018\"\n\n[dependencies]\nserde = \"1\"\nsyn = { version = \"2\", features = [\"full\"] }\n\n[dev-dependencies]\ntempfile = \"3\"\n";

#[test]
fn package_name_and_edition_are_read() {
    let manifest = scan_manifest(SAMPLE);
    assert_eq!(manifest.package.as_deref(), Some("demo-app"));
    assert_eq!(manifest.edition.as_deref(), Some("2018"));
}

#[test]
fn dependency_tables_are_copied_verbatim() {
    let manifest = scan_manifest(SAMPLE);
    assert_eq!(
        manifest.dependencies,
        "[dependencies]\nserde = \"1\"\nsyn = { version = \"2\", features = [\"full\"] }\n\n"
    );
}

#[test]
fn dotted_dependency_tables_are_kept() {
    let manifest = scan_manifest("[dependencies.serde]\nversion = \"1\"\n[features]\nx = []\n");
    assert_eq!(manifest.dependencies, "[dependencies.serde]\nversion = \"1\"\n");
}

#[test]
fn keys_with_a_common_prefix_are_not_confused() {
    let manifest = scan_manifest("[package]\nnames = \"x\"\nedition.workspace = true\n");
    assert_eq!(manifest.package, None);
    assert_eq!(manifest.edition, None);
}

#[test]
fn missing_manifest_yields_defaults() {
    let dir = std::env::temp_dir().join(format!("rustir-manifest-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    assert_eq!(read_manifest(&dir).unwrap(), Manifest::default());
}
