use lumi_ast::{
    create_binary_expression, create_identifier, create_program, create_variable_declaration, Node,
    Visitor,
};

#[derive(Debug)]
struct IdentifierCollector {
    identifiers: Vec<String>,
}

impl IdentifierCollector {
    fn new() -> Self {
        Self {
            identifiers: Vec::new(),
        }
    }
}

impl Visitor for IdentifierCollector {
    type Output = ();

    fn visit_node(&mut self, node: &Node) -> Self::Output {
        match node {
            Node::Program(program) => {
                for node in &program.body {
                    self.visit_node(node);
                }
            }
            Node::VariableDeclaration(decl) => {
                for var_decl in &decl.declarations {
                    self.visit_node(&var_decl.var_name);
                    if let Some(init) = &var_decl.init {
                        self.visit_node(init);
                    }
                    if let Some(var_type) = &var_decl.var_type {
                        self.visit_node(var_type);
                    }
                }
            }
            Node::BinaryExpression(expr) => {
                self.visit_node(&expr.left);
                self.visit_node(&expr.right);
            }
            Node::Identifier(id) => self.visit_identifier(id),
            _ => {}
        }
    }

    fn visit_identifier(&mut self, id: &str) -> Self::Output {
        self.identifiers.push(id.to_string());
    }
}

#[test]
fn test_identifier_collector_visitor() {
    let mut collector = IdentifierCollector::new();

    let ast = create_program(vec![
        create_variable_declaration("let", "x", Some("str"), Some(create_identifier("y"))),
        create_binary_expression(create_identifier("a"), "+", create_identifier("b")),
    ]);

    collector.visit_node(&ast);

    assert_eq!(collector.identifiers.len(), 5);
    assert!(collector.identifiers.contains(&"str".to_string()));
    assert!(collector.identifiers.contains(&"x".to_string()));
    assert!(collector.identifiers.contains(&"y".to_string()));
    assert!(collector.identifiers.contains(&"a".to_string()));
    assert!(collector.identifiers.contains(&"b".to_string()));
}
