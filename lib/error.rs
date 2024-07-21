use std::{
    fmt::{Debug, Display},
    process::{ExitCode, Termination},
};

#[derive(Debug)]
pub enum Error {
    None,
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
            Error::None => write!(f, ""),
            Error::IO(e) => write!(f, "{}", e),
            Error::Str(e) => write!(f, "{}", e),
            Error::String(e) => write!(f, "{}", e),
            Error::Number(e) => write!(f, "{}", e),
            Error::Format(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for Error {}

impl Termination for Error {
    fn report(self) -> ExitCode {
        if let Error::None = self {
            return ExitCode::SUCCESS;
        };
        eprintln!("{}", self);
        ExitCode::FAILURE
    }
}
