use std::collections::HashMap;

use lumi_ast::Node;

use crate::{
    expressions::{ArithmeticCore, ArithmeticGenerator, AssignmentCore, AssignmentGenerator},
    instruction::{Constant, ConstantPool, Instruction, Label, PendingJump},
    scope::local_vars::{ScopeCore, ScopeManager},
    statements::{
        control_flow::{ControlFlowCore, ControlFlowGenerator},
        function::{FunctionCore, FunctionGenerator},
        variable::{VariableCore, VariableGenerator},
    },
};

pub struct Bytecode {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<Constant>,
}

/// Bytecode generator for Lumi engine
pub struct BytecodeGenerator {
    pub constants: ConstantPool,
    pub instructions: Vec<Instruction>,
    pub label_positions: HashMap<Label, usize>,
    pub unpatched_jumps: HashMap<Label, Vec<PendingJump>>,
    pub symbol_table: HashMap<String, usize>, // Maps variable names to their indices
    pub next_label_id: usize,                 // Index for the next variable to be added
}

impl BytecodeGenerator {
    pub fn new() -> Self {
        BytecodeGenerator {
            constants: ConstantPool::default(),
            instructions: Vec::new(),
            label_positions: HashMap::new(),
            unpatched_jumps: HashMap::new(),
            symbol_table: HashMap::new(),
            next_label_id: 0,
        }
    }

    pub fn generate(&mut self, node: &Node) -> Bytecode {
        self.visit_node(node);
        Bytecode {
            instructions: self.instructions.clone(),
            constants: self.constants.values.clone(),
        }
    }

    fn visit_node(&mut self, node: &Node) {
        match node {
            Node::Program(program) => {
                for stmt in &program.body {
                    self.visit_node(stmt);
                }
            }
            Node::VariableDeclaration(_decl) => {
                <Self as VariableGenerator>::generate_variable_declaration(self, node);
            }
            Node::FunctionDeclaration(_fn) => {
                <Self as FunctionGenerator>::generate_function_declaration(self, node);
            }
            Node::IfStatement(_stmt) => {
                <Self as ControlFlowGenerator>::generate_if_statement(self, node);
            }
            Node::ForStatement(_stmt) => {
                <Self as ControlFlowGenerator>::generate_for_statement(self, node);
            }
            Node::BlockStatement(block) => {
                // NOTE: push instruction for entering a new block scope
                for stmt in &block.body {
                    self.visit_node(&stmt);
                }
                // NOTE: push instruction that we are leaving the block scope
            }
            Node::PrintStatement(stmt) => {
                self.visit_node(&stmt.argument);
                self.instructions.push(Instruction::Print);
            }
            Node::ExpressionStatement(stmt) => {
                self.visit_node(&stmt.expression);
            }
            Node::AssignmentExpression(_expr) => {
                <Self as AssignmentGenerator>::generate_assignment_expression(self, node)
            }
            Node::BinaryExpression(_expr) => {
                <Self as ArithmeticGenerator>::generate_binary_expression(self, node);
            }
            Node::CallExpression(_expr) => {
                <Self as AssignmentGenerator>::generate_call_expression(self, node);
            }
            Node::Identifier(id) => {
                if let Some(idx) = <Self as ScopeManager>::get_local(self, id) {
                    self.instructions.push(Instruction::LoadVar(*idx));
                } else {
                    <Self as ScopeManager>::get_or_create_local(self, &id.to_string());
                }
            }
            Node::Number(num) => {
                let idx = self.constants.add(Constant::Number(*num));
                self.instructions.push(Instruction::PushConst(idx));
            }
            Node::String(s) => {
                let idx = self.constants.add(Constant::String(s.clone()));
                self.instructions.push(Instruction::PushConst(idx));
            }
            Node::Boolean(b) => {
                let idx = self.constants.add(Constant::Boolean(*b));
                self.instructions.push(Instruction::PushConst(idx));
            }
            // Handle other node types...
            _ => {}
        }
    }
}

impl ScopeCore for BytecodeGenerator {
    fn local_vars(&self) -> &HashMap<String, usize> {
        &self.symbol_table
    }

    fn local_vars_mut(&mut self) -> &mut HashMap<String, usize> {
        &mut self.symbol_table
    }

    fn next_local(&self) -> usize {
        self.next_label_id
    }

    fn set_next_local(&mut self, next: usize) {
        self.next_label_id = next;
    }
}

impl AssignmentCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }
}

impl ArithmeticCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }
}

impl VariableCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }
}

impl FunctionCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }

    fn constants(&mut self) -> &mut ConstantPool {
        &mut self.constants
    }

    fn symbol_table(&mut self) -> &mut HashMap<String, usize> {
        &mut self.symbol_table
    }

    fn next_label_id(&self) -> usize {
        self.next_label_id
    }

    fn set_next_local(&mut self, next: usize) {
        self.next_label_id = next;
    }
}

impl ControlFlowCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }

    /// Create a new label
    fn new_label(&mut self) -> Label {
        let label = Label(self.next_label_id);
        self.next_label_id += 1;
        label
    }

    /// Patch all unpatched_jump instructions with their correct indices.
    fn patch_label(&mut self, label: Label) {
        // the instruction index where this label starts
        let position = self.instructions.len();

        // Defensive: don't allow re-defining a label twice.
        if self.label_positions.contains_key(&label) {
            panic!("Label {:?} already defined", label);
        }
        self.label_positions.insert(label, position);

        // If there are jumps waiting for this label, patch them.
        if let Some(jumps) = self.unpatched_jumps.remove(&label) {
            for jump in jumps {
                match jump {
                    PendingJump::Jump(pos) => {
                        // sanity check: ensure pos is in-range
                        if pos >= self.instructions.len() {
                            panic!("Pending jump position {} out of range", pos);
                        }
                        self.instructions[pos] = Instruction::Jump(position);
                    }
                    PendingJump::JumpIfFalse(pos) => {
                        if pos >= self.instructions.len() {
                            panic!("Pending jump position {} out of range", pos);
                        }
                        self.instructions[pos] = Instruction::JumpIfFalse(position);
                    }
                }
            }
        }
    }

    /// Emit a Jump instruction. If the label is already defined, emit the final target.
    /// Otherwise emit a placeholder and record the pending jump.
    fn emit_jump(&mut self, label: Label) {
        let pos = self.instructions.len();

        if let Some(&target) = self.label_positions.get(&label) {
            // Label already defined: emit final jump directly
            self.instructions.push(Instruction::Jump(target));
        } else {
            // Label not defined yet: push a placeholder immediate and record the pending jump.
            // Use usize::MAX as a clear sentinel value so it's obvious if something goes wrong.
            self.instructions.push(Instruction::Jump(usize::MAX));
            self.unpatched_jumps
                .entry(label)
                .or_default()
                .push(PendingJump::Jump(pos));
        }
    }

    /// Emit a JumpIfFalse instruction and push to unpatched_jumps
    fn emit_jump_if_false(&mut self, label: Label) {
        let pos = self.instructions.len();

        if let Some(&target) = self.label_positions.get(&label) {
            self.instructions.push(Instruction::JumpIfFalse(target));
        } else {
            self.instructions.push(Instruction::JumpIfFalse(usize::MAX));
            self.unpatched_jumps
                .entry(label)
                .or_default()
                .push(PendingJump::JumpIfFalse(pos));
        }
    }

    /// Emit an instruction
    fn emit(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }
}
