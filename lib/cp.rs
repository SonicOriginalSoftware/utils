use std::env::args;

pub fn run() -> Result<(String, String), String> {
    let args = args().collect::<Vec<String>>();

    let source = match args.get(1) {
        Some(p) => p,
        None => return Err("No source given".to_string()),
    };

    let target = match args.get(2) {
        Some(p) => p,
        None => return Err("No destination given".to_string()),
    };

    match std::fs::copy(source, target) {
        Ok(_) => Ok((source.to_string(), target.to_string())),
        Err(err) => Err(format!("{}", err)),
    }
}
