fn main() -> lib::Return {
    let entries = match lib::show::run(std::env::args().collect::<Vec<String>>()) {
        Ok(p) => p,
        Err(e) => return e,
    };

    for each_entry in entries {
        println!("{}", each_entry)
    }
    lib::error::Error::None
}
