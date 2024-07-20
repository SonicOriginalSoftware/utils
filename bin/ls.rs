fn main() -> lib::Return {
    let entries = match lib::ls::run(std::env::args().collect::<Vec<String>>()) {
        Ok(p) => p,
        Err(e) => return Err(e),
    };

    for each_entry in entries {
        println!("{}", each_entry)
    }
    Ok(())
}
