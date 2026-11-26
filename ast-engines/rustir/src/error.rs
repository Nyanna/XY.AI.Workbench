use std::fmt;
use std::path::PathBuf;
pub const CONFLICTS_FILE: &str = "name-conflicts.yaml";
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameConflict {
    pub name: String,
    pub files: Vec<PathBuf>,
}
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Usage(String),
    NotFound(PathBuf),
    Io(String),
    Parse { path: PathBuf, message: String },
    NameConflicts(Vec<NameConflict>),
}
pub type Result<T> = std::result::Result<T, Error>;
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Usage(message) => write!(f, "usage error: {message}"),
            Error::NotFound(path) => write!(f, "not found: {}", path.display()),
            Error::Io(message) => write!(f, "io error: {message}"),
            Error::Parse { path, message } => {
                write!(f, "parse error in {}: {message}", path.display())
            }
            Error::NameConflicts(conflicts) => {
                write!(f, "{} name conflict(s), see {CONFLICTS_FILE}", conflicts.len())
            }
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
