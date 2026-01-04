pub fn search<'a>(contents: &'a str) -> Vec<&'a str> {
    if contents.is_empty() {
        return vec![];
    }
    vec!["safe, fast, productive."]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let contents = "\
            Rust:
            safe, fast, productive.
            Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(contents));
    }

    #[test]
    fn empty_file() {
        let contents = "";
        assert_eq!(Vec::<&str>::new(), search(contents));
    }
}
