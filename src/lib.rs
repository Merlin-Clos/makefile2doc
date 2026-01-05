pub fn search<'a>(contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.starts_with("## @"))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_documentation_lines() {
        let contents = "\
## @target: build
## @name: up
## @desc: Start the docker container
up:
    docker compose up
";

        assert_eq!(
            vec![
                "## @target: build",
                "## @name: up",
                "## @desc: Start the docker container"
            ],
            search(contents)
        );
    }
}
