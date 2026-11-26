use std::io::Write;
use std::path::Path;
use crate::cli::args::{parse_args, Command};
use crate::cli::report::write_conflicts;
use crate::emit::render;
use crate::emit::write_crate;
use crate::emit::write_sources;
use crate::error::{Error, Result};
use crate::ir::Project;
use crate::parser::load_edges;
use crate::parser::parse_project;
use crate::transform::apply_all;
const USAGE: &str = "Usage:\n  rustir help\n  rustir parse <project-dir> [--edges <file>]\n  rustir render <project-dir> --out <dir> [--edges <file>] [--sources-only]\n";
fn load_project(path: &Path, edges: Option<&Path>) -> Result<Project> {
    let mut project = match parse_project(path) {
        Err(Error::NameConflicts(conflicts)) => {
            write_conflicts(&std::env::current_dir()?, &conflicts)?;
            return Err(Error::NameConflicts(conflicts));
        }
        other => other?,
    };
    if let Some(file) = edges {
        load_edges(&mut project, file)?;
    }
    apply_all(&mut project, &[])?;
    Ok(project)
}
pub fn execute(command: Command, out: &mut dyn Write) -> Result<()> {
    match command {
        Command::Help => out.write_all(USAGE.as_bytes())?,
        Command::Parse { path, edges } => {
            let project = load_project(&path, edges.as_deref())?;
            out.write_all(render(&project).as_bytes())?;
        }
        Command::Render { path, output, edges, sources_only } => {
            let project = load_project(&path, edges.as_deref())?;
            if sources_only {
                write_sources(&project, &output)?;
            } else {
                write_crate(&project, &output)?;
            }
            writeln!(
                out, "wrote {} modules to {}", project.objects.len(), output.display()
            )?;
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
