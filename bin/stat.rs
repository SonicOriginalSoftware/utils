fn main() -> lib::Return {
    Ok(println!(
        "{}",
        lib::stat::run(std::env::args().collect::<Vec<String>>())?
    ))
}
