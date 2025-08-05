use lumi_ast::Node;

use crate::Instruction;

pub trait ControlFlowGenerator {
    fn generate_if_statement(&mut self, node: &Node);
}

pub trait ControlFlowCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);

    fn emit_jump_if_false(&mut self) -> usize {
        let pos = self.instructions().len();
        self.instructions().push(Instruction::JumpIfFalse(0));
        pos
    }

    fn emit_jump(&mut self) -> usize {
        let pos = self.instructions().len();
        self.instructions().push(Instruction::Jump(0));
        pos
    }

    fn patch_jump(&mut self, pos: usize, target: usize) {
        let instr = self.instructions();

        match &instr[pos] {
            Instruction::Jump(_) => {
                instr[pos] = Instruction::Jump(target);
            }
            Instruction::JumpIfFalse(_) => instr[pos] = Instruction::JumpIfFalse(target),
            _ => panic!("Trying to patch a non-jump instruction at position {pos}"),
        }
    }
}

impl<T> ControlFlowGenerator for T
where
    T: ControlFlowCore,
{
    fn generate_if_statement(&mut self, node: &Node) {
        if let Node::IfStatement(stmt) = node {
            self.visit_node(&stmt.expr);

            let jump_if_false_pos = self.emit_jump_if_false();
            self.visit_node(&stmt.stmt);

            let jump_to_end_pos = self.emit_jump();

            let else_start = self.instructions().len();
            self.patch_jump(jump_if_false_pos, else_start);

            if let Some(else_part) = &stmt.else_part {
                self.visit_node(else_part);
            }

            let end = self.instructions().len();
            self.patch_jump(jump_to_end_pos, end);
        }
    }
}
