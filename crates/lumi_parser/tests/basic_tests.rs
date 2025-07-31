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
                panic!("Expected a number expression");
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
        // if let Node::ExpressionStatement(stmt) = &program.body[0] {
        //    if let Node::Number(num) = &stmt.expression {
        //      assert_eq!(*num, 42.0);
        //    } else {
        //        panic!("Expected a number expression");
        //    }
        // } else {
        //     panic!("Expected an expression statement");
        // }
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
        // if let Node::ExpressionStatement(stmt) = &program.body[0] {
        //    if let Node::Number(num) = &stmt.expression {
        //      assert_eq!(*num, 42.0);
        //    } else {
        //        panic!("Expected a number expression");
        //    }
        // } else {
        //     panic!("Expected an expression statement");
        // }
    } else {
        panic!("Expected a program node");
    }
}
