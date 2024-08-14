use std::env::args;

use lib::concatenate::run;

fn main() -> lib::Return {
    match run(args().collect::<Vec<String>>()) {
        Ok(p) => {
            print!("{}", p);
            lib::error::Error::None
        }
        Err(e) => e,
    }
}
