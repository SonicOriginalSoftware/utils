use std::process::ExitCode;

use lib::mv::Op;

fn main() -> ExitCode {
    let command_name = "mv";
    match lib::mv::run(Op::Move) {
        Ok((source, target)) => {
            println!("{}: Moved '{}' to '{}'", command_name, source, target);
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}: {}", command_name, err);
            ExitCode::FAILURE
        }
    }
}
