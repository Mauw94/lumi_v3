use lumi_ast::Node;
use lumi_parser::Parser;

#[test]
fn test_if_statement_bool_node() {
    let mut parser = Parser::new(
        r#"
        if (true) {
            x = 1;
        }
    "#,
    );

    let result = parser.parse();

    if let Ok(Node::Program(program)) = result {
        if let Node::IfStatement(stmt) = &program.body[0] {
            if let Node::Boolean(b) = &*stmt.expr {
                assert_eq!(*b, true);
            } else {
                panic!("Expected a Boolean");
            }
            if let Node::BlockStatement(block) = &*stmt.stmt {
                assert_eq!(block.body.len(), 1);
            } else {
                panic!("Expected BlockStatement");
            }
        } else {
            panic!("Expected IfStatement node");
        }
    }
}

#[test]
fn test_if_statement() {
    let mut parser = Parser::new(
        r#"
        let x -> 42;
        if (x > 30) {
            x;
        }
    "#,
    );

    let result = parser.parse();

    if let Ok(Node::Program(program)) = result {
        if let Node::IfStatement(stmt) = &program.body[1] {
            if let Node::BinaryExpression(b) = &*stmt.expr {
                assert_eq!(b.operator, ">");
            } else {
                panic!("Expected a Boolean");
            }
            if let Node::BlockStatement(block) = &*stmt.stmt {
                assert_eq!(block.body.len(), 1);
            } else {
                panic!("Expected BlockStatement");
            }
        } else {
            panic!("Expected IfStatement node");
        }
    }
}

#[test]
fn test_if_statement_with_print() {
    let mut parser = Parser::new(
        r#"
        let x -> 42;
        if (x > 30) {
            print "yes";
        }
    "#,
    );

    let result = parser.parse();
    if let Ok(Node::Program(program)) = result {
        if let Node::IfStatement(stmt) = &program.body[1] {
            if let Node::BinaryExpression(b) = &*stmt.expr {
                assert_eq!(b.operator, ">");
            } else {
                panic!("Expected a Boolean");
            }
            if let Node::BlockStatement(block) = &*stmt.stmt {
                if let Node::PrintStatement(prnt) = &block.body[0] {
                    if let Node::String(s) = &*prnt.argument {
                        assert_eq!(*s, "yes".to_string());
                    }
                }
                assert_eq!(block.body.len(), 1);
            } else {
                panic!("Expected BlockStatement");
            }
        } else {
            panic!("Expected IfStatement node");
        }
    }
}
