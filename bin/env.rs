fn main() -> lib::Return {
    println!("{}", lib::env::Vars(std::env::vars().collect()));
    lib::error::Error::None
}
