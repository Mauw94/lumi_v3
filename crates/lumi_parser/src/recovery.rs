use lumi_ast::Position;
use lumi_lexer::{token::TokenKind, Token};

use crate::error::ParserError;

/// Recovery strategy for the parser
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryStrategy {
    /// Skip until a specific token is found
    SkipUntil(Vec<String>),

    /// Skip until a specific statement is found
    SkipUntilStatement,

    /// Skip until a block is found
    SkipUntilBlock,

    /// Skip until a function declaration is found
    SkipUntilFunction,

    /// Skip until a class declaration
    SkipUntilClass,

    /// Skip until a module declaration is found
    SkipUntilModule,

    /// Insert a missing token
    InsertToken(String),

    /// Replace the current token
    ReplaceToken(String),

    /// Delete the current token
    DeleteToken,

    /// No recovery, just log the error
    NoRecovery,
}

/// Error recovery context that holds information about the current parsing state
#[derive(Debug, Clone)]
pub struct RecoveryContext {
    /// Current token that caused the error
    pub current_token: Option<Token>,

    /// Previous token
    pub previous_token: Option<Token>,

    /// Tokens that can be used for recovery
    pub recovery_tokens: Vec<String>,

    /// Current parsing context
    pub context: ParsingContext,
}

/// Parsing context for the parser, which can change based on the current parsing state
#[derive(Debug, Clone, PartialEq)]
pub enum ParsingContext {
    /// Parsing at the top level of the document
    TopLevel,

    /// Parsing inside a statement
    Statement,

    /// Parsing inside a block
    Block,

    /// Parsing inside a function
    Function,

    /// Parsing inside a class
    Class,

    /// Parsing inside a module
    Module,

    /// Parsing inside an expression
    Expression,

    /// Parsing inside a variable declaration
    Declaration,
}

impl RecoveryContext {
    pub fn new(
        current_token: Option<Token>,
        previous_token: Option<Token>,
        context: ParsingContext,
    ) -> Self {
        Self {
            current_token,
            previous_token,
            recovery_tokens: Vec::new(),
            context,
        }
    }

    /// Add recovery tokens to the context
    pub fn with_recovery_tokens(mut self, tokens: Vec<String>) -> Self {
        self.recovery_tokens = tokens;
        self
    }

    /// Determine the recovery strategy based on the current context and token
    pub fn determine_strategy(&self) -> RecoveryStrategy {
        match &self.context {
            ParsingContext::TopLevel => {
                if let Some(token) = &self.current_token {
                    match token.kind {
                        TokenKind::Semicolon | TokenKind::RightBrace => {
                            RecoveryStrategy::SkipUntil(vec![";".to_string(), "}".to_string()])
                        }
                        _ => RecoveryStrategy::SkipUntilStatement,
                    }
                } else {
                    RecoveryStrategy::NoRecovery
                }
            }

            ParsingContext::Statement => {
                RecoveryStrategy::SkipUntil(vec![";".to_string(), "}".to_string(), ")".to_string()])
            }

            ParsingContext::Block => RecoveryStrategy::SkipUntil(vec!["}".to_string()]),

            ParsingContext::Function => {
                RecoveryStrategy::SkipUntil(vec!["}".to_string(), ";".to_string()])
            }

            ParsingContext::Class => RecoveryStrategy::SkipUntil(vec!["}".to_string()]),

            ParsingContext::Module => RecoveryStrategy::SkipUntil(vec![
                "}".to_string(),
                "import".to_string(),
                "export".to_string(),
            ]),

            ParsingContext::Expression => RecoveryStrategy::SkipUntil(vec![
                ";".to_string(),
                ",".to_string(),
                ")".to_string(),
                "]".to_string(),
                "}".to_string(),
            ]),

            ParsingContext::Declaration => {
                RecoveryStrategy::SkipUntil(vec![";".to_string(), "}".to_string()])
            }
        }
    }

    /// Check if the token is a recovery token
    pub fn is_recovery_token(&self, token: &Token) -> bool {
        let token_str = format!("{:?}", token.kind);
        self.recovery_tokens.iter().any(|t| t == &token_str)
    }

    /// Get the current position of the parser
    pub fn current_position(&self) -> Option<Position> {
        self.current_token.as_ref().map(|t| Position {
            line: t.start().line,
            column: t.start().column,
        })
    }
}

/// Error recovery mechanism for the parser
#[derive(Debug)]
pub struct ErrorRecovery {
    max_errors: usize,
    error_count: usize,
    errors: Vec<ParserError>,
}

impl ErrorRecovery {
    /// Create a new ErrorRecovery instance with a specified maximum number of errors
    pub fn new(max_errors: usize) -> Self {
        Self {
            max_errors,
            error_count: 0,
            errors: Vec::new(),
        }
    }

    /// Check if recovery is still possible
    pub fn can_recover(&self) -> bool {
        self.error_count < self.max_errors
    }

    /// Add an error to the recovery list, respecting the maximum error count
    pub fn record_error(&mut self, error: ParserError) {
        if self.error_count < self.max_errors {
            self.errors.push(error);
            self.error_count += 1;
        }
    }

    /// Check if the error recovery has reached its maximum error count
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Clear all recorded errors
    pub fn clear_errors(&mut self) {
        self.errors.clear();
        self.error_count = 0;
    }

    /// Get the list of recorded errors
    pub fn errors(&self) -> &[ParserError] {
        &self.errors
    }

    /// Get the error count
    pub fn error_count(&self) -> usize {
        self.error_count
    }
}

impl Default for ErrorRecovery {
    fn default() -> Self {
        Self::new(10) // Default to a maximum of 10 errors
    }
}
