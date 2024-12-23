use std::fmt::{self, Debug, Display, Formatter};
use std::{error, io, str};

pub struct Error(Box<ErrorImpl>);

#[derive(Debug)]
pub enum ErrorKind {
    Utf8Error,
    #[allow(dead_code)]
    Io(io::ErrorKind),
    JsonDeserialize,
    YamlDeserialize,
    TomlDeserialize,
}

#[derive(Debug)]
pub struct ErrorImpl {
    kind: ErrorKind,
    error: Box<dyn error::Error + Send + Sync>,
}

impl From<str::Utf8Error> for Error {
    fn from(e: str::Utf8Error) -> Self {
        Self(Box::new(ErrorImpl {
            kind: ErrorKind::Utf8Error,
            error: Box::new(e),
        }))
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Self(Box::new(ErrorImpl {
            kind: ErrorKind::Io(e.kind()),
            error: Box::new(e),
        }))
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self(Box::new(ErrorImpl {
            kind: ErrorKind::JsonDeserialize,
            error: Box::new(e),
        }))
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(e: serde_yaml::Error) -> Self {
        Self(Box::new(ErrorImpl {
            kind: ErrorKind::YamlDeserialize,
            error: Box::new(e),
        }))
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Self(Box::new(ErrorImpl {
            kind: ErrorKind::TomlDeserialize,
            error: Box::new(e),
        }))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(&format!("{:?}: {}", &self.0.kind, &self.0.error), f)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&format!("{:?}", &self.0), f)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
