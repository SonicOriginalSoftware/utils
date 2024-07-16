use crate::pwd;
use std::{env::args, fs::read_dir};

pub fn run() -> Result<Vec<String>, String> {
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
        .filter_map(|each_entry| each_entry.ok())
        .filter_map(|each_entry| each_entry.file_name().into_string().ok());

    Ok(entries.collect::<Vec<String>>())
}
