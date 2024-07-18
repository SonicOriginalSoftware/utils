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
            Self::IO(p) => f.debug_tuple("IO").field(p).finish(),
            Self::String(p) => f.debug_tuple("String").field(p).finish(),
            Self::Number(p) => f.debug_tuple("Number").field(p).finish(),
            Error::Format(p) => f.debug_tuple("Format").field(p).finish(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
