use crate::{generator::generate, parser::parse};

pub mod anchor;
pub mod generator;
pub mod model;
pub mod parser;

pub fn process(content: &str) -> String {
    let doc = parse(content);
    generate(&doc)
}
