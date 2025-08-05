use std::collections::HashMap;

use lumi_ast::Node;

use crate::{
    expressions::{ArithmeticCore, ArithmeticGenerator, AssignmentCore, AssignmentGenerator},
    instruction::{Constant, ConstantPool, Instruction},
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
    pub symbol_table: HashMap<String, usize>, // Maps variable names to their indices
    pub next_var_index: usize,                // Index for the next variable to be added
}

impl BytecodeGenerator {
    pub fn new() -> Self {
        BytecodeGenerator {
            constants: ConstantPool::default(),
            instructions: Vec::new(),
            symbol_table: HashMap::new(),
            next_var_index: 0,
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
        self.next_var_index
    }

    fn set_next_local(&mut self, next: usize) {
        self.next_var_index = next;
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
}
