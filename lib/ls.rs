use std::fs::read_dir;

use crate::{error::Error, fs::file::File, pwd};

pub fn run(args: &[String]) -> Result<Vec<File>, Error> {
    let target = match args.get(1) {
        Some(p) => p,
        None => &pwd::run()?,
    };

    let mut entries: Vec<File> = read_dir(target)?
        .map(|p| File::new(&p.map_err(Error::IO)?))
        .collect::<Result<_, _>>()?;

    entries.sort();
    Ok(entries)
}
