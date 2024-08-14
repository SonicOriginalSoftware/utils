use std::env::args;

use lib::create_link::run;

fn main() -> lib::Return {
    if let Err(e) = run(args().collect::<Vec<String>>()) {
        return e;
    }
    lib::error::Error::None
}
