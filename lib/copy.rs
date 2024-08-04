use crate::error::Error;

pub fn run(args: Vec<String>) -> Result<(), Error> {
    let source = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No source given")),
    };

    let target = match args.get(2) {
        Some(p) => p,
        None => return Err(Error::Str("No destination given")),
    };

    match std::fs::copy(source, target) {
        Ok(_) => Ok(()),
        Err(e) => Err(Error::IO(e)),
    }
}
