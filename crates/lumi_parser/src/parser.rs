use lumi_ast::{
    AssignmentExpression, BinaryExpression, ExpressionStatement, LogicalExpression, Node, Position,
    Program, Span, UnaryExpression, VariableDeclaration, VariableDeclarator,
};
use lumi_lexer::{lexer, token::TokenKind, Lexer, Token};

use crate::{
    error::{ParseResult, ParserError},
    recovery::{ErrorRecovery, ParsingContext, RecoveryContext, RecoveryStrategy},
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
                Err(error) => {
                    if !self.try_recover_from_error(error.clone()) {
                        return Err(error);
                    }
                    if self.is_eof() {
                        break; // Stop parsing if we reach EOF
                    }
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
                _ => self.parse_expression_statement(),
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

    fn parse_expression_statement(&mut self) -> ParseResult<Node> {
        let expr = self.parse_expression()?;

        if self.check(TokenKind::Semicolon) {
            self.advance(); // consume the semicolon
        }

        let span = self.create_span_from_tokens();
        Ok(Node::ExpressionStatement(ExpressionStatement {
            expression: Box::new(expr),
            span: Some(span),
        }))
    }

    fn parse_expression(&mut self) -> ParseResult<Node> {
        self.parse_assignment_expression()
    }

    fn parse_assignment_expression(&mut self) -> ParseResult<Node> {
        let left = self.parse_logical_or_expression()?;

        if self.is_assignment_operator() {
            let operator = self.current_token_string();
            self.advance(); // Consume operator
            let right = Box::new(self.parse_assignment_expression()?);

            let span = self.create_span_from_tokens();
            return Ok(Node::AssignmentExpression(AssignmentExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            }));
        } else {
            return Ok(left);
        }
    }

    fn parse_logical_or_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_logical_and_expression()?;

        while self.check(TokenKind::LogicalOr) {
            let operator = self.current_token_string();
            self.advance(); // consume the 'or' token
            let right = Box::new(self.parse_logical_and_expression()?);
            let span = self.create_span_from_tokens();

            left = Node::LogicalExpression(LogicalExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }

        Ok(left)
    }

    fn parse_logical_and_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_equality_expression()?;

        while self.check(TokenKind::LogicalAnd) {
            let operator = self.current_token_string();
            self.advance(); // consume the 'and' token
            let right = Box::new(self.parse_equality_expression()?);
            let span = self.create_span_from_tokens();

            left = Node::LogicalExpression(LogicalExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }

        Ok(left)
    }

    fn parse_equality_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_relational_expression()?;

        while self.is_equality_operator() {
            let operator = self.current_token_string();
            self.advance(); // consume the equality operator
            let right = Box::new(self.parse_relational_expression()?);
            let span = self.create_span_from_tokens();

            left = Node::BinaryExpression(BinaryExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }

        Ok(left)
    }

    fn parse_relational_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_additive_expression()?;

        while self.is_relational_operator() {
            let operator = self.current_token_string();
            self.advance(); // consume the relational operator
            let right = Box::new(self.parse_additive_expression()?);
            let span = self.create_span_from_tokens();

            left = Node::BinaryExpression(BinaryExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }

        Ok(left)
    }

    fn parse_additive_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_multiplicative_expression()?;

        while self.is_additive_operator() {
            let operator = self.current_token_string();
            self.advance(); // consume the operator
            let right = Box::new(self.parse_multiplicative_expression()?);
            let span = self.create_span_from_tokens();

            left = Node::BinaryExpression(BinaryExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }

        Ok(left)
    }

    fn parse_multiplicative_expression(&mut self) -> ParseResult<Node> {
        let mut left = self.parse_unary_expression()?;

        while self.is_multiplicative_operator() {
            let operator = self.current_token_string();
            self.advance(); // consume the operator
            let right = Box::new(self.parse_unary_expression()?);
            let span = self.create_span_from_tokens();

            left = Node::BinaryExpression(BinaryExpression {
                left: Box::new(left),
                operator,
                right,
                span: Some(span),
            });
        }

        Ok(left)
    }

    fn parse_unary_expression(&mut self) -> ParseResult<Node> {
        if self.is_unary_operator() {
            let operator = self.current_token_string();
            let prefix = true;
            self.advance(); // Consume operator
            let argument = Box::new(self.parse_unary_expression()?);

            let span = self.create_span_from_tokens();
            return Ok(Node::UnaryExpression(UnaryExpression {
                operator,
                argument,
                prefix,
                span: Some(span),
            }));
        }

        self.parse_postfix_expression()
    }

    fn parse_postfix_expression(&mut self) -> ParseResult<Node> {
        self.parse_primary_expression()
        // let mut expr = self.parse_primary_expression()?;

        // TODO: add later
        //   loop {
        //     if let Some(token) = &self.current {
        //         match &token.kind {
        //             TokenKind::LeftBracket => {
        //                 self.advance(); // Consume '['
        //                 let property = Box::new(self.parse_expression()?);
        //                 self.expect(TokenKind::RightBracket)?;

        //                 let span = self.create_span_from_tokens();
        //                 expr = Node::MemberExpression(MemberExpression {
        //                     object: Box::new(expr),
        //                     property,
        //                     computed: true,
        //                     optional: false,
        //                     span: Some(span),
        //                 });
        //             }

        //             TokenKind::Dot => {
        //                 self.advance(); // Consume '.'
        //                 let property = Box::new(self.parse_identifier()?);

        //                 let span = self.create_span_from_tokens();
        //                 expr = Node::MemberExpression(MemberExpression {
        //                     object: Box::new(expr),
        //                     property,
        //                     computed: false,
        //                     optional: false,
        //                     span: Some(span),
        //                 });
        //             }

        //             TokenKind::LeftParen => {
        //                 self.advance(); // Consume '('
        //                 let arguments = self.parse_arguments()?;
        //                 self.expect(TokenKind::RightParen)?;

        //                 let span = self.create_span_from_tokens();
        //                 expr = Node::CallExpression(CallExpression {
        //                     callee: Box::new(expr),
        //                     arguments,
        //                     span: Some(span),
        //                 });
        //             }

        //             TokenKind::Increment | TokenKind::Decrement => {
        //                 let operator = self.current_token_string();
        //                 let prefix = false;
        //                 self.advance(); // Consume operator

        //                 let span = self.create_span_from_tokens();
        //                 expr = Node::UpdateExpression(UpdateExpression {
        //                     operator,
        //                     argument: Box::new(expr),
        //                     prefix,
        //                     span: Some(span),
        //                 });
        //             }

        //             _ => break,
        //         }
        //     } else {
        //         break;
        //     }
        // }

        // Ok(expr)
    }

    fn is_unary_operator(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(
                token.kind,
                TokenKind::Plus | TokenKind::Minus | TokenKind::Increment | TokenKind::Decrement
            )
        } else {
            false
        }
    }

    fn is_multiplicative_operator(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(
                token.kind,
                TokenKind::Star | TokenKind::Slash | TokenKind::Percent
            )
        } else {
            false
        }
    }

    fn is_additive_operator(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(token.kind, TokenKind::Plus | TokenKind::Minus)
        } else {
            false
        }
    }

    fn is_relational_operator(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(
                token.kind,
                TokenKind::LessThan
                    | TokenKind::GreaterThan
                    | TokenKind::LessThanEqual
                    | TokenKind::GreaterThanEqual
            )
        } else {
            false
        }
    }

    fn is_assignment_operator(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(
                token.kind,
                TokenKind::Arrow // TODO: add those when we implement them
                                 // | TokenKind::PlusAssign | TokenKind::MinusAssign | TokenKind::Assign
                                 // TokenKind::MultiplyAssign | TokenKind::DivideAssign | TokenKind::ModuloAssign |
            )
        } else {
            false
        }
    }

    fn is_equality_operator(&self) -> bool {
        if let Some(token) = &self.current {
            matches!(
                token.kind,
                TokenKind::EqualEqual | TokenKind::NotEqual // | TokenKind::StrictEqual | TokenKind::NotStrictEqual // TODO: implement
            )
        } else {
            false
        }
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

    fn current_token_string(&self) -> String {
        if let Some(token) = &self.current {
            match &token.kind {
                TokenKind::Plus => "+".to_string(),
                TokenKind::Minus => "-".to_string(),
                TokenKind::Star => "*".to_string(),
                TokenKind::Slash => "/".to_string(),
                TokenKind::Percent => "%".to_string(),
                TokenKind::Equal => "=".to_string(),
                TokenKind::EqualEqual => "==".to_string(),
                TokenKind::LessThan => "<".to_string(),
                TokenKind::GreaterThan => ">".to_string(),
                TokenKind::LessThanEqual => "<=".to_string(),
                TokenKind::GreaterThanEqual => ">=".to_string(),
                TokenKind::Increment => "++".to_string(),
                TokenKind::Decrement => "--".to_string(),
                TokenKind::Identifier(name) => name.clone(),
                TokenKind::String(s) => s.clone(),
                TokenKind::Boolean(b) => b.to_string(),
                TokenKind::Number(n) => n.to_string(),
                TokenKind::Eof => "EOF".to_string(),
                _ => format!("{:?}", token.kind), // Fallback for other token kinds
            }
        } else {
            "EOF".to_string() // If no current token, return EOF
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
        self.current
            .as_ref()
            .map(|t| Position {
                line: t.start().line,
                column: t.start().column,
            })
    }

    /// Gets the previous token
    fn previous_position(&self) -> Option<Position> {
        self.previous
            .as_ref()
            .map(|t| Position {
                line: t.end().line,
                column: t.end().column,
            })
    }

    /// Get the current token
    fn current_token(&self) -> Option<&Token> {
        self.current.as_ref()
    }

    /// Create a span from the current tokens
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

    fn try_recover_from_error(&mut self, error: ParserError) -> bool {
        if !self.error_recovery.can_recover() {
            return false; // No recovery possible
        }

        self.error_recovery.record_error(error);

        let context = RecoveryContext::new(
            self.current.clone(),
            self.previous.clone(),
            self.context.clone(),
        );

        let strategy = context.determine_strategy();

        match strategy {
            RecoveryStrategy::SkipUntil(tokens) => {
                while !self.is_eof() {
                    if let Some(token) = self.current_token() {
                        if tokens
                            .iter()
                            .any(|t| format!("{:?}", token.kind).contains(t))
                        {
                            break;
                        }
                    }
                    self.advance();
                }
                true
            }

            RecoveryStrategy::SkipUntilStatement => {
                while !self.is_eof() {
                    if let Some(token) = self.current_token() {
                        match token.kind {
                            TokenKind::Semicolon | TokenKind::RightBrace => break,
                            _ => self.advance(),
                        }
                    } else {
                        break;
                    }
                }
                true
            }

            RecoveryStrategy::SkipUntilBlock => {
                while !self.is_eof() {
                    if let Some(token) = self.current_token() {
                        if matches!(token.kind, TokenKind::RightBrace) {
                            break;
                        }
                    }
                    self.advance();
                }
                true
            }

            RecoveryStrategy::SkipUntilFunction => {
                while !self.is_eof() {
                    if let Some(token) = self.current_token() {
                        match token.kind {
                            TokenKind::RightBrace | TokenKind::Semicolon => break,
                            _ => self.advance(),
                        }
                    } else {
                        break;
                    }
                }
                true
            }

            RecoveryStrategy::SkipUntilClass => {
                while !self.is_eof() {
                    if let Some(token) = self.current_token() {
                        if matches!(token.kind, TokenKind::RightBrace) {
                            break;
                        }
                    }
                    self.advance();
                }
                true
            }

            RecoveryStrategy::SkipUntilModule => {
                while !self.is_eof() {
                    let should_break = if let Some(token) = &self.current {
                        matches!(token.kind, TokenKind::RightBrace)
                            || matches!(token.kind, TokenKind::Keyword(ref kw) if kw == "import" || kw == "export")
                    } else {
                        false
                    };

                    if should_break {
                        break;
                    }
                    self.advance();
                }
                true
            }

            RecoveryStrategy::InsertToken(_) => {
                // Simplified: just advance
                self.advance();
                true
            }

            RecoveryStrategy::ReplaceToken(_) => {
                // Simplified: just advance
                self.advance();
                true
            }

            RecoveryStrategy::DeleteToken => {
                self.advance();
                true
            }

            RecoveryStrategy::NoRecovery => false,
        }
    }
}
