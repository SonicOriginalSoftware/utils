fn main() -> lib::Return {
    match lib::pwd::run() {
        Ok(p) => Ok(println!("{}", p)),
        Err(e) => Err(e),
    }
}
