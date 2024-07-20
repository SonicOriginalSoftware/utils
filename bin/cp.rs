fn main() -> Result<(), lib::error::Error> {
    match lib::mv::run(lib::mv::Op::Copy) {
        Ok((source, target)) => Ok(println!("Copied '{}' to '{}'", source, target)),
        Err(e) => Err(e),
    }
}
