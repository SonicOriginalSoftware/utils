use std::fs::File;

use crate::error::Error;

pub fn run(args: Vec<String>) -> Result<(), Error> {
    let target = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No path passed")),
    };
    match File::create_new(target) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::IO(e)),
    }
}
