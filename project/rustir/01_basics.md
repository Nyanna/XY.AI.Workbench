Erstelle eine grundlegendes Rust Projekt.

Die Applikation soll ein CLI-Interface haben, ein Rust Projekt parsen und in eine AST/IR Rpräsentation einlesen (später), Modifikationen darauf Anwenden und anschließend wieder als Code ausgeben (später). 

[include](/home/user/xyan/xy.ai.workbench/ast-engines/rustir/index.md)
SystemInit:  id=1f51da0e-4848-4d86-a0d1-712eb119c429, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
ReasoningToken: 40
Control Request:
```yaml
id: 6539-63-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines
    recursive: true
  reason: Check whether the target directory already exists and what it contains before creating files.
```
/answer 6539-63-0 allow
Control Request:
```yaml
id: 6539-64-63
result:
  structuredContent:
    results:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines
      entries:
      - ".: (1 file below)"
      - "./java: (4 files below)"
      - "./java/.bin/xy/ai: (64 files below)"
      - "./java/.settings: (2 files below)"
      - "./java/libs: (6 files below)"
      - "./java/scripts: (1 file below)"
      - "./java/src/xy/ai: (57 files below)"
      - "./markdown: (7 files below)"
      - "./markdown/src/xy/ai: (62 files below)"
      - "./rust: (6 files below)"
      - "./rust/.bin: (2 files below)"
      - "./rust/.bin/debug: (5 files below)"
      - "./rust/.bin/debug/.fingerprint: (214 files below)"
      - "./rust/.bin/debug/deps: (108 files below)"
      - "./rust/.bin/debug/incremental: (542 files below)"
      - "./rust/.bin/doc: (5 files below)"
      - "./rust/.bin/doc/search.index: (169 files below)"
      - "./rust/.bin/doc/src: (52 files below)"
      - "./rust/.bin/doc/static.files: (34 files below)"
      - "./rust/.bin/doc/syn: (1317 files below)"
      - "./rust/.bin/doc/trait.impl: (40 files below)"
      - "./rust/.bin/doc/type.impl: (2 files below)"
      - "./rust/.bin/release: (5 files below)"
      - "./rust/.bin/release/.fingerprint: (154 files below)"
      - "./rust/.bin/release/deps: (80 files below)"
      - "./rust/.cargo: (1 file below)"
      - "./rust/src: (1 file below)"
      - "./rust/src/engine: (8 files below)"
      - "./rust/src/generated: (1 file below)"
      - "./rust/src/generated/engine: (112 files below)"
      - "./rustir: (3 files below)"
      warnings:
      - Result reduced to directories up to depth 4 because the number of directories with matches (371) exceeds the limit of 50. Narrow down the result using the 'pattern' regular expression parameter.
```
/answer 6539-64-63 deny existiert schon
Control Request:
```yaml
id: 6539-65-0
toolName: list
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir
  reason: "Inspect the existing rustir directory to see what is already there, so I build on it instead of overwriting."
```
/answer 6539-65-0 allow
Control Request:
```yaml
id: 6539-66-65
result:
  structuredContent:
    errors:
    - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir
      error: Directory not found or not a directory.
```
/answer 6539-66-65 allow
ReasoningToken: 61
Control Request:
```yaml
id: 6539-67-0
toolName: run_command
arguments:
  items:
  - id: read.Cargo.toml
    cwd: /home/user/xyan/xy.ai.workbench/ast-engines/rustir
  reason: Read the existing Cargo.toml to align the project with its current configuration.
```
/answer 6539-67-0 allow
Control Request:
```yaml
id: 6539-68-67
result:
  structuredContent:
    errors:
    - index: 0
      error: Working directory not found or not a directory.
```
/answer 6539-68-67 allow
ReasoningToken: 4265
Control Request:
```yaml
id: 6539-69-0
toolName: write
arguments:
  items:
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/Cargo.toml
    mode: replace
    content: |
      [package]
      name = "rustir"
      version = "0.1.0"
      edition = "2021"
      description = "Parse Rust projects into an IR, transform it and emit code again"

      [lib]
      name = "rustir"
      path = "src/lib.rs"

      [[bin]]
      name = "rustir"
      path = "src/main.rs"

      [dependencies]
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/.gitignore
    mode: replace
    content: |
      /target
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/README.md
    mode: replace
    content: |
      # rustir

      Command line tool that will parse a Rust project into an AST/IR, apply modifications and emit code again.

      Current state: project skeleton only. Parsing, transformation and emission are placeholders.

      ## Usage

      ```
      rustir help
      rustir parse <project-dir>
      ```

      ## Layout

      | Module      | Layer | Responsibility                          |
      |-------------|-------|-----------------------------------------|
      | `error`     | 0     | Shared error type                       |
      | `ir`        | 1     | IR data model                           |
      | `parser`    | 2     | Project to IR                           |
      | `transform` | 2     | IR to IR modifications                  |
      | `emit`      | 2     | IR to source code                       |
      | `cli`       | 3     | Argument handling and command dispatch  |

      ## Conventions

      - A module may only depend on modules in strictly lower layers.
      - Non-leaf modules (`lib.rs`, `mod.rs`) contain only module declarations and re-exports.
      - Child modules live in the directory of their parent.
      - Unit tests of `<dir>/<name>.rs` live in `<dir>/test/<name>.rs`.
      - Tests verify contracts through return values, never through log output.
      - No `use ... as` alias chains.

      These rules are enforced by `tests/hygiene.rs`.

      ## Development

      ```
      cargo test
      ```
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/main.rs
    mode: replace
    content: |
      fn main() {
          let code = rustir::run(
              std::env::args().skip(1),
              &mut std::io::stdout(),
              &mut std::io::stderr(),
          );
          std::process::exit(code);
      }
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/lib.rs
    mode: replace
    content: |
      pub mod cli;
      pub mod emit;
      pub mod error;
      pub mod ir;
      pub mod parser;
      pub mod transform;

      pub use cli::run;
      pub use error::{Error, Result};
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/error.rs
    mode: replace
    content: |
      use std::fmt;
      use std::path::PathBuf;

      #[derive(Debug, PartialEq, Eq)]
      pub enum Error {
          Usage(String),
          NotFound(PathBuf),
          Io(String),
      }

      pub type Result<T> = std::result::Result<T, Error>;

      impl fmt::Display for Error {
          fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
              match self {
                  Error::Usage(message) => write!(f, "usage error: {message}"),
                  Error::NotFound(path) => write!(f, "not found: {}", path.display()),
                  Error::Io(message) => write!(f, "io error: {message}"),
              }
          }
      }

      impl std::error::Error for Error {}

      impl From<std::io::Error> for Error {
          fn from(error: std::io::Error) -> Self {
              Error::Io(error.to_string())
          }
      }

      #[cfg(test)]
      #[path = "test/error.rs"]
      mod tests;
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/test/error.rs
    mode: replace
    content: |
      use super::*;

      #[test]
      fn usage_display() {
          assert_eq!(Error::Usage("x".into()).to_string(), "usage error: x");
      }

      #[test]
      fn not_found_display() {
          let error = Error::NotFound(PathBuf::from("/a/b"));
          assert_eq!(error.to_string(), "not found: /a/b");
      }

      #[test]
      fn io_error_converts() {
          let source = std::io::Error::new(std::io::ErrorKind::Other, "boom");
          assert_eq!(Error::from(source), Error::Io("boom".into()));
      }
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/ir/mod.rs
    mode: replace
    content: |
      mod model;

      pub use model::{Module, Project};
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/ir/model.rs
    mode: replace
    content: |
      use std::path::PathBuf;

      #[derive(Debug, Clone, PartialEq, Eq)]
      pub struct Module {
          pub name: String,
      }

      #[derive(Debug, Clone, PartialEq, Eq)]
      pub struct Project {
          pub root: PathBuf,
          pub modules: Vec<Module>,
      }

      impl Project {
          pub fn new(root: PathBuf) -> Self {
              Self {
                  root,
                  modules: Vec::new(),
              }
          }
      }

      #[cfg(test)]
      #[path = "test/model.rs"]
      mod tests;
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/ir/test/model.rs
    mode: replace
    content: |
      use super::*;

      #[test]
      fn new_project_is_empty() {
          let project = Project::new(PathBuf::from("/p"));
          assert_eq!(project.root, PathBuf::from("/p"));
          assert!(project.modules.is_empty());
      }
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/parser/mod.rs
    mode: replace
    content: |
      mod project;

      pub use project::parse_project;
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/parser/project.rs
    mode: replace
    content: |
      use std::path::Path;

      use crate::error::{Error, Result};
      use crate::ir::Project;

      pub fn parse_project(root: &Path) -> Result<Project> {
          let manifest = root.join("Cargo.toml");
          if !manifest.is_file() {
              return Err(Error::NotFound(manifest));
          }
          Ok(Project::new(root.to_path_buf()))
      }

      #[cfg(test)]
      #[path = "test/project.rs"]
      mod tests;
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/parser/test/project.rs
    mode: replace
    content: |
      use super::*;
      use std::fs;
      use std::path::PathBuf;

      fn scratch(name: &str) -> PathBuf {
          let dir = std::env::temp_dir().join(format!("rustir-parser-{name}-{}", std::process::id()));
          let _ = fs::remove_dir_all(&dir);
          fs::create_dir_all(&dir).unwrap();
          dir
      }

      #[test]
      fn missing_manifest_is_not_found() {
          let dir = scratch("missing");
          let result = parse_project(&dir);
          assert_eq!(result, Err(Error::NotFound(dir.join("Cargo.toml"))));
      }

      #[test]
      fn existing_manifest_yields_project() {
          let dir = scratch("present");
          fs::write(dir.join("Cargo.toml"), "").unwrap();
          let project = parse_project(&dir).unwrap();
          assert_eq!(project.root, dir);
          assert!(project.modules.is_empty());
      }
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/transform/mod.rs
    mode: replace
    content: |
      mod pipeline;

      pub use pipeline::{apply_all, Transform};
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/transform/pipeline.rs
    mode: replace
    content: |
      use crate::error::Result;
      use crate::ir::Project;

      pub trait Transform {
          fn name(&self) -> &str;
          fn apply(&self, project: &mut Project) -> Result<()>;
      }

      pub fn apply_all(project: &mut Project, transforms: &[Box<dyn Transform>]) -> Result<()> {
          for transform in transforms {
              transform.apply(project)?;
          }
          Ok(())
      }

      #[cfg(test)]
      #[path = "test/pipeline.rs"]
      mod tests;
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/transform/test/pipeline.rs
    mode: replace
    content: |
      use super::*;
      use crate::error::Error;
      use crate::ir::Module;
      use std::path::PathBuf;

      struct AddModule(&'static str);

      impl Transform for AddModule {
          fn name(&self) -> &str {
              "add-module"
          }

          fn apply(&self, project: &mut Project) -> Result<()> {
              project.modules.push(Module {
                  name: self.0.to_string(),
              });
              Ok(())
          }
      }

      struct Failing;

      impl Transform for Failing {
          fn name(&self) -> &str {
              "failing"
          }

          fn apply(&self, _project: &mut Project) -> Result<()> {
              Err(Error::Io("fail".into()))
          }
      }

      fn project() -> Project {
          Project::new(PathBuf::from("/p"))
      }

      #[test]
      fn applies_in_order() {
          let mut project = project();
          let transforms: Vec<Box<dyn Transform>> =
              vec![Box::new(AddModule("a")), Box::new(AddModule("b"))];
          apply_all(&mut project, &transforms).unwrap();
          let names: Vec<&str> = project.modules.iter().map(|m| m.name.as_str()).collect();
          assert_eq!(names, ["a", "b"]);
      }

      #[test]
      fn stops_at_first_error() {
          let mut project = project();
          let transforms: Vec<Box<dyn Transform>> = vec![Box::new(Failing), Box::new(AddModule("a"))];
          assert_eq!(
              apply_all(&mut project, &transforms),
              Err(Error::Io("fail".into()))
          );
          assert!(project.modules.is_empty());
      }

      #[test]
      fn exposes_name() {
          assert_eq!(AddModule("a").name(), "add-module");
      }
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/emit/mod.rs
    mode: replace
    content: |
      mod writer;

      pub use writer::render;
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/emit/writer.rs
    mode: replace
    content: |
      use crate::ir::Project;

      pub fn render(project: &Project) -> String {
          project
              .modules
              .iter()
              .map(|module| format!("mod {};\n", module.name))
              .collect()
      }

      #[cfg(test)]
      #[path = "test/writer.rs"]
      mod tests;
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/emit/test/writer.rs
    mode: replace
    content: |
      use super::*;
      use crate::ir::Module;
      use std::path::PathBuf;

      #[test]
      fn empty_project_renders_nothing() {
          assert_eq!(render(&Project::new(PathBuf::from("/p"))), "");
      }

      #[test]
      fn renders_one_line_per_module() {
          let mut project = Project::new(PathBuf::from("/p"));
          for name in ["a", "b"] {
              project.modules.push(Module { name: name.into() });
          }
          assert_eq!(render(&project), "mod a;\nmod b;\n");
      }
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/cli/mod.rs
    mode: replace
    content: |
      mod args;
      mod dispatch;

      pub use args::{parse_args, Command};
      pub use dispatch::{execute, run};
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/cli/args.rs
    mode: replace
    content: |
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
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/cli/test/args.rs
    mode: replace
    content: |
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
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/cli/dispatch.rs
    mode: replace
    content: |
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
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/src/cli/test/dispatch.rs
    mode: replace
    content: |
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
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/tests/cli.rs
    mode: replace
    content: |
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
  - path: /home/user/xyan/xy.ai.workbench/ast-engines/rustir/tests/hygiene.rs
    mode: replace
    content: |
      use std::fs;
      use std::path::{Path, PathBuf};

      fn src_dir() -> PathBuf {
          Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
      }

      fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
          for entry in fs::read_dir(dir).unwrap() {
              let path = entry.unwrap().path();
              if path.is_dir() {
                  rust_files(&path, out);
              } else if path.extension().is_some_and(|ext| ext == "rs") {
                  out.push(path);
              }
          }
      }

      fn all_sources() -> Vec<PathBuf> {
          let mut files = Vec::new();
          rust_files(&src_dir(), &mut files);
          files.sort();
          files
      }

      fn file_name(path: &Path) -> &str {
          path.file_name().unwrap().to_str().unwrap()
      }

      fn in_test_dir(path: &Path) -> bool {
          path.parent().is_some_and(|p| file_name(p) == "test")
      }

      fn is_non_leaf(path: &Path) -> bool {
          matches!(file_name(path), "mod.rs" | "lib.rs")
      }

      fn is_leaf(path: &Path) -> bool {
          !in_test_dir(path) && !is_non_leaf(path) && file_name(path) != "main.rs"
      }

      fn layer(module: &str) -> Option<u8> {
          match module {
              "error" => Some(0),
              "ir" => Some(1),
              "parser" | "transform" | "emit" => Some(2),
              "cli" => Some(3),
              _ => None,
          }
      }

      fn top_module(path: &Path) -> Option<String> {
          let relative = path.strip_prefix(src_dir()).ok()?;
          let first = relative.components().next()?.as_os_str().to_str()?;
          let name = first.strip_suffix(".rs").unwrap_or(first);
          layer(name).map(|_| name.to_string())
      }

      #[test]
      fn sources_are_found() {
          assert!(all_sources().len() > 5);
      }

      #[test]
      fn non_leaf_modules_only_declare_and_reexport() {
          for path in all_sources().iter().filter(|p| is_non_leaf(p)) {
              let text = fs::read_to_string(path).unwrap();
              for line in text.lines().map(str::trim).filter(|l| !l.is_empty()) {
                  let allowed = line.starts_with("//")
                      || ((line.starts_with("mod ")
                          || line.starts_with("pub mod ")
                          || line.starts_with("use ")
                          || line.starts_with("pub use "))
                          && line.ends_with(';'));
                  assert!(allowed, "{}: not a declaration or re-export: {line}", path.display());
              }
          }
      }

      #[test]
      fn leaf_modules_declare_their_test_module() {
          for path in all_sources().iter().filter(|p| is_leaf(p)) {
              let name = file_name(path).trim_end_matches(".rs");
              let text = fs::read_to_string(path).unwrap();
              let block = format!("#[cfg(test)]\n#[path = \"test/{name}.rs\"]\nmod tests;\n");
              assert!(text.ends_with(&block), "{}: missing trailing test block", path.display());
              let test_file = path.parent().unwrap().join("test").join(format!("{name}.rs"));
              assert!(test_file.is_file(), "missing {}", test_file.display());
          }
      }

      #[test]
      fn test_files_belong_to_a_leaf_module() {
          for path in all_sources().iter().filter(|p| in_test_dir(p)) {
              let leaf = path.parent().unwrap().parent().unwrap().join(file_name(path));
              assert!(leaf.is_file(), "orphan test file {}", path.display());
          }
      }

      #[test]
      fn dependencies_point_only_to_lower_layers() {
          for path in all_sources() {
              let Some(own) = top_module(&path) else { continue };
              let own_layer = layer(&own).unwrap();
              let text = fs::read_to_string(&path).unwrap();
              assert!(!text.contains("super::super"), "{}: super::super path", path.display());
              for (name, other_layer) in ["error", "ir", "parser", "transform", "emit", "cli"]
                  .iter()
                  .map(|n| (*n, layer(n).unwrap()))
              {
                  if name == own || !text.contains(&format!("crate::{name}")) {
                      continue;
                  }
                  assert!(
                      other_layer < own_layer,
                      "{}: {own} must not depend on {name}",
                      path.display()
                  );
              }
          }
      }

      #[test]
      fn no_use_aliases() {
          for path in all_sources() {
              let text = fs::read_to_string(&path).unwrap();
              for line in text.lines().map(str::trim) {
                  let is_use = line.starts_with("use ") || line.starts_with("pub use ");
                  assert!(!(is_use && line.contains(" as ")), "{}: alias in {line}", path.display());
              }
          }
      }

      #[test]
      fn tests_do_not_depend_on_logging() {
          let forbidden = ["log::", "tracing::", "env_logger", "testing_logger", "tracing_test"];
          let mut files: Vec<PathBuf> = all_sources().into_iter().filter(|p| in_test_dir(p)).collect();
          rust_files(&Path::new(env!("CARGO_MANIFEST_DIR")).join("tests"), &mut files);
          for path in files.iter().filter(|p| file_name(p) != "hygiene.rs") {
              let text = fs::read_to_string(path).unwrap();
              for token in forbidden {
                  assert!(!text.contains(token), "{}: uses {token}", path.display());
              }
          }
      }
  reason: "Create the Rust project skeleton (Cargo.toml, layered modules with tests in test/ subdirs, README) and the hygiene/architecture tests in the tests directory."
```
/answer 6539-69-0 allow
Control Request:
```yaml
id: 6539-6a-0
toolName: run_command
arguments:
  items:
  - id: runTests
    cwd: /home/user/xyan/xy.ai.workbench/ast-engines/rustir
  reason: "Run the cargo test suite to verify the new project compiles and unit, CLI and hygiene tests pass."
```
/answer 6539-6a-0 allow
Control Request:
```yaml
id: 6539-6b-6a
result:
  structuredContent:
    results:
    - index: 0
      exit_code: 127
      stdout: ""
      stderr: |
        bash: line 1: scripts/test-compact.sh: No such file or directory
```
/answer 6539-6b-6a allow
ReasoningToken: 39
Control Request:
```yaml
id: 6539-6c-0
toolName: run_command
arguments:
  items:
  - id: runTests
    cwd: /home/user/xyan/xy.ai.workbench
  reason: The runTests command looks for scripts/test-compact.sh relative to cwd; retry from the workbench root where it may exist.
```
/answer 6539-6c-0 allow
Control Request:
```yaml
id: 6539-6d-6c
result:
  structuredContent:
    errors:
    - index: 0
      error: '''commands.json'' not found in working directory.'
```
/answer 6539-6d-6c allow
Result Stats: id=1f51da0e-4848-4d86-a0d1-712eb119c429, total: 36174, in: 12, out: 16379, reason: 83, read: 48429, write: 19783, start: 1795715404536, end: 1795715413097, time: 8489, total: 98414, io: 2, control: 72, sent: 0, recv: 220
ReasoningToken: 83
Ich habe das Projekt angelegt, aber noch nicht ausgeführt. `runTests` startet in `rustir` nicht, weil dort `scripts/test-compact.sh` fehlt (Exit 127). Im Workbench-Root gibt es keine `commands.json`. Ob die Tests grün sind, weiß ich daher nicht.

Ich habe kein Skript erfunden, weil ich nicht weiß, was `runTests` erwartet. Soll ich `scripts/test-compact.sh` anlegen, oder legst du es selbst ab? Als Inhalt würde ich `cargo test` nehmen. Alternativ führst du `cargo test` direkt in `rustir` aus.

Das Verzeichnis `rustir` existierte noch nicht. Ich habe es neu angelegt, nur `ast-engines` war schon da.

## Aufbau
Das Projekt hat nur Standardbibliothek-Abhängigkeiten. Parsen, Transformieren und Ausgeben sind Platzhalter, die man später ausbauen kann.

| Modul | Ebene | Inhalt |
|---|---|---|
| `error` | 0 | Fehlertyp |
| `ir` | 1 | `Project` und `Module` als IR-Platzhalter |
| `parser` | 2 | `parse_project` prüft, ob eine `Cargo.toml` existiert |
| `transform` | 2 | Trait `Transform` und `apply_all` |
| `emit` | 2 | `render` gibt `mod x;` pro Modul aus |
| `cli` | 3 | Argument-Parsing (`help` und `parse <dir>`) und Dispatch |

- **Aufruf:** `rustir help` und `rustir parse <dir>`. Exit-Code 2 steht für Usage-Fehler, 1 für alle anderen Fehler.
- **Modulregeln:** Nicht-Blatt-Module (`lib.rs`, `mod.rs`) enthalten nur `mod`- und `use`-Zeilen. Kindmodule liegen im Verzeichnis ihres Elternmoduls.
- **Unit-Tests:** Sie liegen in `<ebene>/test/<name>.rs`. Am Ende jeder Quelldatei steht der geforderte `#[cfg(test)] #[path = ...] mod tests;`-Block.

## Tests in `tests/`
- **`hygiene.rs`** prüft die Regeln aus deiner Liste:
  - Nicht-Blatt-Module enthalten nur Deklarationen und Re-Exports.
  - Jede Blattdatei hat den Testblock und die zugehörige Testdatei.
  - Abhängigkeiten zeigen nur auf niedrigere Ebenen, `super::super` ist verboten.
  - Es gibt keine `use … as`-Aliase.
  - Tests nutzen kein Logging.
- **`cli.rs`** startet die Binärdatei mit `help`, `parse` auf dem eigenen Projekt und einem unbekannten Befehl.

Die Regel „keine Plan-Referenzen in Kommentaren" prüfen die Tests nicht. Es gibt allerdings fast keine Kommentare.
Probier die Tests nochmal
SystemInit:  id=1f51da0e-4848-4d86-a0d1-712eb119c429, cwd=/home/user/xyan/xy.ai.workbench, model=claude-sonnet-5-5
Control Request:
```yaml
id: 6539-6e-0
toolName: run_command
arguments:
  items:
  - id: runTests
    cwd: /home/user/xyan/xy.ai.workbench/ast-engines/rustir
  reason: Re-run the cargo test suite in the rustir project as requested.
```
/answer 6539-6e-0 allow
Control Request:
```yaml
id: 6539-6f-6e
result:
  structuredContent:
    results:
    - index: 0
      stdout: |
        OK: 30 tests passed
```
/answer 6539-6f-6e allow
Result Stats: id=1f51da0e-4848-4d86-a0d1-712eb119c429, total: 37878, in: 16, out: 16638, reason: 0, read: 89230, write: 21224, start: 1795715475546, end: 1795715477306, time: 1713, total: 102841, io: 0, control: 47, sent: 0, recv: 29
Der Testlauf ist grün: 30 Tests bestanden. Dazu gehören die Unit-Tests, die CLI-Tests und die Hygiene-/Architektur-Tests. Beim ersten Versuch fehlte `scripts/test-compact.sh`. Jetzt läuft `runTests` offenbar durch.
