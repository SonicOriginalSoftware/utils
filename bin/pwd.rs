fn main() -> Result<(), lib::error::Error<'static>> {
    match lib::pwd::run() {
        Ok(p) => Ok(println!("{}", p)),
        Err(e) => Err(e),
    }
}
