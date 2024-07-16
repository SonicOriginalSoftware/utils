use std::process::ExitCode;

fn main() -> ExitCode {
    let command_name = "ls";
    match lib::ls::run() {
        Ok(entries) => {
            for each_path in entries {
                match each_path.to_str() {
                    Some(p) => println!("{}", p),
                    None => eprintln!("{}: Could not translate path - {:?}", command_name, each_path),
                }
            }
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}: {}", command_name, err);
            ExitCode::FAILURE
        }
    }
}
