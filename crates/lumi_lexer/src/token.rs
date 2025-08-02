use serde::{Deserialize, Serialize};

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

/// All supported token kinds
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    // Literals
    Identifier(String),
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Undefined,

    // Keywords
    Keyword(String),

    // Comments and whitespaces
    Comment(String),
    Whitespace,
    Eof,

    // Specifics
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Dot,
    Semicolon,
    Comma,
    Colon,
    Question,
    Exclamation,

    // Assignment
    Assign,
    PlusAssign,
    MinusAssign,
    Arrow,
    // TODO: extend

    // Comparison
    Equal,
    NotEqual,
    EqualEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,

    // Logical operators
    LogicalAnd,
    LogicalOr,

    // Increment/decrement operators
    Increment,
    Decrement,

    // Arithmetic operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
}

/// A token with position information
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn with_positions(
        kind: TokenKind,
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
    ) -> Self {
        Self {
            kind,
            span: Span::from_positions(start_line, start_col, end_line, end_col),
        }
    }
    pub fn start(&self) -> Position {
        self.span.start
    }

    pub fn end(&self) -> Position {
        self.span.end
    }

    pub fn is_keyword(&self) -> bool {
        matches!(self.kind, TokenKind::Keyword(_))
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self.kind, TokenKind::Identifier(_))
    }

    pub fn is_literal(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Number(_)
                | TokenKind::String(_)
                | TokenKind::Boolean(_)
                | TokenKind::Null
                | TokenKind::Undefined
        )
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self.kind,
            TokenKind::Plus
                | TokenKind::Minus
                | TokenKind::Star
                | TokenKind::Slash
                | TokenKind::Percent
                | TokenKind::Equal
                | TokenKind::EqualEqual
                | TokenKind::LessThan
                | TokenKind::LessThanEqual
                | TokenKind::GreaterThan
                | TokenKind::GreaterThanEqual
                | TokenKind::NotEqual
                | TokenKind::Increment
                | TokenKind::Decrement
        )
    }
}
