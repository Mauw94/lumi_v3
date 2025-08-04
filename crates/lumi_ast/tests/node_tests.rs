use lumi_ast::{
    create_binary_expression, create_block_statement, create_identifier, create_if_statement,
    create_multiple_variable_declarations, create_number, create_string,
    create_variable_declaration, Node, Position, Span,
};

#[test]
fn test_position_creation() {
    let pos = Position::new(1, 5);
    assert_eq!(pos.line, 1);
    assert_eq!(pos.column, 5);
}

#[test]
fn test_span_creation() {
    let start = Position::new(1, 1);
    let end = Position::new(1, 10);
    let span = Span::new(start, end);

    assert_eq!(span.start.line, 1);
    assert_eq!(span.start.column, 1);
    assert_eq!(span.end.line, 1);
    assert_eq!(span.end.column, 10);
}

#[test]
fn test_literal_nodes() {
    // Identifier
    let id = create_identifier("x");
    assert!(matches!(id, Node::Identifier(name) if name == "x"));

    // Number
    let id = create_number(42.0);
    assert!(matches!(id, Node::Number(n) if n == 42.0));

    // String
    let id = create_string("hello world");
    assert!(matches!(id, Node::String(s) if s == "hello world"));
}

#[test]
fn test_expression_nodes() {
    let left = create_identifier("a");
    let right = create_identifier("b");
    let bin_expr = create_binary_expression(left, "+", right);

    assert!(matches!(bin_expr, Node::BinaryExpression(expr) if expr.operator == "+"));
}

#[test]
fn test_declaration_nodes() {
    let declaration = create_variable_declaration("let", "x", None, Some(create_number(42.0)));

    assert!(matches!(declaration, Node::VariableDeclaration(decl) if decl.kind == "let"));

    let multiple_declarations = create_multiple_variable_declarations(
        "let",
        "x",
        "y",
        Some("int"),
        Some("str"),
        Some(create_number(42.0)),
        Some(create_number(42.42)),
    );

    assert!(
        matches!(multiple_declarations, Node::VariableDeclaration(decl) if decl.declarations.len() == 2)
    );
}

#[test]
fn test_if_statement() {
    let expr = create_identifier("condition");
    let params = create_block_statement(vec![]);
    let if_stmt = create_if_statement(expr, params, None);

    assert!(matches!(if_stmt, Node::IfStatement(_)));
}
