fn main() -> lib::Return {
    match lib::copy::run(std::env::args().collect::<Vec<String>>()) {
        Ok((source, target)) => println!("Copied '{}' to '{}'", source, target),
        Err(e) => return e,
    };
    lib::error::Error::None
}
