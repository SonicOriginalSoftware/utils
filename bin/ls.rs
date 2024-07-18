use std::env::args;

use lib::error::Error;

fn main() -> Result<(), Error> {
    let args = args().collect::<Vec<String>>();
    let entries = lib::ls::run(&args)?;
    for each_entry in entries {
        println!("{}", each_entry)
    }
    Ok(())
}
