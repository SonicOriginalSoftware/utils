use std::env::current_dir;

pub fn run() -> Result<String, String> {
    let cwd = match current_dir() {
        Ok(p) => p,
        Err(err) => return Err(format!("{}", err)),
    };

    match cwd.to_str() {
        Some(p) => Ok(p.to_owned()),
        None => Err("Unable to translate working directory".to_string()),
    }
}
