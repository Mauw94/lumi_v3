use lumi_ast::Node;
use lumi_parser::Parser;

#[test]
fn test_fn_call_expression() {
    let mut parser = Parser::new(
        r#"
        fn test(x, y) {
            x + y;
        }

        test(2, 3);
    "#,
    );

    let result = parser.parse();

    if let Ok(Node::Program(program)) = result {
        if let Node::ExpressionStatement(expr) = &program.body[1] {
            if let Node::CallExpression(call) = &*expr.expression {
                if let Node::Identifier(fn_name) = &*call.callee {
                    assert_eq!(*fn_name, "test".to_string());
                }
            } else {
                panic!("Expected CallExpression node");
            }
        } else {
            panic!("Expected ExpressionStatement node");
        }
    }
}
