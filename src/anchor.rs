use std::collections::HashSet;

use crate::model::MakefileDoc;

#[derive(Debug, Clone)]
pub struct AnchorManager {
    category_ids: Vec<String>,
    command_ids: Vec<Vec<String>>,
}

impl AnchorManager {
    pub fn build(doc: &MakefileDoc) -> Self {
        let mut allocator = AnchorAllocator::new();
        let mut category_ids = Vec::with_capacity(doc.categories.len());
        let mut command_ids = Vec::with_capacity(doc.categories.len());

        for cat in &doc.categories {
            let category_id = allocator.next_id("cat", &cat.name);
            category_ids.push(category_id);

            let mut ids = Vec::with_capacity(cat.commands.len());
            for cmd in &cat.commands {
                ids.push(allocator.next_id("cmd", &cmd.name));
            }

            command_ids.push(ids);
        }

        Self {
            category_ids,
            command_ids,
        }
    }

    pub fn category_id(&self, cat_idx: usize) -> &str {
        self.category_ids[cat_idx].as_str()
    }

    pub fn command_id(&self, cat_idx: usize, cmd_idx: usize) -> &str {
        self.command_ids[cat_idx][cmd_idx].as_str()
    }
}

struct AnchorAllocator {
    used_ids: HashSet<String>,
}

impl AnchorAllocator {
    fn new() -> Self {
        Self {
            used_ids: HashSet::new(),
        }
    }

    fn next_id(&mut self, prefix: &str, raw: &str) -> String {
        let base = slugify(raw);
        let base = if base.is_empty() {
            String::from("item")
        } else {
            base
        };

        let base_id = format!("{}-{}", prefix, base);

        if self.used_ids.insert(base_id.clone()) {
            return base_id;
        }

        let mut suffix = 2;

        loop {
            let candidate = format!("{}-{}", base_id, suffix);
            if self.used_ids.insert(candidate.clone()) {
                return candidate;
            }
            suffix += 1;
        }
    }
}

pub fn slugify(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    let mut slug = String::with_capacity(trimmed.len());
    let mut last_was_dash = false;

    for c in trimmed.chars() {
        if c.is_ascii_alphanumeric() {
            slug.push(c.to_ascii_lowercase());
            last_was_dash = false;
        } else if !slug.is_empty() && !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    if slug.ends_with('-') {
        slug.pop();
    }

    slug
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{Category, Command, MakefileDoc};

    #[test]
    fn slugify_normalizes_separators() {
        assert_eq!(slugify("Setup & Initialization"), "setup-initialization");
        assert_eq!(slugify("foo_bar"), "foo-bar");
        assert_eq!(slugify("  A   B  "), "a-b");
    }

    #[test]
    fn slugify_can_be_empty() {
        assert_eq!(slugify("   "), "");
        assert_eq!(slugify("+++"), "");
    }

    #[test]
    fn manager_assigns_unique_prefixed_ids() {
        let doc = MakefileDoc {
            categories: vec![
                Category {
                    name: String::from("C++"),
                    commands: vec![Command {
                        name: String::from("build"),
                        description: String::new(),
                        dependencies: vec![],
                        env: vec![],
                    }],
                },
                Category {
                    name: String::from("C#"),
                    commands: vec![
                        Command {
                            name: String::from("build"),
                            description: String::new(),
                            dependencies: vec![],
                            env: vec![],
                        },
                        Command {
                            name: String::from("build-2"),
                            description: String::new(),
                            dependencies: vec![],
                            env: vec![],
                        },
                    ],
                },
                Category {
                    name: String::from("build"),
                    commands: vec![],
                },
            ],
        };

        let anchors = AnchorManager::build(&doc);

        assert_eq!(anchors.category_id(0), "cat-c");
        assert_eq!(anchors.category_id(1), "cat-c-2");
        assert_eq!(anchors.category_id(2), "cat-build");

        assert_eq!(anchors.command_id(0, 0), "cmd-build");
        assert_eq!(anchors.command_id(1, 0), "cmd-build-2");
        assert_eq!(anchors.command_id(1, 1), "cmd-build-2-2");
    }
}
