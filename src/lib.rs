pub mod generator;
pub mod model;
pub mod parser;

pub fn process(content: &str) -> String {
    let doc = parser::parse(content);

    format!("{:#?}", doc)
}
