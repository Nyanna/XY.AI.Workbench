use std::fs;
use std::path::PathBuf;
use std::process::Command;

const SAMPLE: &str = r#"
pub mod resident {
    pub struct Config;
    pub trait Loader {
        fn load(&self) -> Config;
    }
    impl Loader for Vec<u8> {
        fn load(&self) -> Config {
            Config
        }
    }
    pub struct Holder {
        pub map: std::collections::HashMap<u8, Config>,
    }
    pub fn run(v: Vec<u8>) -> Config {
        v.load()
    }
    pub fn unrelated() -> u8 {
        1
    }
}
pub fn outside() -> resident::Config {
    resident::Config
}
"#;

fn analyze(name: &str, prefix: &str) -> Vec<(String, String)> {
    let dir = std::env::temp_dir().join(format!("rustir-driver-{name}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"sample\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
    )
    .unwrap();
    fs::write(dir.join("src/lib.rs"), SAMPLE).unwrap();
    let out: PathBuf = dir.join("edges.txt");
    let status = Command::new("cargo")
        .args(["check", "--lib"])
        .current_dir(&dir)
        .env("RUSTC_WORKSPACE_WRAPPER", env!("CARGO_BIN_EXE_rustir-driver"))
        .env("RUSTIR_OUT", &out)
        .env("RUSTIR_PREFIX", prefix)
        .env("CARGO_TARGET_DIR", dir.join("target"))
        .status()
        .unwrap();
    assert!(status.success());
    fs::read_to_string(out)
        .unwrap()
        .lines()
        .map(|line| {
            let (owner, dependency) = line.split_once('\t').unwrap();
            (owner.to_string(), dependency.to_string())
        })
        .collect()
}

fn deps_of<'a>(edges: &'a [(String, String)], owner: &str) -> Vec<&'a str> {
    edges
        .iter()
        .filter(|(o, d)| o == owner && !d.is_empty())
        .map(|(_, d)| d.as_str())
        .collect()
}

#[test]
fn method_calls_resolve_to_the_exact_trait() {
    let edges = analyze("trait", "resident");
    let run = deps_of(&edges, "run");
    assert!(run.contains(&"Loader"), "{run:?}");
    assert!(run.contains(&"Config"), "{run:?}");
}

#[test]
fn impl_for_foreign_type_is_attributed_to_the_local_trait() {
    let edges = analyze("owner", "resident");
    assert!(deps_of(&edges, "Loader").contains(&"Config"));
}

#[test]
fn external_dependencies_are_full_paths() {
    let edges = analyze("external", "resident");
    let holder = deps_of(&edges, "Holder");
    assert!(holder.contains(&"Config"));
    assert!(holder.iter().any(|d| d.ends_with("HashMap") && d.contains("::")), "{holder:?}");
}

#[test]
fn items_outside_the_prefix_are_not_objects() {
    let edges = analyze("prefix", "resident");
    assert!(edges.iter().all(|(owner, _)| owner != "outside"));
    assert!(edges.iter().any(|(owner, _)| owner == "unrelated"));
}
