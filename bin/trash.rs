fn main() -> lib::Return {
    match lib::trash::run(std::env::args().collect::<Vec<String>>()) {
        Ok(_) => lib::error::Error::None,
        Err(e) => e,
    }
}
