use lumi_ast::Node;

use crate::{instruction::Label, scope::local_vars::ScopeManager, Instruction};

pub trait ControlFlowGenerator {
    fn generate_if_statement(&mut self, node: &Node);
    fn generate_for_statement(&mut self, node: &Node);
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
    T: ControlFlowCore + ScopeManager,
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

    fn generate_for_statement(&mut self, node: &Node) {
        if let Node::ForStatement(stmt) = node {
            let var_idx = match &*stmt.iterator {
                Node::Identifier(id) => self.get_or_create_local(&id.clone()),
                _ => unreachable!("Iterator must be an identifier"),
            };

            self.visit_node(&stmt.start);
            self.emit(Instruction::StoreVar(var_idx));

            let end_idx = self.new_temp_local();
            self.visit_node(&stmt.end);
            self.emit(Instruction::StoreVar(end_idx));

            let step_idx = self.new_temp_local();
            if let Some(step) = &stmt.step {
                self.visit_node(step);
            } else {
                self.emit(Instruction::PushConst(1));
            }
            self.emit(Instruction::StoreVar(step_idx));

            let start_label = self.new_label();
            let end_label = self.new_label();

            self.patch_label(start_label);

            self.emit(Instruction::LoadVar(var_idx));
            self.emit(Instruction::LoadVar(end_idx));
            self.emit(Instruction::Leq);

            self.emit_jump_if_false(end_label);

            self.visit_node(&stmt.body);
            self.emit(Instruction::Pop);

            self.emit(Instruction::LoadVar(var_idx));
            self.emit(Instruction::LoadVar(step_idx));
            self.emit(Instruction::Add);
            self.emit(Instruction::StoreVar(var_idx));

            self.emit_jump(start_label);
            self.patch_label(end_label);
        }
    }
}
