fn main() -> Result<(), lib::error::Error> {
    match lib::mv::run(lib::mv::Op::Move) {
        Ok((source, target)) => Ok(println!("Moved '{}' to '{}'", source, target)),
        Err(e) => Err(e),
    }
}
