use makefile2doc;
use std::fs;

#[test]
fn test_generate_documentation() {
    let input = fs::read_to_string("tests/fixtures/Makefile").unwrap();
    let expected = fs::read_to_string("tests/fixtures/expected.md").unwrap();

    assert_eq!(input, expected);
}
