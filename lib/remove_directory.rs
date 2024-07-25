use crate::error::Error;

pub fn run(args: Vec<String>) -> Result<(), Error> {
    let target = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No path passed")),
    };

    Ok(std::fs::remove_dir_all(target)?)
}
