use std::fs::File;
use std::io::Read;

use crate::error::Error;

pub fn run(args: Vec<String>) -> Result<String, Error> {
    let target = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No path passed")),
    };

    let mut file = match File::open(target) {
        Ok(p) => p,
        Err(err) => return Err(Error::IO(err)),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_p) => Ok(contents),
        Err(err) => Err(Error::IO(err)),
    }
}
