use std::{
    fs::{read_dir, symlink_metadata},
    os::unix::fs::MetadataExt,
    path::PathBuf,
};

use crate::{
    error::Error,
    fs::{file::File, mode::Mode},
};

pub fn run(args: Vec<String>) -> Result<Vec<File>, Error> {
    let target = match args.get(1) {
        Some(p) => p,
        None => &match crate::current_directory::run() {
            Ok(p) => p,
            Err(e) => return Err(e),
        },
    };

    let target_path = PathBuf::from(target);

    let mut entries: Vec<File> = Vec::<File>::new();
    match Mode(symlink_metadata(&target_path)?.mode()).file_type() {
        crate::fs::kind::Kind::Dir => {
            for each_result_entry in read_dir(target)? {
                entries.push(File::try_from(each_result_entry?.path())?)
            }
        }
        _ => {
            let file = match File::try_from(target_path) {
                Ok(p) => p,
                Err(e) => return Err(e),
            };
            entries.push(file)
        }
    }

    entries.sort();
    Ok(entries)
}
