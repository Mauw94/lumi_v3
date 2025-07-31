use lumi_ast::Node;

use crate::instruction::{Constant, ConstantPool, Instruction};

/// Bytecode generator for Lumi engine
pub struct BytecodeGenerator {
    pub constants: ConstantPool,
    pub instructions: Vec<Instruction>,
}

impl BytecodeGenerator {
    pub fn new() -> Self {
        BytecodeGenerator {
            constants: ConstantPool::default(),
            instructions: Vec::new(),
        }
    }

    pub fn generate(&mut self, node: &Node) {
        self.visit_node(node);
    }

    fn visit_node(&mut self, node: &Node) {
        match node {
            Node::Program(program) => {
                for stmt in &program.body {
                    self.visit_node(stmt);
                }
            }
            Node::VariableDeclaration(decl) => {
                for var in &decl.declarations {
                    self.visit_node(&var.id);
                    if let Some(init) = &var.init {
                        self.visit_node(init);
                        self.instructions.push(Instruction::StoreVar(0)); // TODO: Handle variable index
                    }
                }
            }
            Node::ExpressionStatement(stmt) => {
                self.visit_node(&stmt.expression);
            }
            Node::Number(num) => {
                let idx = self.constants.add(Constant::Number(*num));
                self.instructions.push(Instruction::PushConst(idx));
            }
            Node::String(s) => {
                let idx = self.constants.add(Constant::String(s.clone()));
                self.instructions.push(Instruction::PushConst(idx));
            }
            // Handle other node types...
            _ => {}
        }
    }
}
