use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    Str(&'static str),
    String(String),
    Number(u32),
    Format(std::fmt::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::IO(e) => write!(f, "{}", e),
            Error::Str(e) => write!(f, "{}", e),
            Error::String(e) => write!(f, "{}", e),
            Error::Number(e) => write!(f, "{}", e),
            Error::Format(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}
