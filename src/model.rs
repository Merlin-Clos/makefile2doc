#[derive(Debug, PartialEq, Clone)]
pub struct MakefileDoc {
    pub categories: Vec<Category>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Category {
    pub name: String,
    pub commands: Vec<Command>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub dependencies: Vec<String>,
    pub env: Vec<String>,
}