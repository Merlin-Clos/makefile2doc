pub mod model;
pub mod parser;
pub mod generator;

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

        let expected = vec![
            "## @target: build",
            "## @name: up",
            "## @desc: Start the docker container",
        ];
        
        assert_eq!(expected, search(contents));
    }

    #[test]
    fn finds_no_results() {
        let contents = "\
up:
    docker compose up
";
        let expected: Vec<&str> = Vec::new();

        assert_eq!(expected, search(contents));
    }
}
