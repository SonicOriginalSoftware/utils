use std::process::ExitCode;

fn main() -> ExitCode {
    let command_name = "pwd";
    match lib::pwd::run() {
        Ok(p) => {
            println!("{}", p);
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}: {}", command_name, err);
            ExitCode::FAILURE
        }
    }
}
