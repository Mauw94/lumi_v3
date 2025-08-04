use lumi_parser::Parser;
use lumi_semantic::analyze;

#[test]
fn test_valid_if_statement() {
    let mut parser = Parser::new("if (true) { true; }");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}