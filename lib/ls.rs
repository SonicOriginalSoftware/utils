use crate::pwd;
use std::{env::args, fs::read_dir, path::PathBuf};

pub fn run() -> Result<Vec<PathBuf>, String> {
    let args = args().collect::<Vec<String>>();

    let target = match args.get(1) {
        Some(p) => p,
        None => &match pwd::run() {
            Err(err) => return Err(err.to_string()),
            Ok(p) => p,
        },
    };

    let dirs = match read_dir(target) {
        Ok(p) => p,
        Err(err) => return Err(err.to_string()),
    };

    let entries = dirs
        .filter(|each_entry| each_entry.is_ok())
        .map(|each_entry| each_entry.unwrap().path())
        .collect::<Vec<PathBuf>>();
    Ok(entries)
}
