use lumi_ast::Node;
use lumi_parser::Parser;

#[test]
fn test_empty_source() {
    let mut parser = Parser::new("");
    let result = parser.parse();
    assert!(result.is_ok());
}

#[test]
fn test_number() {
    let mut parser = Parser::new("42");
    let result = parser.parse();
    assert!(result.is_ok());

    if let Ok(Node::Program(program)) = result {
        assert_eq!(program.body.len(), 1);
        if let Node::ExpressionStatement(stmt) = &program.body[0] {
            if let Node::Number(num) = &*stmt.expression {
                assert_eq!(*num, 42.0);
            } else {
                panic!("Expected a number node");
            }
        } else {
            panic!("Expected an expression statement");
        }
    } else {
        panic!("Expected a program node");
    }
}

#[test]
fn test_string() {
    let mut parser = Parser::new("\"hello world\"");
    let result = parser.parse();
    assert!(result.is_ok());

    if let Ok(Node::Program(program)) = result {
        assert_eq!(program.body.len(), 1);
        if let Node::ExpressionStatement(stmt) = &program.body[0] {
            if let Node::String(s) = &*stmt.expression {
                assert_eq!(*s, "hello world".to_string());
            } else {
                panic!("Expected a string node");
            }
        } else {
            panic!("Expected an expression statement");
        }
    } else {
        panic!("Expected a program node");
    }
}

#[test]
fn test_boolean() {
    let mut parser = Parser::new("true");
    let result = parser.parse();
    assert!(result.is_ok());

    if let Ok(Node::Program(program)) = result {
        assert_eq!(program.body.len(), 1);
        if let Node::ExpressionStatement(stmt) = &program.body[0] {
            if let Node::Boolean(b) = &*stmt.expression {
                assert_eq!(*b, true);
            } else {
                panic!("Expected a boolean node");
            }
        } else {
            panic!("Expected an expression statement");
        }
    } else {
        panic!("Expected a program node");
    }
}

#[test]
fn test_identifier() {
    let mut parser = Parser::new("x");
    let result = parser.parse();
    assert!(result.is_ok());

    if let Ok(Node::Program(program)) = result {
        assert_eq!(program.body.len(), 1);
        if let Node::ExpressionStatement(stmt) = &program.body[0] {
            if let Node::Identifier(id) = &*stmt.expression {
                assert_eq!(id, "x");
            } else {
                panic!("Expected Identifier node");
            }
        } else {
            panic!("Expected ExpressionStatement");
        }
    } else {
        panic!("Expected Program node");
    }
}

#[test]
fn test_binary_expression() {
    let mut parser = Parser::new("1 + 2");
    let result = parser.parse();
    assert!(result.is_ok());

    if let Ok(Node::Program(program)) = result {
        assert_eq!(program.body.len(), 1);
        if let Node::ExpressionStatement(stmt) = &program.body[0] {
            if let Node::BinaryExpression(expr) = &*stmt.expression {
                assert_eq!(expr.operator, "+");
                if let Node::Number(left) = &*expr.left {
                    assert_eq!(*left, 1.0);
                } else {
                    panic!("Expected Number on left");
                }
                if let Node::Number(right) = &*expr.right {
                    assert_eq!(*right, 2.0);
                } else {
                    panic!("Expected Number on right");
                }
            } else {
                panic!("Expected BinaryExpression");
            }
        } else {
            panic!("Expected ExpressionStatement");
        }
    } else {
        panic!("Expected Program node");
    }
}
