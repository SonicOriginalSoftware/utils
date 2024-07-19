use std::fmt::{Debug, Display};

#[derive(Debug)]
pub enum Error<'a> {
    IO(std::io::Error),
    Str(&'a str),
    String(String),
    Number(u32),
    Format(std::fmt::Error),
}

impl<'a> From<std::io::Error> for Error<'a> {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<'a> std::error::Error for Error<'a> {}
