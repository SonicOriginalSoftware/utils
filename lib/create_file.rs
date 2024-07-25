use std::fs::File;

use crate::error::Error;

pub fn run(args: Vec<String>) -> Result<File, Error> {
    let target = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No path passed")),
    };
    Ok(File::create_new(target)?)
}
