use std::{env::args, process::ExitCode};

fn main() -> ExitCode {
    let command_name = "ls";
    let args = args().collect::<Vec<String>>();

    match lib::ls::run(&args) {
        Ok(entries) => {
            for each_path in entries {
                println!("{}", each_path);
            }
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}: {}", command_name, err);
            ExitCode::FAILURE
        }
    }
}
