fn main() -> lib::Return {
    match lib::show_file::run(std::env::args().collect::<Vec<String>>()) {
        Ok(p) => println!("{}", p),
        Err(e) => return e,
    }
    lib::error::Error::None
}
