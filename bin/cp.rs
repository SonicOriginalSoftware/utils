fn main() -> lib::Return {
    match lib::mv::run(std::env::args().collect::<Vec<String>>(), lib::mv::Op::Copy) {
        Ok((source, target)) => println!("Copied '{}' to '{}'", source, target),
        Err(e) => return e,
    };
    lib::error::Error::None
}
