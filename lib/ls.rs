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
        None => &match crate::pwd::run() {
            Ok(p) => p,
            Err(e) => return Err(e),
        },
    };

    let target_path = PathBuf::from(target);
    let target_copy = target_path.clone();

    let md = match symlink_metadata(target_path) {
        Ok(p) => p,
        Err(e) => return Err(Error::IO(e)),
    };
    let ki = Mode(md.mode()).file_type();

    let mut entries: Vec<File> = Vec::<File>::new();
    match ki {
        crate::fs::kind::Kind::Dir => {
            let dirs = match read_dir(target) {
                Ok(p) => p,
                Err(e) => return Err(Error::IO(e)),
            };

            for each_result_entry in dirs {
                let path = match each_result_entry {
                    Ok(p) => p.path(),
                    Err(e) => return Err(Error::IO(e)),
                };
                let file = match File::try_from(path) {
                    Ok(p) => p,
                    Err(e) => return Err(e),
                };
                entries.push(file)
            }
        }
        _ => {
            let file = match File::try_from(target_copy) {
                Ok(p) => p,
                Err(e) => return Err(e),
            };
            entries.push(file)
        }
    }

    entries.sort();
    Ok(entries)
}
