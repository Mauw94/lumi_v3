use std::collections::HashMap;

use lumi_ast::Node;

use crate::{
    expressions::{ArithmeticCore, ArithmeticGenerator, AssignmentCore, AssignmentGenerator},
    instruction::{Constant, ConstantPool, Instruction, Label, PendingJump},
    scope::local_vars::{ScopeCore, ScopeManager},
    statements::{
        control_flow::{ControlFlowCore, ControlFlowGenerator},
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
            Node::IfStatement(_stmt) => {
                <Self as ControlFlowGenerator>::generate_if_statement(self, node);
            }
            Node::BlockStatement(block) => {
                for stmt in &block.body {
                    self.visit_node(&stmt);
                }
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
            Node::Identifier(id) => {
                if let Some(idx) = <Self as ScopeManager>::get_local(self, id) {
                    self.instructions.push(Instruction::LoadVar(*idx));
                } else {
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
        let position = self.instructions.len();
        self.label_positions.insert(label, position);

        if let Some(jumps) = self.unpatched_jumps.remove(&label) {
            for jump in jumps {
                match jump {
                    PendingJump::Jump(pos) => self.instructions[pos] = Instruction::Jump(position),
                    PendingJump::JumpIfFalse(pos) => {
                        self.instructions[pos] = Instruction::JumpIfFalse(position)
                    }
                }
            }
        }
    }

    /// Emit a Jump instruction and push to unpatched_jumps
    fn emit_jump(&mut self, label: Label) {
        let pos = self.instructions.len();
        self.instructions.push(Instruction::Jump(0)); // placeholder
        
        self.unpatched_jumps
            .entry(label)
            .or_default()
            .push(PendingJump::Jump(pos));
    }

    /// Emit a JumpIfFalse instruction and push to unpatched_jumps
    fn emit_jump_if_false(&mut self, label: Label) {
        let pos = self.instructions.len();
        self.instructions.push(Instruction::JumpIfFalse(0)); // placeholder

        self.unpatched_jumps
            .entry(label)
            .or_default()
            .push(PendingJump::JumpIfFalse(pos));
    }

    /// Emit an instruction
    fn emit(&mut self, instr: Instruction) {
        self.instructions.push(instr);
    }
}
