use crate::Node;

/// Visitor trait for AST traversal
pub trait Visitor {
    type Output;

    fn visit_node(&mut self, node: &Node) -> Self::Output {
        match node {
            Node::Program(program) => self.visit_program(program),
            Node::VariableDeclaration(decl) => self.visit_variable_declaration(decl),
            Node::BinaryExpression(expr) => self.visit_binary_expression(expr),
            Node::String(s) => self.visit_string(s),
            Node::Boolean(b) => self.visit_boolean(*b),
            Node::Number(n) => self.visit_number(*n),
            Node::Null => self.visit_null(),
            Node::Identifier(i) => self.visit_identifier(i),
            Node::Undefined => self.visit_undefined(),
            Node::UnaryExpression(_) => todo!(),
            Node::PrintStatement(_) => todo!(),
            Node::ArrayLiteral(_) => todo!(),
            Node::ExpressionStatement(_) => todo!(),
            Node::AssignmentExpression(_) => todo!(),
            Node::LogicalExpression(_) => todo!(),
            Node::IfStatement(if_statement) => todo!(),
            Node::BlockStatement(block_statement) => todo!(),
            Node::FunctionDeclaration(function_declaration) => todo!(),
            Node::CallExpression(call_expression) => todo!(),
            Node::ForStatement(for_statement) => todo!(),
        }
    }

    fn visit_program(&mut self, _: &crate::Program) -> Self::Output {
        unimplemented!()
    }
    fn visit_variable_declaration(&mut self, _: &crate::VariableDeclaration) -> Self::Output {
        unimplemented!()
    }
    fn visit_binary_expression(&mut self, _: &crate::BinaryExpression) -> Self::Output {
        unimplemented!()
    }
    fn visit_string(&mut self, _: &str) -> Self::Output {
        unimplemented!()
    }
    fn visit_identifier(&mut self, _: &str) -> Self::Output {
        unimplemented!()
    }
    fn visit_number(&mut self, _: f64) -> Self::Output {
        unimplemented!()
    }
    fn visit_boolean(&mut self, _: bool) -> Self::Output {
        unimplemented!()
    }
    fn visit_null(&mut self) -> Self::Output {
        unimplemented!()
    }
    fn visit_undefined(&mut self) -> Self::Output {
        unimplemented!()
    }
}

/// Simple visitor that counts nodes
pub struct NodeCounter {
    pub count: usize,
}

impl NodeCounter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

impl Visitor for NodeCounter {
    type Output = ();

    fn visit_node(&mut self, node: &Node) -> Self::Output {
        self.count += 1;
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
                }
            }
            Node::BinaryExpression(expr) => {
                self.visit_node(&expr.left);
                self.visit_node(&expr.right);
            }
            _ => {}
        }
    }
}
