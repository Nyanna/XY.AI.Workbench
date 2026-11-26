#[derive(Debug, PartialEq, Eq)]
pub struct Analysis {
    pub prefix: String,
    pub out: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Plan {
    pub compiler_args: Vec<String>,
    pub analysis: Option<Analysis>,
}

pub fn plan<F>(raw: &[String], env: F) -> Option<Plan>
where
    F: Fn(&str) -> Option<String>,
{
    raw.first()?;
    let analysis = match (env("CARGO_PRIMARY_PACKAGE"), env("RUSTIR_OUT")) {
        (Some(_), Some(out)) if is_analyzed_target(raw) => Some(Analysis {
            prefix: env("RUSTIR_PREFIX").unwrap_or_default(),
            out,
        }),
        _ => None,
    };
    Some(Plan {
        compiler_args: raw.to_vec(),
        analysis,
    })
}

pub fn has_sysroot(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "--sysroot" || arg.starts_with("--sysroot="))
}

fn is_analyzed_target(raw: &[String]) -> bool {
    let named = crate_name(raw).is_some_and(|name| !name.starts_with("build_script"));
    named && !raw.iter().any(|arg| arg.starts_with("--print"))
}

fn crate_name(raw: &[String]) -> Option<&str> {
    let position = raw.iter().position(|arg| arg == "--crate-name")?;
    raw.get(position + 1).map(String::as_str)
}

#[cfg(test)]
#[path = "test/args.rs"]
mod tests;
