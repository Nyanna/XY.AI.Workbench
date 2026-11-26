use super::*;
fn parse(args: &[&str]) -> Result<Command> {
    parse_args(args.iter().map(|a| a.to_string()))
}
#[test]
fn no_arguments_means_help() {
    assert_eq!(parse(& []), Ok(Command::Help));
}
#[test]
fn help_aliases() {
    for flag in ["help", "--help", "-h"] {
        assert_eq!(parse(& [flag]), Ok(Command::Help));
    }
}
#[test]
fn parse_takes_path() {
    assert_eq!(
        parse(& ["parse", "/p"]), Ok(Command::Parse { path : PathBuf::from("/p"), edges :
        None })
    );
}
#[test]
fn parse_takes_edges_file() {
    assert_eq!(
        parse(& ["parse", "/p", "--edges", "/e.txt"]), Ok(Command::Parse { path :
        PathBuf::from("/p"), edges : Some(PathBuf::from("/e.txt")) })
    );
}
#[test]
fn parse_requires_path() {
    assert!(matches!(parse(& ["parse"]), Err(Error::Usage(_))));
}
#[test]
fn edges_requires_a_file() {
    assert!(matches!(parse(& ["parse", "/p", "--edges"]), Err(Error::Usage(_))));
}
#[test]
fn edges_may_not_repeat() {
    let result = parse(&["parse", "/p", "--edges", "a", "--edges", "b"]);
    assert!(matches!(result, Err(Error::Usage(_))));
}
#[test]
fn parse_rejects_extra_argument() {
    assert!(matches!(parse(& ["parse", "/p", "x"]), Err(Error::Usage(_))));
}
#[test]
fn unknown_command_is_usage_error() {
    assert!(matches!(parse(& ["bogus"]), Err(Error::Usage(_))));
}
#[test]
fn render_takes_path_and_output() {
    assert_eq!(
        parse(& ["render", "/p", "--out", "/o"]), Ok(Command::Render { path :
        PathBuf::from("/p"), output : PathBuf::from("/o"), edges : None, sources_only :
        false })
    );
}
#[test]
fn render_accepts_options_in_any_order() {
    assert_eq!(
        parse(& ["render", "/p", "--sources-only", "--edges", "/e.txt", "--out", "/o"]),
        Ok(Command::Render { path : PathBuf::from("/p"), output : PathBuf::from("/o"),
        edges : Some(PathBuf::from("/e.txt")), sources_only : true })
    );
}
#[test]
fn render_requires_path_and_output() {
    assert!(matches!(parse(& ["render"]), Err(Error::Usage(_))));
    assert!(matches!(parse(& ["render", "/p"]), Err(Error::Usage(_))));
}
#[test]
fn output_requires_a_directory() {
    assert!(matches!(parse(& ["render", "/p", "--out"]), Err(Error::Usage(_))));
}
#[test]
fn output_may_not_repeat() {
    let result = parse(&["render", "/p", "--out", "a", "--out", "b"]);
    assert!(matches!(result, Err(Error::Usage(_))));
}
#[test]
fn render_rejects_unknown_option() {
    assert!(matches!(parse(& ["render", "/p", "--bogus"]), Err(Error::Usage(_))));
}
#[test]
fn parse_does_not_accept_the_output_option() {
    assert!(matches!(parse(& ["parse", "/p", "--out", "/o"]), Err(Error::Usage(_))));
}
#[test]
fn parse_does_not_accept_sources_only() {
    assert!(matches!(parse(& ["parse", "/p", "--sources-only"]), Err(Error::Usage(_))));
}
