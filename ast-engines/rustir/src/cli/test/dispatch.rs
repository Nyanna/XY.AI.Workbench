use super::*;
fn invoke(args: &[&str]) -> (i32, String, String) {
    let mut out = Vec::new();
    let mut err = Vec::new();
    let code = run(args.iter().map(|a| a.to_string()), &mut out, &mut err);
    (code, String::from_utf8(out).unwrap(), String::from_utf8(err).unwrap())
}
#[test]
fn help_prints_usage() {
    let (code, out, err) = invoke(&["help"]);
    assert_eq!(code, 0);
    assert_eq!(out, USAGE);
    assert!(err.is_empty());
}
#[test]
fn unknown_command_exits_with_two() {
    let (code, out, err) = invoke(&["bogus"]);
    assert_eq!(code, 2);
    assert!(out.is_empty());
    assert_eq!(err, "error: usage error: unknown command: bogus\n");
}
#[test]
fn missing_project_exits_with_one() {
    let (code, out, err) = invoke(&["parse", "/nonexistent/rustir/project"]);
    assert_eq!(code, 1);
    assert!(out.is_empty());
    assert!(err.starts_with("error: not found: "));
}
#[test]
fn name_conflicts_end_the_run_with_all_conflicts_listed() {
    let dir = std::env::temp_dir()
        .join(format!("rustir-cli-conflicts-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("a.rs"), "pub struct One; pub struct Two;").unwrap();
    std::fs::write(dir.join("b.rs"), "pub struct One; pub struct Two;").unwrap();
    let (code, out, err) = invoke(&["parse", dir.to_str().unwrap()]);
    assert_eq!(code, 1);
    assert!(out.is_empty());
    assert!(err.contains("2 name conflict(s)"));
    assert!(err.contains("One") && err.contains("Two"));
}
#[test]
fn parse_prints_objects_with_dependencies() {
    let dir = std::env::temp_dir()
        .join(format!("rustir-cli-objects-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("a.rs"), "pub struct One { t: Two } pub struct Two;")
        .unwrap();
    let (code, out, _) = invoke(&["parse", dir.to_str().unwrap()]);
    assert_eq!(code, 0);
    assert!(out.contains("struct One") && out.contains("-> Two"));
}
#[test]
fn edges_file_replaces_provisional_dependencies() {
    let dir = std::env::temp_dir()
        .join(format!("rustir-cli-edges-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("a.rs"), "pub struct One { t: Two } pub struct Two;")
        .unwrap();
    let edges = dir.join("edges.txt");
    std::fs::write(&edges, "One\tstd::fmt::Display\nTwo\t\n").unwrap();
    let (code, out, _) = invoke(
        &["parse", dir.to_str().unwrap(), "--edges", edges.to_str().unwrap()],
    );
    assert_eq!(code, 0);
    assert!(out.contains("struct One -> std::fmt::Display\n"));
    assert!(out.contains("struct Two\n"));
}
#[test]
fn missing_edges_file_exits_with_one() {
    let dir = std::env::temp_dir()
        .join(format!("rustir-cli-noedges-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let missing = dir.join("absent.txt");
    let (code, _, err) = invoke(
        &["parse", dir.to_str().unwrap(), "--edges", missing.to_str().unwrap()],
    );
    assert_eq!(code, 1);
    assert!(err.contains("not found"));
}
