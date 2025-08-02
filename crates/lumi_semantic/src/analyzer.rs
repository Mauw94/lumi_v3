use lumi_ast::{Node, node};

use crate::{SemanticResult, analyzer, errors::SemanticError, scope::Scope, types::Type};

pub struct SemanticAnalyzer {
    /// Current scope being analyzed
    scope_stack: Vec<Scope>,

    /// Collected semantic errors
    errors: Vec<SemanticError>,
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer with an initial global scope
    pub fn new() -> Self {
        let mut analyzer = Self {
            scope_stack: Vec::new(),
            errors: Vec::new(),
        };

        analyzer.scope_stack.push(Scope::new_global());
        analyzer
    }

    /// Analyze the given AST and return any semantic errors found
    pub fn analyze(&mut self, ast: &Node) -> SemanticResult<()> {
        // Traverse the AST and perform semantic analysis
        self.visit_node(ast)?;

        if !self.errors.is_empty() {
            return Err(self.errors.remove(0));
        }
        Ok(())
    }

    fn visit_node(&mut self, node: &Node) -> SemanticResult<Type> {
        match node {
            Node::Program(program) => self.visit_program(program),
            Node::VariableDeclaration(decl) => self.visit_variable_declaration(decl),
            // Node::BinaryExpression(expr) => self.visit_binary_expression(expr),
            Node::String(_) => Ok(Type::String),
            Node::Boolean(_) => Ok(Type::Boolean),
            Node::Number(_) => Ok(Type::Number),
            // Node::Null => self.visit_null(),
            // Node::Identifier(i) => self.visit_identifier(i),
            // Node::Undefined => self.visit_undefined(),
            _ => Ok(Type::Undefined), // Temporary
        }
    }

    fn visit_program(&mut self, program: &node::Program) -> SemanticResult<Type> {
        for stmt in &program.body {
            self.visit_node(stmt)?;
        }
        Ok(Type::Undefined)
    }

    fn visit_variable_declaration(
        &mut self,
        decl: &node::VariableDeclaration,
    ) -> SemanticResult<Type> {
        let is_const = decl.kind == "const";

        for var_decl in &decl.declarations {
            if let Node::Identifier(var_name) = &*var_decl.id {
                // Get the type
                let var_type = if let Some(var_type_node) = &var_decl.var_type {
                    // Get type from the type annotation
                    let var_declared_type = self.fetch_variable_type(var_type_node, decl)?;
                    // Check for mismatch between type annotation and initializer
                    if let Some(init) = &var_decl.init {
                        let init_type = self.visit_node(init)?;
                        if init_type != var_declared_type {
                            return Err(SemanticError::TypeMismatch {
                                expected: var_declared_type.to_string(),
                                found: init_type.to_string(),
                                position: decl.span.as_ref().map(|s| s.start.clone()),
                            });
                        }
                    }
                    var_declared_type
                } else if let Some(init) = &var_decl.init {
                    // Infer type from initializer
                    self.visit_node(init)?
                } else {
                    Type::Undefined // Default type if no initializer and no type annotation
                };

                // Get mutable reference to the current scope
                let current_scope = self.scope_stack.last_mut().unwrap();

                // Check if variable already exists in the current scope
                if current_scope.is_variable_declared_in_current_scope(var_name) {
                    self.errors.push(SemanticError::DuplicateDeclaration {
                        name: var_name.clone(),
                        position: decl.span.as_ref().map(|s| s.start.clone()),
                    });
                    continue; // Skip further processing for this variable
                }

                current_scope.declare_variable(var_name, var_type.clone(), !is_const, 1);

                if var_decl.init.is_some() {
                    // If initialized, mark as initialized
                    current_scope.initialize_variable(var_name);
                }
            }
        }

        Ok(Type::Undefined)
    }

    // fn visit_identifier(&mut self, identifier: &node::Identifier) -> SemanticResult<Type> {
    //     let current_scope = self.scope_stack.last().unwrap();
    //     if let Some(var_type) = current_scope.get_variable_type(identifier) {
    //         Ok(var_type.clone())
    //     } else {
    //         Err(SemanticError::UndeclaredVariable {
    //             name: identifier.to_string(),
    //             position: identifier.span.as_ref().map(|s| s.start.clone()),
    //         })
    //     }
    // }

    fn fetch_variable_type(
        &self,
        var_type: &Box<Node>,
        decl: &node::VariableDeclaration,
    ) -> SemanticResult<Type> {
        return match &**var_type {
            Node::Identifier(id) => match id.to_string().as_str() {
                "int" => Ok(Type::Number),
                // "float" => Some(Type::Float), // TODO: implement when adding float type
                "str" => Ok(Type::String),
                "boolean" => Ok(Type::Boolean),
                // Add more types as needed
                _ => Err(SemanticError::InvalidType {
                    type_name: id.to_string(),
                    position: decl.span.as_ref().map(|s| s.start.clone()),
                }), // Handle unknown types
            },
            _ => {
                // Handle complex types or type expressions
                Err(SemanticError::InvalidType {
                    type_name: format!("{:?}", var_type),
                    position: decl.span.as_ref().map(|s| s.start.clone()),
                })
            }
        };
    }

    /// Collect semantic errors found during analysis
    pub fn collect_errors(&self) -> &[SemanticError] {
        &self.errors
    }
}
