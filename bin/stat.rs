fn main() -> lib::Return {
    match lib::stat::run(std::env::args().collect::<Vec<String>>()) {
        Ok(p) => println!("{}", p),
        Err(e) => return e,
    }
    lib::error::Error::None
}
