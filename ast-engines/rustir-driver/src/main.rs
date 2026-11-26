#![feature(rustc_private)]
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;
mod args;
mod collect;
mod render;
use std::process::{Command, ExitCode};
struct Passive;
impl rustc_driver::Callbacks for Passive {}
fn sysroot_of(compiler: &str) -> Option<String> {
    let output = Command::new(compiler).args(["--print", "sysroot"]).output().ok()?;
    let text = String::from_utf8(output.stdout).ok()?;
    Some(text.trim().to_string())
}
fn main() -> ExitCode {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    let Some(plan) = args::plan(&raw, |name| std::env::var(name).ok()) else {
        eprintln!("rustir-driver: expected the compiler path as first argument");
        return ExitCode::from(2);
    };
    let mut compiler_args = plan.compiler_args;
    if !args::has_sysroot(&compiler_args) {
        if let Some(root) = sysroot_of(&compiler_args[0]) {
            compiler_args.push("--sysroot".into());
            compiler_args.push(root);
        }
    }
    match plan.analysis {
        Some(analysis) => {
            let mut callbacks = collect::Collect::new(analysis.prefix, analysis.out);
            rustc_driver::compiler_entrypoint(&compiler_args, &mut callbacks);
        }
        None => rustc_driver::compiler_entrypoint(&compiler_args, &mut Passive),
    }
    ExitCode::SUCCESS
}
