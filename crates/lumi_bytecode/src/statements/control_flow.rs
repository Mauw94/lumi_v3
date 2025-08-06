use lumi_ast::Node;

use crate::{instruction::Label, Instruction};

pub trait ControlFlowGenerator {
    fn generate_if_statement(&mut self, node: &Node);
}

pub trait ControlFlowCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
    fn emit_jump(&mut self, label: Label);
    fn emit_jump_if_false(&mut self, label: Label);
    fn emit(&mut self, instr: Instruction);
    fn new_label(&mut self) -> Label;
    fn patch_label(&mut self, label: Label);
}

impl<T> ControlFlowGenerator for T
where
    T: ControlFlowCore,
{
    fn generate_if_statement(&mut self, node: &Node) {
        if let Node::IfStatement(stmt) = node {
            self.visit_node(&stmt.expr);

            let else_label = self.new_label();
            let end_label = self.new_label();

            self.emit_jump_if_false(else_label);
            self.emit(Instruction::Pop);

            self.visit_node(&stmt.stmt);
            self.emit_jump(end_label);
            self.emit(Instruction::Pop);

            self.patch_label(else_label);
            if let Some(else_part) = &stmt.else_part {
                self.visit_node(else_part);
            }

            self.patch_label(end_label);
        }
    }
}
