fn main() -> lib::Return {
    match lib::show_env::run() {
        Ok(p) => println!("{}", p),
        Err(e) => return e,
    };
    lib::error::Error::None
}
