use super::*;

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| s.to_string()).collect()
}

fn env_with(pairs: &'static [(&'static str, &'static str)]) -> impl Fn(&str) -> Option<String> {
    move |name| {
        pairs
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, value)| value.to_string())
    }
}

const FULL_ENV: &[(&str, &str)] = &[
    ("CARGO_PRIMARY_PACKAGE", "1"),
    ("RUSTIR_OUT", "/tmp/edges"),
    ("RUSTIR_PREFIX", "resident"),
];

#[test]
fn empty_arguments_have_no_plan() {
    assert_eq!(plan(&[], env_with(FULL_ENV)), None);
}

#[test]
fn primary_target_is_analyzed() {
    let raw = strings(&["rustc", "--crate-name", "rag", "src/lib.rs"]);
    let plan = plan(&raw, env_with(FULL_ENV)).unwrap();
    assert_eq!(plan.compiler_args, raw);
    assert_eq!(
        plan.analysis,
        Some(Analysis {
            prefix: "resident".into(),
            out: "/tmp/edges".into()
        })
    );
}

#[test]
fn prefix_defaults_to_empty() {
    let raw = strings(&["rustc", "--crate-name", "rag"]);
    let plan = plan(&raw, env_with(&[("CARGO_PRIMARY_PACKAGE", "1"), ("RUSTIR_OUT", "o")])).unwrap();
    assert_eq!(plan.analysis.unwrap().prefix, "");
}

#[test]
fn dependencies_are_not_analyzed() {
    let raw = strings(&["rustc", "--crate-name", "serde"]);
    let plan = plan(&raw, env_with(&[("RUSTIR_OUT", "o")])).unwrap();
    assert_eq!(plan.analysis, None);
}

#[test]
fn build_scripts_are_not_analyzed() {
    let raw = strings(&["rustc", "--crate-name", "build_script_build"]);
    assert_eq!(plan(&raw, env_with(FULL_ENV)).unwrap().analysis, None);
}

#[test]
fn queries_without_crate_name_are_not_analyzed() {
    let raw = strings(&["rustc", "-vV"]);
    assert_eq!(plan(&raw, env_with(FULL_ENV)).unwrap().analysis, None);
    let print = strings(&["rustc", "--crate-name", "rag", "--print=cfg"]);
    assert_eq!(plan(&print, env_with(FULL_ENV)).unwrap().analysis, None);
}

#[test]
fn missing_output_disables_analysis() {
    let raw = strings(&["rustc", "--crate-name", "rag"]);
    let plan = plan(&raw, env_with(&[("CARGO_PRIMARY_PACKAGE", "1")])).unwrap();
    assert_eq!(plan.analysis, None);
}

#[test]
fn sysroot_detection_covers_both_forms() {
    assert!(has_sysroot(&strings(&["rustc", "--sysroot", "/s"])));
    assert!(has_sysroot(&strings(&["rustc", "--sysroot=/s"])));
    assert!(!has_sysroot(&strings(&["rustc", "--crate-name", "a"])));
}
