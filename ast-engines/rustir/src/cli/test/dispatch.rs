use super::*;

fn invoke(args: &[&str]) -> (i32, String, String) {
    let mut out = Vec::new();
    let mut err = Vec::new();
    let code = run(args.iter().map(|a| a.to_string()), &mut out, &mut err);
    (
        code,
        String::from_utf8(out).unwrap(),
        String::from_utf8(err).unwrap(),
    )
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
