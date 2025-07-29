use crate::{BinaryExpression, Node, Program, VariableDeclaration, VariableDeclarator};

// Helpers
pub fn create_program(body: Vec<Node>) -> Node {
    Node::Program(Program { body, span: None })
}

pub fn create_identifier(name: &str) -> Node {
    Node::Identifier(name.to_string())
}

pub fn create_number(n: f64) -> Node {
    Node::Number(n)
}

pub fn create_string(s: &str) -> Node {
    Node::String(s.to_string())
}

pub fn create_variable_declaration(kind: &str, name: &str, init: Option<Node>) -> Node {
    Node::VariableDeclaration(VariableDeclaration {
        kind: kind.to_string(),
        declarations: vec![VariableDeclarator {
            id: Box::new(create_identifier(name)),
            init: init.map(Box::new),
            span: None,
        }],
        span: None,
    })
}

pub fn create_multiple_variable_declarations(
    kind: &str,
    name1: &str,
    name2: &str,
    init1: Option<Node>,
    init2: Option<Node>,
) -> Node {
    Node::VariableDeclaration(VariableDeclaration {
        kind: kind.to_string(),
        declarations: vec![
            VariableDeclarator {
                id: Box::new(create_identifier(name1)),
                init: init1.map(Box::new),
                span: None,
            },
            VariableDeclarator {
                id: Box::new(create_identifier(name2)),
                init: init2.map(Box::new),
                span: None,
            },
        ],
        span: None,
    })
}

pub fn create_binary_expression(left: Node, operator: &str, right: Node) -> Node {
    Node::BinaryExpression(BinaryExpression {
        left: Box::new(left),
        operator: operator.to_string(),
        right: Box::new(right),
        span: None,
    })
}
