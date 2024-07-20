#[cfg(test)]
#[test]
fn test_stat() {
    let args = vec![String::from(""), String::from("/")];
    match crate::stat::run(args) {
        Ok(p) => println!("{}", p),
        Err(e) => eprintln!("{}", e),
    }
}

#[test]
fn test_ls() {
    let args = vec![String::from(""), String::from("/")];
    let entries = match crate::ls::run(args) {
        Ok(p) => p,
        Err(e) => return eprintln!("{}", e),
    };
    for each_entry in entries {
        println!("{}", each_entry)
    }
}
