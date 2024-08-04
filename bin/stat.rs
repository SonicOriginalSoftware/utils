use std::env::args;

use lib::show_file::run;

fn main() -> lib::Return {
    match run(args().collect::<Vec<String>>()) {
        Ok(p) => {
            println!("{}", p);
            lib::error::Error::None
        }
        Err(e) => e,
    }
}
