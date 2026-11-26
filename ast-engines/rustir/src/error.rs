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
