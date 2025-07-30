use crate::error::ParserError;

/// Error recovery mechanism for the parser
#[derive(Debug)]
pub struct ErrorRecovery {
    max_errors: usize,
    error_count: usize,
    errors: Vec<ParserError>,
}

/// Parsing context for the parser, which can change based on the current parsing state
#[derive(Debug, Clone, PartialEq)]
pub enum ParsingContext {
    TopLevel,
    Statement,
    Block,
    Function,
    Class,
    Module,
    Expression,
    Declaration,
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
