use std::io::Write;

use crate::cli::args::{parse_args, Command};
use crate::emit::render;
use crate::error::{Error, Result};
use crate::parser::parse_project;
use crate::transform::apply_all;

const USAGE: &str = "Usage:\n  rustir help\n  rustir parse <project-dir>\n";

pub fn execute(command: Command, out: &mut dyn Write) -> Result<()> {
    match command {
        Command::Help => out.write_all(USAGE.as_bytes())?,
        Command::Parse { path } => {
            let mut project = parse_project(&path)?;
            apply_all(&mut project, &[])?;
            out.write_all(render(&project).as_bytes())?;
        }
    }
    Ok(())
}

pub fn run<I>(args: I, out: &mut dyn Write, err: &mut dyn Write) -> i32
where
    I: IntoIterator<Item = String>,
{
    match parse_args(args).and_then(|command| execute(command, out)) {
        Ok(()) => 0,
        Err(error) => {
            let _ = writeln!(err, "error: {error}");
            match error {
                Error::Usage(_) => 2,
                _ => 1,
            }
        }
    }
}

#[cfg(test)]
#[path = "test/dispatch.rs"]
mod tests;
