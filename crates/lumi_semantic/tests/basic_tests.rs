use lumi_parser::Parser;
use lumi_semantic::analyze;

#[test]
fn test_variable_declaration() {
    let mut parser = Parser::new("let x: int -> 42;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}
