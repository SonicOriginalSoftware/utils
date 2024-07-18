use lib::error::Error;

fn main() -> Result<(), Error> {
    Ok(println!("{}", lib::pwd::run()?))
}
