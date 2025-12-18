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

            let idx = self.get_or_create_local(&var_name);

            if expr.operator == "+=" {
                // Load current value of the variable
                self.instructions().push(Instruction::LoadVar(idx));
                // Visit right-hand side (pushes its value onto stack)
                self.visit_node(&expr.right);
                // Add the two values
                self.instructions().push(Instruction::Add);
                // Store the result back
                self.instructions().push(Instruction::StoreVar(idx));
            } else if expr.operator == "-=" {
                // Load current value of the variable
                self.instructions().push(Instruction::LoadVar(idx));
                // Visit right-hand side (pushes its value onto stack)
                self.visit_node(&expr.right);
                // Add the two values
                self.instructions().push(Instruction::Sub);
                // Store the result back
                self.instructions().push(Instruction::StoreVar(idx));
            } else {
                // For simple assignment (=), just visit right and store
                self.visit_node(&expr.right);
                self.instructions().push(Instruction::StoreVar(idx));
            }

            // TODO: remove unused constants by doing a compiler pass after byte code generation.
        }
    }

    fn generate_call_expression(&mut self, node: &Node) {
        if let Node::CallExpression(expr) = node {
            self.visit_node(&expr.callee);

            for arg in &expr.arguments {
                self.visit_node(arg);
            }

            // NOTE: here we assume that the callee is always an identifier (function name)
            // In a more complete implementation, we would need to handle other cases (e.g.,
            // function expressions, member expressions, etc.)
            self.instructions()
                .push(Instruction::CallFn(expr.callee.name().to_string()));
        }
    }
}
