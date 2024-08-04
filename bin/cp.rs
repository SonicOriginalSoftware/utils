use std::env::args;

use lib::copy::run;

fn main() -> lib::Return {
    if let Err(e) = run(args().collect::<Vec<String>>()) {
        return e;
    }
    lib::error::Error::None
}
