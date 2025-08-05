use lumi_ast::Node;

use crate::Instruction;

pub trait ControlFlowGenerator {
    fn generate_if_statement(&mut self, node: &Node);
}

pub trait ControlFlowCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> ControlFlowGenerator for T
where
    T: ControlFlowCore,
{
    fn generate_if_statement(&mut self, node: &Node) {
        if let Node::IfStatement(stmt) = node {
            self.visit_node(&stmt.expr);

            let jump_if_false_pos = self.instructions().len();
            self.instructions().push(Instruction::JumpIfFalse(0));
            self.instructions().push(Instruction::Pop); // pop condition result

            self.visit_node(&stmt.stmt);
            
            let jump_to_end_pos = self.instructions().len();
            self.instructions().push(Instruction::Jump(0));
            self.instructions().push(Instruction::Pop); // optional cleanup

            let else_start = self.instructions().len();
            self.instructions()[jump_if_false_pos] = Instruction::JumpIfFalse(else_start);
            if let Some(else_part) = &stmt.else_part {
                self.visit_node(else_part);
            }

            let end = self.instructions().len();
            self.instructions()[jump_to_end_pos] = Instruction::Jump(end);
        }
    }
}
