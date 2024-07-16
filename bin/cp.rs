use std::process::ExitCode;

fn main() -> ExitCode {
    let command_name = "cp";
    match lib::cp::run() {
        Ok((source, target)) => {
            println!("{}: Copied '{}' to '{}'", command_name, source, target);
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}: {}", command_name, err);
            ExitCode::FAILURE
        }
    }
}
