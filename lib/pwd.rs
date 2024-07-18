use std::env::current_dir;

use crate::error::Error;

pub fn run() -> Result<String, Error> {
    match current_dir()?.to_str() {
        Some(p) => Ok(p.to_owned()),
        None => Err(Error::String("Unable to translate working directory".to_string())),
    }
}
