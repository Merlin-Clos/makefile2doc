use crate::model::{Category, Command, MakefileDoc};
use lazy_regex::regex;

struct ParserContext {
    current_category: String,
    buffer_desc: String,
    buffer_deps: Vec<String>,
    buffer_envs: Vec<String>,
}

impl ParserContext {
    fn new() -> Self {
        Self {
            current_category: String::from("General"),
            buffer_desc: String::new(),
            buffer_deps: Vec::new(),
            buffer_envs: Vec::new(),
        }
    }

    fn clear_metadata(&mut self) {
        self.buffer_deps.clear();
        self.buffer_desc.clear();
        self.buffer_envs.clear();
    }
}

fn try_extract_category(line: &str) -> Option<String> {
    regex!(r"^##\s@category\s+(.*)$")
        .captures(line)
        .map(|captures| captures[1].trim().to_string())
}

fn try_extract_description(line: &str) -> Option<String> {
    regex!(r"^##\s@description\s+(.*)$")
        .captures(line)
        .map(|captures| captures[1].trim().to_string())
}

fn try_extract_depends(line: &str) -> Option<Vec<String>> {
    regex!(r"^##\s@depends\s+(.*)$")
        .captures(line)
        .map(|captures| {
            captures[1]
                .split(",")
                .map(|split| split.trim().to_string())
                .collect()
        })
}

fn try_extract_envs(line: &str) -> Option<Vec<String>> {
    regex!(r"^##\s@env\s+(.*)$").captures(line).map(|captures| {
        captures[1]
            .split(",")
            .map(|split| split.trim().to_string())
            .collect()
    })
}

fn try_extract_target(line: &str) -> Option<String> {
    regex!(r"^([a-zA-Z0-9_-]+):")
        .captures(line)
        .map(|captures| captures[1].trim().to_string())
}

pub fn parse(content: &str) -> MakefileDoc {
    let lines = match clean_content(content) {
        Ok(l) => l,
        Err(_) => return MakefileDoc { categories: vec![] },
    };

    let mut categories: Vec<Category> = Vec::new();
    let mut ctx = ParserContext::new();

    for line in lines {
        let line = line.trim();
        
        if let Some(cat) = try_extract_category(line) {
            ctx.current_category = cat;
            continue;
        }

        if let Some(desc) = try_extract_description(line) {
            ctx.buffer_desc.push_str(&desc);
            continue;
        }

        if let Some(deps) = try_extract_depends(line) {
            ctx.buffer_deps.extend(deps);
            continue;
        }

        if let Some(envs) = try_extract_envs(line) {
            ctx.buffer_envs.extend(envs);
            continue;
        }

        if let Some(target_name) = try_extract_target(line) {
            let command = Command {
                name: target_name,
                description: ctx.buffer_desc.clone(),
                dependencies: ctx.buffer_deps.clone(),
                env: ctx.buffer_envs.clone(),
            };

            if let Some(cat) = categories
                .iter_mut()
                .find(|c| c.name == ctx.current_category)
            {
                cat.commands.push(command);
            } else {
                categories.push(Category {
                    name: ctx.current_category.clone(),
                    commands: vec![command],
                })
            }

            ctx.clear_metadata();
        }
    }

    MakefileDoc { categories }
}

pub fn clean_content(content: &str) -> Result<Vec<&str>, &'static str> {
    if content.trim().is_empty() {
        return Err("The Makefile is empty");
    }

    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();

    Ok(lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_content_return_error_if_empty() {
        let input = "";
        let result = clean_content(input);

        assert_eq!(result, Err("The Makefile is empty"))
    }

    #[test]
    fn clean_content_return_error_if_only_spaces() {
        let input = "     ";
        let result = clean_content(input);

        assert_eq!(result, Err("The Makefile is empty"))
    }

    #[test]
    fn clean_content_return_error_if_line_break() {
        let input = "\n";
        let result = clean_content(input);

        assert_eq!(result, Err("The Makefile is empty"))
    }

    #[test]
    fn clean_content_return_error_if_indent() {
        let input = "\t";
        let result = clean_content(input);

        assert_eq!(result, Err("The Makefile is empty"))
    }

    #[test]
    fn clean_content_return_error_if_spaces_indent_line_break() {
        let input = " \t\n ";
        let result = clean_content(input);

        assert_eq!(result, Err("The Makefile is empty"))
    }
    
    #[test]
    fn test_parse_makefile() {
        let content = r#"
            ## @category Deployment
            ## @description Deploy to Production \n 1. Build frontend assets \n 2. Optimize Laravel cache \n 3. Run migrations force
            
            ## @depends build-front, migrate
            
            ## @env APP_KEY, SSH_USER
            
            deploy:
            "#;
        
        let doc = parse(content);
        assert_eq!(doc.categories.len(), 1);
        
        let cat = &doc.categories[0];
        assert_eq!(cat.name, "Deployment");
        assert_eq!(cat.commands.len(), 1);
        
        let cmd = &cat.commands[0];
        assert_eq!(cmd.name, "deploy");
        assert_eq!(cmd.description, r#"Deploy to Production \n 1. Build frontend assets \n 2. Optimize Laravel cache \n 3. Run migrations force"#);
        assert_eq!(cmd.dependencies, vec!["build-front", "migrate"]);
        assert_eq!(cmd.env, vec!["APP_KEY", "SSH_USER"]);
    }
}
