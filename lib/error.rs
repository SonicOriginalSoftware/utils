use std::fmt::{Debug, Display};

// #[derive(Debug)]
pub enum Error {
    IO(std::io::Error),
    String(String),
    Number(u32),
    Format(std::fmt::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<String> for Error {
    fn from(err: String) -> Self {
        Self::String(err)
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(p) => write!(f, "{}", p),
            Self::String(p) => write!(f, "{}", p),
            Self::Number(p) => write!(f, "{}", p),
            Self::Format(p) => write!(f, "{}", p),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
