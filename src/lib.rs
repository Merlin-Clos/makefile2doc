pub mod model;
pub mod parser;
pub mod generator;

pub fn process(content: &str) -> String {
    let doc = parser::parse(content);
    
    format!("{:#?}", doc)
}