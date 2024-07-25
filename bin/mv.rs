fn main() -> lib::Return {
    match lib::rename_file::run(std::env::args().collect::<Vec<String>>()) {
        Ok((source, target)) => println!("Moved '{}' to '{}'", source, target),
        Err(e) => return e,
    }
    lib::error::Error::None
}
