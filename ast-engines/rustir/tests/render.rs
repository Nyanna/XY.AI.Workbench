use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn tmp() -> &'static Path {
    Path::new(env!("CARGO_TARGET_TMPDIR"))
}

fn fresh(name: &str) -> PathBuf {
    let dir = tmp().join(format!("render-{name}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write(dir: &Path, relative: &str, text: &str) {
    let file = dir.join(relative);
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(file, text).unwrap();
}

fn rustir(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rustir"))
        .args(args)
        .output()
        .unwrap()
}

fn cargo(subcommand: &[&str], crate_dir: &Path) -> Output {
    let program = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    Command::new(program)
        .args(subcommand)
        .args(["--offline", "--manifest-path"])
        .arg(crate_dir.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", tmp().join("render-target"))
        .output()
        .unwrap()
}

fn render(project: &Path, output: &Path) {
    let result = rustir(&["render", project.to_str().unwrap(), "--out", output.to_str().unwrap()]);
    let stderr = String::from_utf8_lossy(&result.stderr).into_owned();
    assert!(result.status.success(), "render failed: {stderr}");
}

fn demo_project(dir: &Path) {
    write(dir, "Cargo.toml", "[package]\nname = \"demo-app\"\nversion = \"0.1.0\"\nedition = \"2021\"\n");
    write(dir, "src/lib.rs", "pub mod model;\npub mod shapes;\nmod util;\n#[macro_use]\nmod macros;\npub use model::Shape;\n");
    write(dir, "src/macros.rs", "macro_rules! double {\n    ($x:expr) => {\n        $x * 2\n    };\n}\n");
    write(
        dir,
        "src/model.rs",
        "use std::collections::BTreeMap;\nuse std::fmt;\n\nuse crate::util::clamp;\n\npub struct Shape {\n    name: String,\n    sides: u32,\n    tags: BTreeMap<String, u32>,\n}\n\nimpl Shape {\n    pub fn new(name: &str, sides: u32) -> Self {\n        Shape { name: name.to_string(), sides: clamp(sides), tags: BTreeMap::new() }\n    }\n\n    fn secret(&self) -> u32 {\n        self.sides + self.tags.len() as u32\n    }\n\n    pub fn total(&self) -> u32 {\n        double!(self.secret())\n    }\n}\n\nimpl fmt::Display for Shape {\n    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {\n        write!(f, \"{}:{}\", self.name, self.sides)\n    }\n}\n",
    );
    write(
        dir,
        "src/util.rs",
        "const LIMIT: u32 = 12;\n\npub fn clamp(value: u32) -> u32 {\n    if value > LIMIT {\n        LIMIT\n    } else {\n        value\n    }\n}\n",
    );
    write(
        dir,
        "src/shapes.rs",
        "use crate::model::Shape;\nuse crate::util::clamp;\n\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\npub enum Kind {\n    Triangle,\n    Square,\n}\n\npub fn describe(shape: &Shape) -> String {\n    format!(\"{shape} total={}\", shape.total())\n}\n\npub(super) fn corners(kind: Kind) -> u32 {\n    match kind {\n        Kind::Triangle => clamp(3),\n        Kind::Square => 4,\n    }\n}\n",
    );
    write(
        dir,
        "src/main.rs",
        "use demo_app::shapes::describe;\nuse demo_app::Shape;\n\nfn main() {\n    let shape = Shape::new(\"tri\", 3);\n    println!(\"{}\", describe(&shape));\n    println!(\"{}\", demo_app::shapes::Kind::Square as u8);\n}\n",
    );
}

#[test]
fn rendered_demo_project_compiles_and_behaves_like_the_original() {
    let project = fresh("demo-in");
    let output = fresh("demo-out").join("crate");
    demo_project(&project);
    render(&project, &output);
    let run = cargo(&["run", "--quiet"], &output);
    let stderr = String::from_utf8_lossy(&run.stderr).into_owned();
    assert!(run.status.success(), "generated crate failed: {stderr}");
    assert_eq!(String::from_utf8(run.stdout).unwrap(), "tri:3 total=6\n1\n");
}

#[test]
fn rendering_twice_into_the_same_directory_is_stable() {
    let project = fresh("twice-in");
    let output = fresh("twice-out").join("crate");
    demo_project(&project);
    render(&project, &output);
    let first = fs::read_to_string(output.join("src/lib.rs")).unwrap();
    render(&project, &output);
    assert_eq!(fs::read_to_string(output.join("src/lib.rs")).unwrap(), first);
}

#[test]
fn rendered_rustir_compiles() {
    let output = fresh("self").join("crate");
    render(Path::new(env!("CARGO_MANIFEST_DIR")), &output);
    let check = cargo(&["check", "--quiet"], &output);
    let stderr = String::from_utf8_lossy(&check.stderr).into_owned();
    assert!(check.status.success(), "generated crate failed: {stderr}");
}
