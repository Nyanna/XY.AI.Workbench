use std::path::PathBuf;
use crate::error::{Error, Result};
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    Parse { path: PathBuf, edges: Option<PathBuf> },
    Render {
        path: PathBuf,
        output: PathBuf,
        edges: Option<PathBuf>,
        sources_only: bool,
    },
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
            let path = project_path(&mut args, "parse")?;
            let mut edges = None;
            while let Some(argument) = args.next() {
                match argument.as_str() {
                    "--edges" => take_option(&mut args, &mut edges, "--edges", "a file")?,
                    _ => return Err(unexpected(&argument)),
                }
            }
            Ok(Command::Parse { path, edges })
        }
        "render" => {
            let path = project_path(&mut args, "render")?;
            let mut edges = None;
            let mut output = None;
            let mut sources_only = false;
            while let Some(argument) = args.next() {
                match argument.as_str() {
                    "--sources-only" => sources_only = true,
                    "--edges" => take_option(&mut args, &mut edges, "--edges", "a file")?,
                    "--out" => {
                        take_option(&mut args, &mut output, "--out", "a directory")?
                    }
                    _ => return Err(unexpected(&argument)),
                }
            }
            let output = output
                .ok_or_else(|| Error::Usage("render requires --out <dir>".into()))?;
            Ok(Command::Render {
                path,
                output,
                edges,
                sources_only,
            })
        }
        other => Err(Error::Usage(format!("unknown command: {other}"))),
    }
}
fn project_path<I>(args: &mut I, command: &str) -> Result<PathBuf>
where
    I: Iterator<Item = String>,
{
    args.next()
        .map(PathBuf::from)
        .ok_or_else(|| Error::Usage(format!("{command} requires a project directory")))
}
fn take_option<I>(
    args: &mut I,
    slot: &mut Option<PathBuf>,
    flag: &str,
    what: &str,
) -> Result<()>
where
    I: Iterator<Item = String>,
{
    let value = args
        .next()
        .ok_or_else(|| Error::Usage(format!("{flag} requires {what}")))?;
    if slot.replace(PathBuf::from(value)).is_some() {
        return Err(Error::Usage(format!("{flag} given more than once")));
    }
    Ok(())
}
fn unexpected(argument: &str) -> Error {
    Error::Usage(format!("unexpected argument: {argument}"))
}
#[cfg(test)]
#[path = "test/args.rs"]
mod tests;
