use std::fs::read_dir;

use crate::{error::Error, fs::file::File};

pub fn run(args: Vec<String>) -> Result<Vec<File>, Error<'static>> {
    let target = match args.get(1) {
        Some(p) => p,
        None => &match crate::pwd::run() {
            Ok(p) => p,
            Err(e) => return Err(e),
        },
    };

    let dirs = match read_dir(target) {
        Ok(p) => p,
        Err(e) => return Err(Error::IO(e)),
    };

    let mut entries: Vec<File> = Vec::<File>::new();
    for each_result_entry in dirs {
        let entry = match each_result_entry {
            Ok(p) => p,
            Err(e) => return Err(Error::IO(e)),
        };
        let file = match File::new(entry) {
            Ok(p) => p,
            Err(e) => return Err(e),
        };
        entries.push(file)
    }

    entries.sort();
    Ok(entries)
}
