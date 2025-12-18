use lumi_parser::Parser;
use lumi_semantic::analyze;

#[test]
fn test_invalid_operation_assignment_expression_plus_assign() {
    let mut parser = Parser::new("let x: int -> 5; x += \"test\"");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(
            e.to_string(),
            "Invalid operation '+=' on type 'number' at line 1, column 36"
        );
    }
}

#[test]
fn test_invalid_operation_assignment_expression_minus_assign() {
    let mut parser = Parser::new("let x: int -> 5; x -= \"test\"");
    let ast = parser.parse().unwrap();
    let result = analyze(&ast);
    assert!(result.is_err());
    if let Err(e) = result {
        assert_eq!(
            e.to_string(),
            "Invalid operation '-=' on type 'number' at line 1, column 36"
        );
    }
}

// TODO: doesn't throw an error, lexing stops at +* and does not return a token.
// #[test]
// fn test_invalid_operator_assignment_expression() {
//     let mut parser = Parser::new("let x: int -> 5; x +* 2");
//     let ast = parser.parse().unwrap();
//     let result = analyze(&ast);
//     assert!(result.is_err());
//     if let Err(e) = result {
//         assert_eq!(
//             e.to_string(),
//             "Unsupported operator found '$' at line 1, column 36"
//         );
//     }
// }
