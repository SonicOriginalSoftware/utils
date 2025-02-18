use std::env::args;

use lib::change_ownership::run;

fn main() -> lib::Return {
    if let Err(e) = run(args().collect::<Vec<String>>()) {
        return e;
    };
    lib::error::Error::None
}
