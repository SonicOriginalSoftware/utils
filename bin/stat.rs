use std::env::args;

use lib::{error::Error, stat};

fn main() -> Result<(), Error> {
    Ok(println!("{}", stat::run(args().collect::<Vec<String>>())?))
}
