use makefile2doc;
use pretty_assertions::assert_eq;
use std::fs;

#[test]
fn test_generate_documentation() {
    let input = fs::read_to_string("tests/fixtures/Makefile").unwrap();
    let expected = fs::read_to_string("tests/fixtures/expected.md").unwrap();

    let actual = makefile2doc::process(&input);

    assert_eq!(actual.trim(), expected.trim());
}
