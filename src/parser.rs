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
            if categories.iter().any(|c| c.name == cat) {
                eprint!(
                    "Warning: The category '{}' is defined multiple times, you should consider combining them.",
                    cat
                )
            }
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
            if ctx.buffer_desc.is_empty() {
                ctx.clear_metadata();
                continue;
            }

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
    fn clean_content_edge_cases() {
        let bad_inputs = vec![
            "",         // Empty
            "   ",      // Spaces
            "\n",       // Line break
            "\t",       // Tabulation
            " \t\n ",   // Space + Tab + LB
            "\r\n",     // Windows style
            "   \n   ", // Multiple empty lines
        ];

        for (i, input) in bad_inputs.iter().enumerate() {
            let result = clean_content(input);
            assert_eq!(
                result,
                Err("The Makefile is empty"),
                "Failed for input #{}: {:?}",
                i,
                input
            );
        }
    }

    #[test]
    fn parse_single_cat_makefile_with_line_break() {
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
        assert_eq!(
            cmd.description,
            r#"Deploy to Production \n 1. Build frontend assets \n 2. Optimize Laravel cache \n 3. Run migrations force"#
        );
        assert_eq!(cmd.dependencies, vec!["build-front", "migrate"]);
        assert_eq!(cmd.env, vec!["APP_KEY", "SSH_USER"]);
    }

    #[test]
    fn parse_multiple_cat_makefile() {
        let content = r#"
            ## @category Deployment
            ## @description Deploy to Production \n 1. Build frontend assets \n 2. Optimize Laravel cache \n 3. Run migrations force
            ## @depends build-front, migrate
            ## @env APP_KEY, SSH_USER
            deploy:
            
            ## @category Database
            ## @description Reset the DB and run seeds (Test data) \n Warning: This deletes all data!
            ## @depends migrate
            ## @env SEED_CLASS
            seed:
            "#;

        let doc = parse(content);
        assert_eq!(doc.categories.len(), 2);

        let deployment = &doc.categories[0];
        assert_eq!(deployment.name, "Deployment");
        let database = &doc.categories[1];
        assert_eq!(database.name, "Database");

        let deploy = &deployment.commands[0];
        assert_eq!(deploy.name, "deploy");
        assert_eq!(
            deploy.description,
            r#"Deploy to Production \n 1. Build frontend assets \n 2. Optimize Laravel cache \n 3. Run migrations force"#
        );
        assert_eq!(deploy.dependencies, vec!["build-front", "migrate"]);
        assert_eq!(deploy.env, vec!["APP_KEY", "SSH_USER"]);

        let lint = &database.commands[0];
        assert_eq!(lint.name, "seed");
        assert_eq!(
            lint.description,
            r#"Reset the DB and run seeds (Test data) \n Warning: This deletes all data!"#
        );
        assert_eq!(lint.dependencies, vec!["migrate"]);
        assert_eq!(lint.env, vec!["SEED_CLASS"]);
    }

    #[test]
    fn parse_single_general_cat_makefile() {
        let content = r#"
            ## @description Deploy to Production \n 1. Build frontend assets \n 2. Optimize Laravel cache \n 3. Run migrations force
            ## @depends build-front, migrate
            ## @env APP_KEY, SSH_USER
            deploy:
            "#;

        let doc = parse(content);
        assert_eq!(doc.categories.len(), 1);

        let general = &doc.categories[0];
        assert_eq!(general.name, "General");

        let deploy = &general.commands[0];
        assert_eq!(deploy.name, "deploy");
        assert_eq!(
            deploy.description,
            r#"Deploy to Production \n 1. Build frontend assets \n 2. Optimize Laravel cache \n 3. Run migrations force"#
        );
        assert_eq!(deploy.dependencies, vec!["build-front", "migrate"]);
        assert_eq!(deploy.env, vec!["APP_KEY", "SSH_USER"]);
    }

    #[test]
    fn parse_single_and_general_cat_makefile() {
        let content = r#"
            ## @description Deploy to Production \n 1. Build frontend assets \n 2. Optimize Laravel cache \n 3. Run migrations force
            ## @depends build-front, migrate
            ## @env APP_KEY, SSH_USER
            deploy:
            
            ## @category Database
            ## @description Reset the DB and run seeds (Test data) \n Warning: This deletes all data!
            ## @depends migrate
            ## @env SEED_CLASS
            seed:
            "#;

        let doc = parse(content);
        assert_eq!(doc.categories.len(), 2);

        let general = &doc.categories[0];
        assert_eq!(general.name, "General");
        let database = &doc.categories[1];
        assert_eq!(database.name, "Database");

        let deploy = &general.commands[0];
        assert_eq!(deploy.name, "deploy");
        assert_eq!(
            deploy.description,
            r#"Deploy to Production \n 1. Build frontend assets \n 2. Optimize Laravel cache \n 3. Run migrations force"#
        );
        assert_eq!(deploy.dependencies, vec!["build-front", "migrate"]);
        assert_eq!(deploy.env, vec!["APP_KEY", "SSH_USER"]);

        let seed = &database.commands[0];
        assert_eq!(seed.name, "seed");
        assert_eq!(
            seed.description,
            r#"Reset the DB and run seeds (Test data) \n Warning: This deletes all data!"#
        );
        assert_eq!(seed.dependencies, vec!["migrate"]);
        assert_eq!(seed.env, vec!["SEED_CLASS"]);
    }

    #[test]
    fn ignores_targets_without_description() {
        let content = r#"
                ## @description Public Command
                public:
                
                # No @description here
                private:
                
                ## @description Another Public
                public2:
            "#;

        let doc = parse(content);
        let cat = &doc.categories[0];

        assert_eq!(
            cat.commands.len(),
            2,
            "Should have ignored the private command"
        );
        assert_eq!(cat.commands[0].name, "public");
        assert_eq!(cat.commands[1].name, "public2");
    }

    #[test]
    fn optional_metadata_are_empty_by_default() {
        let content = r#"
                ## @description Simple command
                simple:
            "#;

        let doc = parse(content);
        let cmd = &doc.categories[0].commands[0];

        assert_eq!(cmd.name, "simple");
        assert_eq!(cmd.description, "Simple command");
        assert!(cmd.dependencies.is_empty(), "Dependencies should be empty");
        assert!(cmd.env.is_empty(), "Env should be empty");
    }

    #[test]
    fn merges_split_categories() {
        let content = r#"
                ## @category Database
                ## @description Migrations
                migrate:
                
                ## @category Frontend
                ## @description Build
                build:
                
                ## @category Database
                ## @description Seed
                seed:
            "#;

        let doc = parse(content);

        assert_eq!(doc.categories.len(), 2);

        let db_cat = doc
            .categories
            .iter()
            .find(|c| c.name == "Database")
            .unwrap();
        assert_eq!(db_cat.commands.len(), 2);
        assert_eq!(db_cat.commands[0].name, "migrate");
        assert_eq!(db_cat.commands[1].name, "seed");
    }
}
