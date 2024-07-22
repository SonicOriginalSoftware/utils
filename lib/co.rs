use crate::error::Error;

pub fn run(args: Vec<String>) -> Result<(), Error> {
    let owner = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No owner passed")),
    };

    let group = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No group passed")),
    };

    let target = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No path passed")),
    };

    Ok(())

    // Ok(std::fs::set_permissions(target)?)
}
