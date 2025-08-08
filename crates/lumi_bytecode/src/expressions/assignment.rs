use lumi_ast::Node;

use crate::{scope::local_vars::ScopeManager, Instruction};

pub trait AssignmentGenerator {
    fn generate_assignment_expression(&mut self, node: &Node);
    fn generate_call_expression(&mut self, node: &Node);
}

pub trait AssignmentCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> AssignmentGenerator for T
where
    T: AssignmentCore + ScopeManager,
{
    fn generate_assignment_expression(&mut self, node: &Node) {
        if let Node::AssignmentExpression(expr) = node {
            let var_name = match &*expr.left {
                Node::Identifier(id) => id.clone(),
                _ => unreachable!(), // TODO: should also give an informative error
            };

            // TODO: remove unused constants by doing a compiler pass after byte code generation.
            self.visit_node(&expr.right);

            let idx = self.get_or_create_local(&var_name);
            self.instructions().push(Instruction::StoreVar(idx));
        }
    }

    fn generate_call_expression(&mut self, node: &Node) {
        if let Node::CallExpression(expr) = node {
            self.visit_node(&expr.callee);

            for arg in &expr.arguments {
                self.visit_node(arg);
            }

            self.instructions()
                .push(Instruction::Call(expr.arguments.len()));
        }
    }
}
