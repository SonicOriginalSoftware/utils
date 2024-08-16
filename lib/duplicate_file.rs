use std::{fs::copy, path::Path};

use crate::error::Error;

pub fn run(args: Vec<String>) -> Result<(), Error> {
    let source = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No source given")),
    };

    let target = match args.get(2) {
        Some(p) => p,
        None => return Err(Error::Str("No destination given")),
    };

    let mut target_path = Path::new(&target).to_owned();
    if target_path.exists() && target_path.is_dir() {
        target_path = target_path.join(source);
    }

    match copy(source, target_path) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::IO(e)),
    }
}
