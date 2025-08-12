use crate::{
    SemanticResult,
    errors::SemanticError,
    scope::{Scope, ScopeType},
    types::{Type, TypeEnvironment},
};
use lumi_ast::{Node, Span, node};

pub struct SemanticAnalyzer {
    /// Current scope being analyzed
    scope_stack: Vec<Scope>,

    /// Collected semantic errors
    errors: Vec<SemanticError>,

    /// Type environment for tracking variable types
    type_env: TypeEnvironment,
}

impl SemanticAnalyzer {
    /// Create a new semantic analyzer with an initial global scope
    pub fn new() -> Self {
        let mut analyzer = Self {
            scope_stack: Vec::new(),
            errors: Vec::new(),
            type_env: TypeEnvironment::new(),
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
            Node::ExpressionStatement(stmt) => self.visit_expression_statement(stmt),
            Node::AssignmentExpression(expr) => self.visit_assignment_expression(expr),
            Node::IfStatement(stmt) => self.visit_if_statement(stmt),
            Node::BlockStatement(stmt) => self.visit_block_statement(stmt),
            Node::FunctionDeclaration(fn_decl) => self.visit_function_declaration(fn_decl),
            Node::CallExpression(expr) => self.visit_call_expression(expr),
            // Node::BinaryExpression(expr) => self.visit_binary_expression(expr),
            Node::String(_) => Ok(Type::String),
            Node::Boolean(_) => Ok(Type::Boolean),
            Node::Number(_) => Ok(Type::Number),
            // Node::Null => self.visit_null(),
            Node::Identifier(i) => self.vist_identifier(i),
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

    fn vist_identifier(&mut self, id: &str) -> SemanticResult<Type> {
        let current_scope = self.scope_stack.last().unwrap();

        if let Some(var_type) = current_scope.get_variable_type(id) {
            Ok(var_type)
        } else {
            self.errors.push(SemanticError::UndeclaredVariable {
                name: id.to_string(),
                position: None,
            });
            Ok(Type::Undefined)
        }
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
                    let var_declared_type = self.get_type_from_annotation(var_type_node, decl)?;
                    // Check for mismatch between type annotation and initializer
                    if let Some(init) = &var_decl.init {
                        let init_type = self.visit_node(init)?;
                        if init_type != var_declared_type {
                            self.errors.push(SemanticError::TypeMismatch {
                                expected: var_declared_type.to_string(),
                                found: init_type.to_string(),
                                position: decl.span.as_ref().map(|s| s.start.clone()),
                            });
                        }
                    }
                    // Declare the type in the type environment
                    self.type_env.declare(var_name, var_declared_type.clone());
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

                current_scope.declare_variable_with_details(
                    var_name,
                    var_type.clone(),
                    !is_const,
                    1,
                );

                if var_decl.init.is_some() {
                    // If initialized, mark as initialized
                    current_scope.initialize_variable(var_name);
                }
            }
        }

        Ok(Type::Undefined)
    }

    fn visit_function_declaration(
        &mut self,
        func: &node::FunctionDeclaration,
    ) -> SemanticResult<Type> {
        let func_name = if let Some(id) = &func.id {
            if let Node::Identifier(name) = &**id {
                name.clone()
            } else {
                return Ok(Type::Undefined);
            }
        } else {
            return Ok(Type::Undefined);
        };

        let _current_scope = self.scope_stack.last().unwrap();
        let function_scope = Scope::new();
        self.scope_stack.push(function_scope);

        for param in &func.params {
            if let Node::Identifier(param_name) = param {
                let current_scope = self.scope_stack.last_mut().unwrap();
                let line_number = func.span.as_ref().map(|s| s.start.line).unwrap_or(1);
                current_scope.declare_variable(param_name.clone(), Type::Undefined, line_number);
            }
        }

        let return_type = self.visit_node(&func.body)?;

        self.scope_stack.pop();

        let current_scope = self.scope_stack.last_mut().unwrap();
        let line_number = func.span.as_ref().map(|s| s.start.line).unwrap_or(1);
        current_scope.declare_variable(
            func_name,
            Type::Function {
                params: vec![],
                return_type: Box::new(Type::Unknown),
            },
            line_number,
        );

        Ok(Type::Function {
            params: vec![],
            return_type: Box::new(return_type),
        })
    }

    fn visit_call_expression(&mut self, expr: &node::CallExpression) -> SemanticResult<Type> {
        let callee_type = self.visit_node(&expr.callee)?;

        for arg in &expr.arguments {
            self.visit_node(arg)?;
        }

        if !matches!(callee_type, Type::Function { .. }) {
            self.errors.push(SemanticError::TypeMismatch {
                expected: "function".to_string(),
                found: format!("{callee_type:?}"),
                position: None,
            });
        }
        Ok(Type::Unknown)
    }

    fn visit_assignment_expression(
        &mut self,
        expr: &node::AssignmentExpression,
    ) -> SemanticResult<Type> {
        let value_type = self.visit_node(&expr.right)?;

        if let Node::Identifier(var_name) = &*expr.left {
            let current_scope = self.scope_stack.last().unwrap();

            if let Some(var_info) = current_scope.get_variable(var_name) {
                if !var_info.mutable {
                    self.errors.push(SemanticError::ConstReassignment {
                        name: var_name.clone(),
                        position: expr.span.as_ref().map(|s| s.start.clone()),
                    });
                }
            } else {
                self.errors.push(SemanticError::UndeclaredVariable {
                    name: var_name.clone(),
                    position: expr.span.as_ref().map(|s| s.start.clone()),
                });
            }

            if let Some(var_type) = self.type_env.get_type(var_name) {
                if *var_type != value_type {
                    self.errors.push(SemanticError::TypeMismatch {
                        expected: var_type.to_string(),
                        found: value_type.to_string(),
                        position: expr.span.as_ref().map(|s| s.start.clone()),
                    });
                }
            }
        }

        Ok(value_type)
    }

    fn visit_expression_statement(
        &mut self,
        stmt: &node::ExpressionStatement,
    ) -> SemanticResult<Type> {
        match &*stmt.expression {
            Node::Identifier(i) => self.visit_identifier(i, stmt.span.clone()),
            Node::AssignmentExpression(expr) => self.visit_assignment_expression(expr),
            Node::CallExpression(expr) => self.visit_call_expression(expr),
            _ => Ok(Type::Undefined),
        }
    }

    /// Visit if statement
    fn visit_if_statement(&mut self, stmt: &node::IfStatement) -> SemanticResult<Type> {
        let condition_type = self.visit_node(&stmt.expr)?;
        if !condition_type.is_compatible_with(&Type::Boolean) {
            self.errors.push(SemanticError::TypeMismatch {
                expected: "boolean".to_string(),
                found: format!("{:?}", condition_type),
                position: stmt.span.as_ref().map(|s| s.start.clone()),
            })
        }

        let current_scope = self.scope_stack.last().unwrap().clone();
        let block_scope = Scope::new_child(current_scope, ScopeType::Block);
        self.scope_stack.push(block_scope);
        self.visit_node(&stmt.stmt)?;
        self.scope_stack.pop();

        if let Some(else_part) = &stmt.else_part {
            let current_scope = self.scope_stack.last().unwrap().clone();
            let block_scope = Scope::new_child(current_scope, ScopeType::Block);
            self.scope_stack.push(block_scope);
            self.visit_node(else_part)?;
            self.scope_stack.pop();
        }

        Ok(Type::Undefined)
    }

    /// Visit block statement
    fn visit_block_statement(&mut self, stmt: &node::BlockStatement) -> SemanticResult<Type> {
        let current_scope = self.scope_stack.last().unwrap().clone();
        let block_scope = Scope::new_child(current_scope, ScopeType::Block);
        self.scope_stack.push(block_scope);

        let mut last_type = Type::Undefined;

        for statement in &stmt.body {
            last_type = self.visit_node(statement)?;
        }

        self.scope_stack.pop();

        Ok(last_type)
    }

    fn visit_identifier(
        &mut self,
        identifier: &String,
        span: Option<Span>,
    ) -> SemanticResult<Type> {
        let name = identifier.to_string();

        let current_scope = self.scope_stack.last_mut().unwrap();

        // If variable doesn't exist, return undeclared variable error
        if !current_scope.is_variable_declared_in_current_scope(&name) {
            self.errors.push(SemanticError::UndeclaredVariable {
                name,
                position: span.as_ref().map(|s| s.start.clone()),
            });
            return Ok(Type::Undefined);
        }

        if let Some(var_type) = self.type_env.get_type(&name) {
            return Ok(var_type.clone());
        }

        Ok(Type::Undefined)
    }

    fn get_type_from_annotation(
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
