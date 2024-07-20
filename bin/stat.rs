fn main() -> Result<(), lib::error::Error> {
    Ok(println!(
        "{}",
        lib::stat::run(std::env::args().collect::<Vec<String>>())?
    ))
}
