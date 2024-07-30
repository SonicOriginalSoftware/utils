#[cfg(test)]
#[test]
fn test_stat() {
    match crate::show_file::run(vec![String::from(""), String::from("/")]) {
        Ok(p) => println!("{}", p),
        Err(e) => eprintln!("{}", e),
    }
}

#[test]
fn test_ls() {
    let entries = match crate::show_directory::run(vec![String::from(""), String::from("/")]) {
        Ok(p) => p,
        Err(e) => return eprintln!("{}", e),
    };
    for each_entry in entries {
        println!("{}", each_entry)
    }
}
