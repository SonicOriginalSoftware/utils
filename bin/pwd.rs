fn main() -> Result<(), lib::error::Error> {
    match lib::pwd::run() {
        Ok(p) => Ok(println!("{}", p)),
        Err(e) => Err(e),
    }
}
