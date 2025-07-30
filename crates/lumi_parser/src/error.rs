use lumi_ast::Position;
use lumi_lexer::Token;
use thiserror::Error;

/// Result type for parser operations
pub type ParseResult<T> = Result<T, ParserError>;

/// Errors that can occur during parsing
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ParserError {
    #[error("Unexpected token: {token} at position {position}")]
    UnexpectedToken {
        token: String,
        position: Position,
        expected: Option<String>,
    },
    #[error("Invalid syntax at position {position}")]
    UnexpectedEndOfFile {
        position: Position,
        expected: Option<String>,
    },
    #[error("Invalid syntax: {message} at position {position}")]
    InvalidSyntax { message: String, position: Position },
}

impl ParserError {
    pub fn unexpected_token(token: &Token, expected: Option<&str>) -> Self {
        let position = Position {
            line: token.start().line,
            column: token.start().column,
        };
        ParserError::UnexpectedToken {
            token: format!("{:?}", token.kind),
            position,
            expected: expected.map(|s| s.to_string()),
        }
    }

    pub fn unexpected_end_of_file(expected: Option<&str>) -> Self {
        ParserError::UnexpectedEndOfFile {
            position: Position::new(1, 1), // Assuming default position for EOF
            expected: expected.map(|s| s.to_string()),
        }
    }

    pub fn invalid_syntax(message: &str, position: Position) -> Self {
        ParserError::InvalidSyntax {
            message: message.to_string(),
            position,
        }
    }
}
