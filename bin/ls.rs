use std::env::args;

use lib::error::Error;

fn main() -> Result<(), Error<'static>> {
    let args = args().collect::<Vec<String>>();

    let entries = match lib::ls::run(args) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };

    for each_entry in entries {
        println!("{}", each_entry)
    }
    Ok(())
}
