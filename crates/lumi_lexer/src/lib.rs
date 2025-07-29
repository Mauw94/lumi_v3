pub use crate::{error::LexError, lexer::Lexer, token::Token};

pub mod error;
pub mod lexer;
pub mod token;

/// Tokenize source code into vec of tokens
pub fn tokenize(source: &str) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}
