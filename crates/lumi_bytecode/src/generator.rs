use std::collections::HashMap;

use lumi_ast::{Node, VariableDeclarator};

use crate::{
    instruction::{Constant, ConstantPool, Instruction},
    scope::local_vars::{ScopeCore, ScopeManager},
    statements::variable::{VariableCore, VariableGenerator},
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
            Node::IfStatement(stmt) => {}
            Node::PrintStatement(stmt) => {
                self.visit_node(&stmt.argument);
                self.instructions.push(Instruction::Print);
            }
            Node::ExpressionStatement(stmt) => {
                self.visit_node(&stmt.expression);
            }
            Node::AssignmentExpression(expr) => {
                let var_name = match &*expr.left {
                    Node::Identifier(id) => id.clone(),
                    _ => unreachable!(), // TODO: should also give an informative error
                };

                // Pushes new constant value for the assignment and a value.
                // TODO: remove unused constants by doing a compiler pass after byte code generation.
                self.visit_node(&expr.right);

                // TODO: finally implement bytecode error handinlg plz
                // Naively expect the variable to exist. Need to handle errors here.
                let idx = match self.symbol_table.get(&var_name) {
                    Some(idx) => idx,
                    None => todo!(), // Should throw error in byte code generation.
                };

                self.instructions.push(Instruction::StoreVar(*idx));
            }
            Node::BinaryExpression(expr) => {
                self.visit_node(&expr.left);
                self.visit_node(&expr.right);
                match expr.operator.as_str() {
                    "+" => self.instructions.push(Instruction::Add),
                    "-" => self.instructions.push(Instruction::Sub),
                    "*" => self.instructions.push(Instruction::Mul),
                    "/" => self.instructions.push(Instruction::Div),
                    _ => panic!("Unsupported operator: {}", expr.operator),
                }
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

impl VariableCore for BytecodeGenerator {
    fn instructions(&mut self) -> &mut Vec<Instruction> {
        &mut self.instructions
    }

    fn visit_node(&mut self, node: &Node) {
        self.visit_node(node)
    }
}
