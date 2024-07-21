fn main() -> lib::Return {
    match lib::pwd::run() {
        Ok(p) => println!("{}", p),
        Err(e) => return e,
    };
    lib::error::Error::None
}
