use std::env::current_dir;

use crate::error::Error;

pub fn run() -> Result<String, Error<'static>> {
    match current_dir() {
        Ok(p) => Ok(p.display().to_string()),
        Err(e) => Err(Error::IO(e)),
    }
}
