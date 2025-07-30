use lumi_ast::{Node, Position, Program, Span, VariableDeclaration, VariableDeclarator};
use lumi_lexer::{lexer, token::TokenKind, Lexer, Token};

use crate::{
    error::{ParseResult, ParserError},
    recovery::{ErrorRecovery, ParsingContext},
};

/// Main parser struct that holds the source code, lexer, and parsing state
pub struct Parser {
    source: String,
    lexer: Lexer,
    current: Option<Token>,
    previous: Option<Token>,
    error_recovery: ErrorRecovery,
    context: ParsingContext,
    strict_mode: bool,
}

impl Parser {
    pub fn new(source: &str) -> Self {
        let mut lexer = Lexer::new(source);
        let current = match lexer.next_token() {
            Ok(token) => Some(token),
            Err(err) => {
                eprintln!("Lexer error: {}", err);
                None
            }
        };

        Self {
            source: source.to_string(),
            lexer,
            current,
            previous: None,
            error_recovery: ErrorRecovery::default(),
            context: ParsingContext::TopLevel,
            strict_mode: false,
        }
    }

    /// Parse the source code and return the AST
    pub fn parse(&mut self) -> ParseResult<Node> {
        self.parse_program()
    }

    /// Parse a program node
    fn parse_program(&mut self) -> ParseResult<Node> {
        let mut body = Vec::new();
        let start_pos = self.current_position();

        if self.is_eof() {
            let end_pos = self.previous_position();
            let span = self.create_span(start_pos, end_pos);
            return Ok(Node::Program(Program {
                body,
                span: Some(span),
            }));
        }

        while !self.is_eof() {
            match self.parse_statement() {
                Ok(stmt) => body.push(stmt),
                Err(err) => {
                    // TODO: handle error recovery
                }
            }
        }

        let end_pos = self.previous_position();
        let span = self.create_span(start_pos, end_pos);

        Ok(Node::Program(Program {
            body,
            span: Some(span),
        }))
    }

    /// Parse a statement node
    fn parse_statement(&mut self) -> ParseResult<Node> {
        let old_context = self.context.clone();
        self.context = ParsingContext::Statement;

        let result = if let Some(token) = &self.current {
            match &token.kind {
                TokenKind::Keyword(kw) => match kw.as_str() {
                    "let" | "const" => self.parse_variable_declaration(),
                    // "if" => self.parse_if_statement(),
                    // "for" => self.parse_for_loop(),
                    // "while" => self.parse_while_loop(),
                    // _ => self.parse_expression_statement(),
                    _ => {
                        // NOTE: temporary placeholder
                        self.advance(); // Advance to the next token or we end up in an infinite loop
                        Ok(Node::Null)
                    }
                },
                _ => {
                    // NOTE: temporary placeholder
                    self.advance(); // Advance to the next token or we end up in an infinite loop
                    Ok(Node::Null)
                }
            }
        } else {
            Err(ParserError::unexpected_end_of_file(None))
        };

        self.context = old_context;
        result
    }

    fn parse_variable_declaration(&mut self) -> ParseResult<Node> {
        let kind = if let Some(token) = &self.current {
            if let TokenKind::Keyword(kw) = &token.kind {
                match kw.as_str() {
                    "let" => "let",
                    "const" => "const",
                    "var" => "var",
                    _ => unreachable!(),
                }
            } else {
                unreachable!()
            }
        } else {
            unreachable!()
        };

        self.advance(); // consume the keyword (let/const)

        let mut declarations: Vec<VariableDeclarator> = Vec::new();

        loop {
            let id = self.parse_identifier()?; // Name of the variable
            let var_type = self.try_parse_identifier_type()?;
            let init = if self.check(TokenKind::Arrow) {
                self.advance(); // consume the arrow
                Some(Box::new(self.parse_expression()?)) // Parse the expression after the arrow
            } else {
                None
            };
            let span = self.create_span_from_tokens();
            declarations.push(VariableDeclarator {
                id: Box::new(id),
                var_type: var_type.map(Box::new),
                init: init,
                span: Some(span),
            });

            if !self.check(TokenKind::Comma) {
                break; // Exit if no more declarations
            }
            self.advance(); // consume the comma
        }

        if self.check(TokenKind::Semicolon) {
            self.advance(); // consume the semicolon
        }

        let pan = self.create_span_from_tokens();
        Ok(Node::VariableDeclaration(VariableDeclaration {
            kind: kind.to_string(),
            declarations,
            span: Some(pan),
        }))
    }

    /// Parse an identifier node
    fn parse_identifier(&mut self) -> ParseResult<Node> {
        if let Some(token) = &self.current {
            if let TokenKind::Identifier(name) = &token.kind {
                let name = name.clone();
                self.advance(); // consume the identifier token
                return Ok(Node::Identifier(name));
            } else {
                Err(ParserError::invalid_syntax(
                    "Expected identifier",
                    self.current_position().unwrap_or_default(),
                ))
            }
        } else {
            Err(ParserError::unexpected_end_of_file(None))
        }
    }

    fn parse_expression(&mut self) -> ParseResult<Node> {
        // TODO: chain of epxression parsing
        // self.parse_assignment_expression()
        self.parse_primary_expression() // TODO: temporary until we implement full expression parsing
    }

    fn parse_assignment_expression(&mut self) -> ParseResult<Node> {
        unreachable!()
    }

    fn parse_primary_expression(&mut self) -> ParseResult<Node> {
        if let Some(token) = &self.current {
            match &token.kind {
                TokenKind::Number(n) => {
                    let value = *n;
                    self.advance(); // consume the number token
                    Ok(Node::Number(value))
                }
                TokenKind::Identifier(name) => {
                    let value = name.clone();
                    self.advance(); // consume the identifier
                    Ok(Node::Identifier(value))
                }
                TokenKind::String(s) => {
                    let value = s.clone();
                    self.advance(); // consume the string
                    Ok(Node::String(value))
                }
                TokenKind::Boolean(b) => {
                    let value = *b;
                    self.advance(); // consume the boolean
                    Ok(Node::Boolean(value))
                }
                TokenKind::Null => {
                    self.advance(); // consume the null
                    Ok(Node::Null)
                }
                TokenKind::Undefined => {
                    self.advance();
                    Ok(Node::Undefined)
                }
                _ => {
                    if self.check_idenfitier() {
                        self.parse_identifier()
                    } else {
                        Err(ParserError::unexpected_token(
                            token,
                            Some("Expected primary expression"),
                        ))
                    }
                }
            }
        } else {
            Err(ParserError::unexpected_end_of_file(None))
        }
    }

    /// Try to parse an identifier type (e.g., after a colon)
    fn try_parse_identifier_type(&mut self) -> ParseResult<Option<Node>> {
        if self.check(TokenKind::Colon) {
            self.advance(); // consume the colon
            if let Some(token) = &self.current {
                if let TokenKind::Keyword(name) = &token.kind {
                    let name = name.clone();
                    self.advance(); // consume the identifier token
                    return Ok(Some(Node::Identifier(name)));
                }
            }
        }
        Ok(None)
    }

    fn check_idenfitier(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(token.kind, TokenKind::Identifier(_))
        } else {
            false
        }
    }

    /// Check if the current token matches a specific kind
    fn check(&self, token_kind: TokenKind) -> bool {
        if let Some(token) = &self.current {
            std::mem::discriminant(&token.kind) == std::mem::discriminant(&token_kind)
        } else {
            false
        }
    }

    /// Advance to the next token
    fn advance(&mut self) {
        self.previous = self.current.take();
        match self.lexer.next_token() {
            Ok(token) => self.current = Some(token),
            Err(err) => {
                eprintln!("Lexer error: {}", err);
                self.current = None;
            }
        }
    }

    /// Check if we're at the end of the file
    fn is_eof(&self) -> bool {
        self.current.is_none()
            || self
                .current
                .as_ref()
                .map_or(false, |t| t.kind == TokenKind::Eof)
    }

    /// Gets the position of the current token
    fn current_position(&self) -> Option<Position> {
        self.current.as_ref().map(|t| Position {
            line: t.start().line,
            column: t.start().column,
        })
    }

    /// Gets the previous token
    fn previous_position(&self) -> Option<Position> {
        self.previous.as_ref().map(|t| Position {
            line: t.end().line,
            column: t.end().column,
        })
    }

    /// Create a span from the current and previous token positions
    fn create_span_from_tokens(&self) -> Span {
        let start = self.previous_position().unwrap_or_default();
        let end = self.current_position().unwrap_or_default();
        Span::new(start, end)
    }

    /// Create a span from positions
    fn create_span(&self, start: Option<Position>, end: Option<Position>) -> Span {
        let start = start.unwrap_or_default();
        let end = end.unwrap_or_default();
        Span::new(start, end)
    }
}
