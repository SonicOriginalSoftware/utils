use std::process::ExitCode;

fn main() -> ExitCode {
    let command_name = "ls";
    match lib::ls::run() {
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
