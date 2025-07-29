use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum LexError {
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Invalid number: {0}")]
    InvalidNumber(String),
    #[error("Unterminated string")]
    UnterminatedString,
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Unterminated comment")]
    UnterminatedComment,
}
