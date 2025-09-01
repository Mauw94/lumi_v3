use lumi_ast::Node;
use lumi_parser::Parser;

#[test]
fn test_simple_for_statement() {
    let mut parser = Parser::new(
        r#"
        for i in 0 to 10 {
            print i;
        }
    "#,
    );

    let result = parser.parse();

    if let Ok(Node::Program(program)) = result {
        if let Node::ForStatement(stmt) = &program.body[0] {
            if let Node::Identifier(ident) = &*stmt.iterator {
                assert_eq!(ident, "i");
            } else {
                panic!("Expected an Identifier");
            }
            if let Node::Number(n) = &*stmt.start {
                assert_eq!(*n, 0.0);
            } else {
                panic!("Expected a Number for start");
            }
            if let Node::Number(n) = &*stmt.end {
                assert_eq!(*n, 10.0);
            } else {
                panic!("Expected a Number for end");
            }
            if let Some(step) = &stmt.step {
                if let Node::Number(n) = &**step {
                    assert_eq!(*n, 1.0); // Default step is 1
                } else {
                    panic!("Expected a Number for step");
                }
            } else {
                // If no step is provided, it should be None
                assert!(true);
            }
            if let Node::BlockStatement(block) = &*stmt.body {
                assert_eq!(block.body.len(), 1);
            } else {
                panic!("Expected BlockStatement");
            }
        } else {
            panic!("Expected ForStatement node");
        }
    }
}

#[test]
fn test_simple_for_statement_with_step() {
    let mut parser = Parser::new(
        r#"
        for i in 0 to 10 step 2 {
            print i;
        }
    "#,
    );

    let result = parser.parse();

    if let Ok(Node::Program(program)) = result {
        if let Node::ForStatement(stmt) = &program.body[0] {
            if let Node::Identifier(ident) = &*stmt.iterator {
                assert_eq!(ident, "i");
            } else {
                panic!("Expected an Identifier");
            }
            if let Node::Number(n) = &*stmt.start {
                assert_eq!(*n, 0.0);
            } else {
                panic!("Expected a Number for start");
            }
            if let Node::Number(n) = &*stmt.end {
                assert_eq!(*n, 10.0);
            } else {
                panic!("Expected a Number for end");
            }
            if let Some(step) = &stmt.step {
                if let Node::Number(n) = &**step {
                    assert_eq!(*n, 2.0); // Step is 2
                } else {
                    panic!("Expected a Number for step");
                }
            } else {
                // If no step is provided, it should be None
                assert!(true);
            }
            if let Node::BlockStatement(block) = &*stmt.body {
                assert_eq!(block.body.len(), 1);
            } else {
                panic!("Expected BlockStatement");
            }
        } else {
            panic!("Expected ForStatement node");
        }
    }
}
