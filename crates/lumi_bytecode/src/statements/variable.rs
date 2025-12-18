use lumi_ast::Node;

use crate::{scope::local_vars::ScopeManager, Instruction};

pub trait VariableGenerator {
    fn generate_variable_declaration(&mut self, node: &Node);
}

pub trait VariableCore {
    fn instructions(&mut self) -> &mut Vec<Instruction>;
    fn visit_node(&mut self, node: &Node);
}

impl<T> VariableGenerator for T
where
    T: VariableCore + ScopeManager,
{
    fn generate_variable_declaration(&mut self, node: &Node) {
        if let Node::VariableDeclaration(decl) = node {
            for var in &decl.declarations {
                if let Node::Identifier(name) = &*var.var_name {
                    if let Some(init) = &var.init {
                        self.visit_node(init);
                        let local_idx = self.get_or_create_local(name);
                        self.instructions().push(Instruction::StoreVar(local_idx));
                    } else {
                        // NOTE: move to something shared. core?
                        if let Some(var_type) = &var.var_type {
                            match &**var_type {
                                Node::Identifier(id) => match id.to_string().as_str() {
                                    "int" => self.visit_node(&Node::Number(0.0)),
                                    "str" => self.visit_node(&Node::String("".to_string())),
                                    _ => {}
                                },
                                _ => {}
                            }
                        }
                        let local_idx = self.get_or_create_local(name);
                        self.instructions().push(Instruction::StoreVar(local_idx));
                    }
                }
            }
        }
    }
}
