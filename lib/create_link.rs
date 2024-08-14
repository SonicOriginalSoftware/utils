use std::os::unix::fs::symlink;

use crate::error::Error;

pub fn run(args: Vec<String>) -> Result<(), Error> {
    let link_origin = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No source given")),
    };

    let link_destination = match args.get(2) {
        Some(p) => p,
        None => return Err(Error::Str("No destination given")),
    };

    match symlink(link_origin, link_destination) {
        Ok(p) => Ok(p),
        Err(err) => Err(Error::IO(err)),
    }
}
