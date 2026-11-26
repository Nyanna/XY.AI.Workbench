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
            path: PathBuf::from("/p")
        })
    );
}

#[test]
fn parse_requires_path() {
    assert!(matches!(parse(&["parse"]), Err(Error::Usage(_))));
}

#[test]
fn parse_rejects_extra_argument() {
    assert!(matches!(parse(&["parse", "/p", "x"]), Err(Error::Usage(_))));
}

#[test]
fn unknown_command_is_usage_error() {
    assert!(matches!(parse(&["bogus"]), Err(Error::Usage(_))));
}
