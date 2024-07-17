#[cfg(test)]
#[test]
fn test_ls() {
    use crate::ls::run;
    match run(&[String::from("")]) {
        Err(err) => panic!("{}", err),
        Ok(entries) => {
            for each_path in entries {
                println!("{}", each_path);
            }
        }
    }
}
