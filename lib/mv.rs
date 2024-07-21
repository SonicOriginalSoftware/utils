use crate::error::Error;

pub enum Op {
    Move,
    Copy,
}

pub fn run(args: Vec<String>, op: Op) -> Result<(String, String), Error> {
    let source = match args.get(1) {
        Some(p) => p,
        None => return Err(Error::Str("No source given")),
    };

    let target = match args.get(2) {
        Some(p) => p,
        None => return Err(Error::Str("No destination given")),
    };

    match op {
        Op::Move => match std::fs::rename(source, target) {
            Ok(_) => Ok((source.to_string(), target.to_string())),
            Err(e) => Err(Error::IO(e)),
        },
        Op::Copy => match std::fs::copy(source, target) {
            Ok(_) => Ok((source.to_string(), target.to_string())),
            Err(e) => Err(Error::IO(e)),
        },
    }
}
