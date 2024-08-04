use std::env::args;

use crate::show_file::run;

pub fn read() -> crate::Return {
    match run(args().collect::<Vec<String>>()) {
        Ok(p) => {
            println!("{}", p);
            crate::error::Error::None
        }
        Err(e) => e,
    }
}
