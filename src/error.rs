use std::error;
use std::fmt::{self, Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum ErrorKind {
    Io(io::ErrorKind),
    JsonDeserialize,
    YamlDeserialize,
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    error: Box<dyn error::Error + Send + Sync>,
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self {
            kind: ErrorKind::Io(e.kind()),
            error: Box::new(e),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self {
            kind: ErrorKind::JsonDeserialize,
            error: Box::new(e),
        }
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Self {
            kind: ErrorKind::YamlDeserialize,
            error: Box::new(e),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&format!("{:?}: {}", &self.kind, &self.error), f)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
