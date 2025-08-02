use lumi_parser::Parser;
use lumi_semantic::analyze;

#[test]
fn test_type_mismatch_number_and_string() {
    let mut parser = Parser::new("let x: int -> \"hello\";");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.to_string(), "Type mismatch: expected number, found string at line 1, column 27");
    }
}

#[test]
fn test_type_mismatch_string_and_number() {
    let mut parser = Parser::new("let x: str -> 42;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(e.to_string(), "Type mismatch: expected string, found number at line 1, column 22");
    }
}

#[test]
fn test_declare_number_varialble() {
    let mut parser = Parser::new("let x: number -> 42;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_declare_string_variable() {
    let mut parser = Parser::new("let x: str -> \"hello\";");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_declare_boolean_variable() {
    let mut parser = Parser::new("let x: boolean -> true;");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_ok());
}
