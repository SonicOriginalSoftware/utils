use std::env::args;

use lib::remove_file::run;

fn main() -> lib::Return {
    if let Err(e) = run(args().collect::<Vec<String>>()) {
        return e;
    }
    lib::error::Error::None
}
