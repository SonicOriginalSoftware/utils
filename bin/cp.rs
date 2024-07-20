fn main() -> lib::Return {
    match lib::mv::run(lib::mv::Op::Copy) {
        Ok((source, target)) => Ok(println!("Copied '{}' to '{}'", source, target)),
        Err(e) => Err(e),
    }
}
