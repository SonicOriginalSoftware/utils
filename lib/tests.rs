#[cfg(test)]
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
