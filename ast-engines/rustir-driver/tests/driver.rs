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
    pub use self::Config as Settings;
    pub fn aliased(_: Settings) -> Settings {
        Settings
    }
    pub fn spelled_plainly(_: Config) {}
    pub struct Limits {
        pub max: u8,
    }
    const DEFAULT_MAX: u8 = 16;
    pub trait Describe {
        fn describe(&self) -> u8;
    }
    impl Default for Limits {
        fn default() -> Self {
            Limits { max: DEFAULT_MAX }
        }
    }
    impl Describe for Limits {
        fn describe(&self) -> u8 {
            self.max
        }
    }
    pub type Slot = u64;
    impl From<Config> for Slot {
        fn from(_: Config) -> Self {
            0
        }
    }
}
pub fn outside() -> resident::Config {
    resident::Config
}
"#;
fn run(name: &str, prefix: &str) -> String {
    let dir = std::env::temp_dir()
        .join(format!("rustir-driver-{name}-{}", std::process::id()));
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
    fs::read_to_string(out).unwrap()
}
fn analyze(name: &str, prefix: &str) -> Vec<(String, String)> {
    run(name, prefix)
        .lines()
        .filter(|line| line.matches('\t').count() == 1)
        .map(|line| {
            let (owner, dependency) = line.split_once('\t').unwrap();
            (owner.to_string(), dependency.to_string())
        })
        .collect()
}
fn aliases_of(name: &str, prefix: &str) -> Vec<(String, String, String)> {
    run(name, prefix)
        .lines()
        .filter(|line| line.matches('\t').count() == 2)
        .map(|line| {
            let mut columns = line.split('\t').map(str::to_string);
            (columns.next().unwrap(), columns.next().unwrap(), columns.next().unwrap())
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
    assert!(run.contains(& "Loader"), "{run:?}");
    assert!(run.contains(& "Config"), "{run:?}");
}
#[test]
fn foreign_trait_impl_body_counts_for_the_type() {
    let edges = analyze("colocated_body", "resident");
    let limits = deps_of(&edges, "Limits");
    assert!(limits.contains(& "DEFAULT_MAX"), "{limits:?}");
}
#[test]
fn local_trait_impl_header_counts_for_the_type() {
    let edges = analyze("colocated_header", "resident");
    let limits = deps_of(&edges, "Limits");
    assert!(limits.contains(& "Describe"), "{limits:?}");
}
#[test]
fn impl_for_a_local_alias_is_attributed_to_the_alias() {
    let edges = analyze("alias_owner", "resident");
    let slot = deps_of(&edges, "Slot");
    assert!(slot.contains(& "Config"), "{slot:?}");
}
#[test]
fn impl_for_foreign_type_is_attributed_to_the_local_trait() {
    let edges = analyze("owner", "resident");
    assert!(deps_of(& edges, "Loader").contains(& "Config"));
}
#[test]
fn external_dependencies_are_full_paths() {
    let edges = analyze("external", "resident");
    let holder = deps_of(&edges, "Holder");
    assert!(holder.contains(& "Config"));
    assert!(
        holder.iter().any(| d | d.ends_with("HashMap") && d.contains("::")), "{holder:?}"
    );
}
#[test]
fn items_outside_the_prefix_are_not_objects() {
    let edges = analyze("prefix", "resident");
    assert!(edges.iter().all(| (owner, _) | owner != "outside"));
    assert!(edges.iter().any(| (owner, _) | owner == "unrelated"));
}
#[test]
fn renamed_paths_resolve_to_their_target_object() {
    let aliases = aliases_of("aliases", "resident");
    let expected = ("aliased".to_string(), "Config".to_string(), "Settings".to_string());
    assert!(aliases.contains(& expected));
    assert!(aliases.iter().all(| (owner, _, _) | owner != "spelled_plainly"));
    assert!(aliases.iter().all(| (_, target, alias) | target != alias));
}
