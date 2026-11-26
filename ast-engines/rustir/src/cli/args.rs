use std::path::PathBuf;

use crate::error::{Error, Result};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    Parse { path: PathBuf },
}

pub fn parse_args<I>(args: I) -> Result<Command>
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();
    let command = match args.next() {
        None => return Ok(Command::Help),
        Some(command) => command,
    };
    match command.as_str() {
        "help" | "--help" | "-h" => Ok(Command::Help),
        "parse" => {
            let path = args
                .next()
                .ok_or_else(|| Error::Usage("parse requires a project directory".into()))?;
            match args.next() {
                Some(extra) => Err(Error::Usage(format!("unexpected argument: {extra}"))),
                None => Ok(Command::Parse {
                    path: PathBuf::from(path),
                }),
            }
        }
        other => Err(Error::Usage(format!("unknown command: {other}"))),
    }
}

#[cfg(test)]
#[path = "test/args.rs"]
mod tests;
