use std::process::ExitCode;

use lib::mv::Op;

fn main() -> ExitCode {
    let command_name = "cp";
    match lib::mv::run(Op::Copy) {
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
