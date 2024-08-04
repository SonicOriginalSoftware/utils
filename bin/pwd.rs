use lib::current_directory::run;

fn main() -> lib::Return {
    match run() {
        Ok(p) => {
            println!("{}", p);
            lib::error::Error::None
        }
        Err(e) => e,
    }
}
