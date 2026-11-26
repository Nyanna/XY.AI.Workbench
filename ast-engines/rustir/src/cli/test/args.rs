use super::*;

fn parse(args: &[&str]) -> Result<Command> {
    parse_args(args.iter().map(|a| a.to_string()))
}

#[test]
fn no_arguments_means_help() {
    assert_eq!(parse(&[]), Ok(Command::Help));
}

#[test]
fn help_aliases() {
    for flag in ["help", "--help", "-h"] {
        assert_eq!(parse(&[flag]), Ok(Command::Help));
    }
}

#[test]
fn parse_takes_path() {
    assert_eq!(
        parse(&["parse", "/p"]),
        Ok(Command::Parse {
            path: PathBuf::from("/p"),
            edges: None
        })
    );
}

#[test]
fn parse_takes_edges_file() {
    assert_eq!(
        parse(&["parse", "/p", "--edges", "/e.txt"]),
        Ok(Command::Parse {
            path: PathBuf::from("/p"),
            edges: Some(PathBuf::from("/e.txt"))
        })
    );
}

#[test]
fn parse_requires_path() {
    assert!(matches!(parse(&["parse"]), Err(Error::Usage(_))));
}

#[test]
fn edges_requires_a_file() {
    assert!(matches!(parse(&["parse", "/p", "--edges"]), Err(Error::Usage(_))));
}

#[test]
fn edges_may_not_repeat() {
    let result = parse(&["parse", "/p", "--edges", "a", "--edges", "b"]);
    assert!(matches!(result, Err(Error::Usage(_))));
}

#[test]
fn parse_rejects_extra_argument() {
    assert!(matches!(parse(&["parse", "/p", "x"]), Err(Error::Usage(_))));
}

#[test]
fn unknown_command_is_usage_error() {
    assert!(matches!(parse(&["bogus"]), Err(Error::Usage(_))));
}
