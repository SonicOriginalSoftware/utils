use std::path::PathBuf;

use crate::{error::Error, fs::file::File};

pub fn run(args: Vec<String>) -> Result<File, Error> {
    let target = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No path given")),
    };

    File::try_from(PathBuf::from(target))
}
