use lumi_ast::Node;
use lumi_parser::Parser;

#[test]
fn test_fn_statement() {
    let mut parser = Parser::new(
        r#"
        fn test(x, y) {
            x + y;
        }
    "#,
    );

    let result = parser.parse();

    if let Ok(Node::Program(program)) = result {
        if let Node::FunctionDeclaration(fn_decl) = &program.body[0] {
            assert_eq!(fn_decl.params.len(), 2);
            if let Node::BlockStatement(blck) = &*fn_decl.body {
                if let Node::BinaryExpression(expr) = &blck.body[0] {
                    assert_eq!(expr.operator, "+");
                }
            } else {
                panic!("Expected BlockStatement node");
            }
        } else {
            panic!("Expected FunctionDeclaration node");
        }
    }
}
