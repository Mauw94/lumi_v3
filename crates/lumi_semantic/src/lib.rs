use lumi_ast::Node;

use crate::errors::SemanticError;

mod analyzer;
mod errors;
mod scope;
mod types;

/// Result type for semantic analysis
pub type SemanticResult<T> = Result<T, SemanticError>;

pub fn analyze(ast: &Node) -> SemanticResult<()> {
    let mut analyzer = analyzer::SemanticAnalyzer::new();
    analyzer.analyze(ast)
}
