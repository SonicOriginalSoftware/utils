use std::env::args;

use lib::show_directory::run;

fn main() -> lib::Return {
    let entries = match run(args().collect::<Vec<String>>()) {
        Ok(p) => p,
        Err(e) => return e,
    };

    for each_entry in entries {
        println!("{}", each_entry)
    }
    lib::error::Error::None
}
