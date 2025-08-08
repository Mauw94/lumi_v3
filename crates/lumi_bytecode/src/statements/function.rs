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
                chunk,
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

        std::mem::swap(&mut old_instructions, self.instructions());
        std::mem::swap(&mut old_constants, &mut self.constants().values);

        for param in &decl.params {
            self.visit_node(param);
        }

        self.visit_node(&decl.body);

        self.instructions().push(Instruction::Return);

        let func_instructions = std::mem::take(self.instructions());
        let func_constants = std::mem::take(self.constants());

        (func_instructions, func_constants.values)
    }
}
