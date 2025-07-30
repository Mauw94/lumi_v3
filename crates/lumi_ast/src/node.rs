use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents a position in the source code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl Default for Position {
    fn default() -> Self {
        Position { line: 1, column: 1 }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// Represents a span of source code
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

impl Span {
    pub fn new(start: Position, end: Position) -> Self {
        Self { start, end }
    }

    pub fn from_positions(
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    ) -> Self {
        Self {
            start: Position::new(start_line, start_col),
            end: Position::new(end_line, end_col),
        }
    }
}

/// Main AST node enum containing all possible node types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Node {
    // Program structure
    Program(Program),

    // Declarations
    VariableDeclaration(VariableDeclaration),

    // Expressions
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),

    // Statements
    PrintStatement(PrintStatement),

    // Literals
    ArrayLiteral(ArrayLiteral),

    // Other
    String(String),
    Identifier(String),
    Boolean(bool),
    Number(f64),
    Null,
    Undefined,
}

// Program structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Program {
    pub body: Vec<Node>,
    pub span: Option<Span>,
}

// Declarations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableDeclaration {
    pub kind: String,                          // "let", "const"
    pub declarations: Vec<VariableDeclarator>, // Supports multiple declarations like "let x = 5, y = 10, z = 15;"
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VariableDeclarator {
    pub id: Box<Node>,
    pub var_type: Option<Box<Node>>, // Optional type annotation
    pub init: Option<Box<Node>>, // The initializer expression (e.g. 5) - can be None if the variable is declared but not initialized.
    pub span: Option<Span>,      // Span of the declarator
}

// Expressions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BinaryExpression {
    pub left: Box<Node>,
    pub operator: String,
    pub right: Box<Node>,
    pub span: Option<Span>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpression {
    pub operator: String,
    pub argument: Box<Node>,
    pub prefix: bool, // true for prefix (e.g. -x), false for postfix (e.g. x++)
    pub span: Option<Span>,
}

// Statements
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PrintStatement {
    pub argument: Box<Option<Node>>,
    pub span: Option<Span>,
}

// Literals
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayLiteral {
    pub elements: Vec<Option<Node>>,
    pub span: Option<Span>,
}
