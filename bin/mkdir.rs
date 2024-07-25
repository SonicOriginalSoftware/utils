fn main() -> lib::Return {
    match lib::create_directory::run(std::env::args().collect::<Vec<String>>()) {
        Ok(_) => lib::error::Error::None,
        Err(e) => e,
    }
}
