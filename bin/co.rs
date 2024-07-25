fn main() -> lib::Return {
    match lib::change_ownership::run(std::env::args().collect::<Vec<String>>()) {
        Ok(_p) => (),
        Err(e) => return e,
    };
    lib::error::Error::None
}
