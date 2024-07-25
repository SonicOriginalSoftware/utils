use std::path::PathBuf;

use crate::{error::Error, fs::file::File};

pub fn run(args: Vec<String>) -> Result<File, Error> {
    match args.get(1) {
        Some(p) => File::try_from(PathBuf::from(p)),
        None => Err(Error::Str("No path given")),
    }
}
