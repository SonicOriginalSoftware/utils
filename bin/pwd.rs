fn main() -> lib::Return {
    match lib::current_directory::run() {
        Ok(p) => println!("{}", p),
        Err(e) => return e,
    };
    lib::error::Error::None
}
