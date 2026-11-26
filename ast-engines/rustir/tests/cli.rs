use std::process::Command;

fn rustir(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rustir"))
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn help_succeeds() {
    let output = rustir(&["help"]);
    assert!(output.status.success());
    assert!(String::from_utf8(output.stdout).unwrap().starts_with("Usage:"));
}

#[test]
fn parse_own_project_succeeds() {
    let output = rustir(&["parse", env!("CARGO_MANIFEST_DIR")]);
    assert!(output.status.success());
}

#[test]
fn unknown_command_fails_with_usage_code() {
    let output = rustir(&["bogus"]);
    assert_eq!(output.status.code(), Some(2));
}
