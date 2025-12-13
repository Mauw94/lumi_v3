use std::collections::HashMap;

use lumi_ast::{FunctionDeclaration, Node};

use crate::{
    core::FunctionObj, scope::local_vars::ScopeManager, Constant, ConstantPool, Instruction,
};

pub trait FunctionGenerator {
    fn generate_function_declaration(&mut self, node: &Node);
    fn code_gen_function_body(
        &mut self,
        decl: &FunctionDeclaration,
    ) -> (Vec<Instruction>, Vec<Constant>);
}

pub trait FunctionCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn symbol_table(&mut self) -> &mut HashMap<String, usize>;
    fn next_label_id(&self) -> usize;
    fn set_next_local(&mut self, next: usize);
    fn constants(&mut self) -> &mut ConstantPool;
    fn visit_node(&mut self, node: &Node);
}

impl<T> FunctionGenerator for T
where
    T: FunctionCore + ScopeManager,
{
    fn generate_function_declaration(&mut self, node: &Node) {
        if let Node::FunctionDeclaration(decl) = node {
            let (chunk, constants) = self.code_gen_function_body(decl);

            let fn_name = if let Some(node) = &decl.id {
                Some(node.name())
            } else {
                None
            };

            let func_obj = FunctionObj {
                name: fn_name,
                arity: decl.params.len(),
                instructions: chunk,
                constants,
            };

            let idx = self.constants().add(Constant::Function(func_obj));
            self.instructions().push(Instruction::PushConst(idx));
        }
    }

    fn code_gen_function_body(
        &mut self,
        decl: &FunctionDeclaration,
    ) -> (Vec<Instruction>, Vec<Constant>) {
        let mut old_instructions = Vec::new();
        let mut old_constants = Vec::new();
        let mut old_locals = HashMap::new();
        let old_next_label = self.next_label_id();

        std::mem::swap(&mut old_instructions, self.instructions());
        std::mem::swap(&mut old_constants, &mut self.constants().values);
        std::mem::swap(&mut old_locals, &mut self.symbol_table());
        self.set_next_local(0);

        for param in &decl.params {
            self.visit_node(param);
        }

        self.visit_node(&decl.body);

        self.instructions().push(Instruction::Return);

        let func_instructions = std::mem::take(self.instructions());
        let func_constants = std::mem::take(self.constants());

        std::mem::swap(self.instructions(), &mut old_instructions);
        std::mem::swap(&mut self.constants().values, &mut old_constants);
        std::mem::swap(self.symbol_table(), &mut old_locals);
        self.set_next_local(old_next_label);

        (func_instructions, func_constants.values)
    }
}
