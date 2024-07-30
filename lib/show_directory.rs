use std::{fs::read_dir, path::PathBuf};

use crate::{error::Error, fs::file::File};

pub fn run(args: Vec<String>) -> Result<Vec<File>, Error> {
    let target = match args.get(1) {
        Some(p) => p,
        None => &match crate::current_directory::run() {
            Ok(p) => p,
            Err(e) => return Err(e),
        },
    };

    let mut entries: Vec<File> = Vec::<File>::new();
    let file = match File::try_from(PathBuf::from(target)) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };
    if let crate::fs::kind::Kind::Dir = file.kind {
        for each_result_entry in read_dir(target)? {
            entries.push(File::try_from(each_result_entry?.path())?)
        }
    } else {
        entries.push(file);
    }

    entries.sort();
    Ok(entries)
}
