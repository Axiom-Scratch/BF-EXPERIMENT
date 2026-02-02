use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum Error {
    Usage,
    ReadFailed { path: PathBuf, source: std::io::Error },
    WriteFailed { path: PathBuf, source: std::io::Error },
    InvalidUtf8 { path: PathBuf },
    IncludeError { path: PathBuf, message: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Usage => write!(f, "Usage: bfpp <input.bfpp> -o <output.bf>"),
            Error::ReadFailed { path, source } => {
                write!(f, "failed to read '{}': {}", path.display(), source)
            }
            Error::WriteFailed { path, source } => {
                write!(f, "failed to write '{}': {}", path.display(), source)
            }
            Error::InvalidUtf8 { path } => {
                write!(f, "invalid UTF-8 in '{}'", path.display())
            }
            Error::IncludeError { path, message } => {
                write!(f, "include error in '{}': {}", path.display(), message)
            }
        }
    }
}
