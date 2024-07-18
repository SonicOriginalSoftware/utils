#[cfg(test)]
#[test]
fn test_ls() {
    let args = [String::from(""), String::from("/")];
    let entries = crate::ls::run(&args).unwrap();
    for each_entry in entries {
        println!("{}", each_entry)
    }
}
