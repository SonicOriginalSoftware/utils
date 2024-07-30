use crate::error::Error;

pub fn run() -> Result<String, Error> {
    match std::env::current_dir() {
        Ok(p) => Ok(p.display().to_string()),
        Err(e) => Err(Error::IO(e)),
    }
}
