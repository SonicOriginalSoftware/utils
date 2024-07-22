use crate::error::Error;

pub fn run(args: Vec<String>) -> Result<(), Error> {
    let target = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No path passed")),
    };
    // TODO Should we IO error if the target already exists?
    Ok(std::fs::create_dir_all(target)?)
}
