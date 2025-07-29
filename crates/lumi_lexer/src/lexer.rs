use crate::{
    error::LexError,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexError> {
        let mut tokens = Vec::new();

        while self.pos < self.source.len() {
            let start_line = self.line;
            let start_col = self.column;

            let token = self.next_token()?;

            if matches!(token.kind, TokenKind::Eof) {
                tokens.push(token);
                break;
            }

            tokens.push(token);

            self.update_positions(start_line, start_col);
        }

        if tokens.is_empty() || !matches!(tokens.last().unwrap().kind, TokenKind::Eof) {
            tokens.push(Token::with_positions(
                TokenKind::Eof,
                self.line,
                self.column,
                self.line,
                self.column,
            ));
        }

        Ok(tokens)
    }

    pub fn next_token(&mut self) -> Result<Token, LexError> {
        self.skip_whitespace();

        if self.pos >= self.source.len() {
            return Ok(Token::with_positions(
                TokenKind::Eof,
                self.line,
                self.column,
                self.line,
                self.column,
            ));
        }

        let start_line = self.line;
        let start_col = self.column;
        let c = self.source[self.pos];

        let token_kind = if c.is_ascii_alphabetic() || c == '_' || c == '$' || !c.is_ascii() {
            self.read_identifier_or_keyword()?
        } else if c.is_ascii_digit() {
            self.read_number()?
        } else if c == '"' || c == '\'' {
            self.read_string()?
        } else if c == '/' {
            if self.peek_char(1) == Some('/') {
                self.read_line_comment()?
            } else if self.peek_char(1) == Some('*') {
                self.read_block_comment()?
            } else {
                self.read_operator()?
            }
        } else {
            self.read_operator()?
        };

        let end_line = self.line;
        let end_col = self.column;

        Ok(Token::with_positions(
            token_kind, start_line, start_col, end_line, end_col,
        ))
    }

    /// Read an identifier or keyword
    fn read_identifier_or_keyword(&mut self) -> Result<TokenKind, LexError> {
        let mut identifier = String::new();

        while self.pos < self.source.len() {
            let c = self.source[self.pos];
            // TODO: Support Unicode identifiers
            if c.is_alphanumeric() || c.is_alphabetic() || !c.is_ascii() {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }

        // Check if it's a keyword
        match identifier.as_str() {
            "true" => Ok(TokenKind::Boolean(true)),
            "false" => Ok(TokenKind::Boolean(false)),
            "null" => Ok(TokenKind::Null),
            "undefined" => Ok(TokenKind::Undefined),
            "this" => Ok(TokenKind::Keyword("this".to_string())),
            "super" => Ok(TokenKind::Keyword("super".to_string())),
            // keywords
            "let" | "const" | "var" | "function" | "if" | "else" | "return" | "async" | "await"
            | "yield" | "import" | "export" | "new" | "class" | "extends" | "static" | "get"
            | "set" | "try" | "catch" | "finally" | "throw" | "break" | "continue" | "switch"
            | "case" | "default" | "for" | "while" | "do" | "in" | "of" | "with" | "delete"
            | "instanceof" | "typeof" | "void" | "debugger" | "enum" | "interface" | "package"
            | "private" | "protected" | "public" | "implements" | "abstract" | "boolean"
            | "byte" | "char" | "double" | "final" | "float" | "goto" | "int" | "long"
            | "native" | "short" | "synchronized" | "throws" | "transient" | "volatile" => {
                Ok(TokenKind::Keyword(identifier))
            }
            _ => Ok(TokenKind::Identifier(identifier)),
        }
    }

    /// Read a number literal
    fn read_number(&mut self) -> Result<TokenKind, LexError> {
        let mut number = String::new();
        let mut is_hex = false;
        let mut is_binary = false;
        let mut is_octal = false;

        // Check for hex, binary, or octal
        if self.source[self.pos] == '0' && self.pos + 1 < self.source.len() {
            match self.source[self.pos + 1] {
                'x' | 'X' => {
                    is_hex = true;
                    number.push('0');
                    number.push(self.source[self.pos + 1]);
                    self.advance();
                    self.advance();
                }
                'b' | 'B' => {
                    is_binary = true;
                    number.push('0');
                    number.push(self.source[self.pos + 1]);
                    self.advance();
                    self.advance();
                }
                'o' | 'O' => {
                    is_octal = true;
                    number.push('0');
                    number.push(self.source[self.pos + 1]);
                    self.advance();
                    self.advance();
                }
                _ => {}
            }
        }

        while self.pos < self.source.len() {
            let c = self.source[self.pos];

            if is_hex {
                if c.is_ascii_hexdigit() {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            } else if is_binary {
                if c == '0' || c == '1' {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            } else if is_octal {
                if c >= '0' && c <= '7' {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            } else {
                if c.is_ascii_digit() || c == '.' || c == 'e' || c == 'E' || c == '+' || c == '-' {
                    number.push(c);
                    self.advance();
                } else {
                    break;
                }
            }
        }

        // TODO: implement later for LNum
        // Check for BigInt suffix
        // if self.pos < self.source.len() && self.source[self.pos] == 'n' {
        //     number.push('n');
        //     self.advance();
        //     return Ok(TokenKind::BigInt(number));
        // }

        // Parse as number
        if is_hex {
            // Parse hex number
            match u64::from_str_radix(&number[2..], 16) {
                Ok(n) => Ok(TokenKind::Number(n as f64)),
                Err(_) => Err(LexError::InvalidNumber(number)),
            }
        } else if is_binary {
            // Parse binary number
            match u64::from_str_radix(&number[2..], 2) {
                Ok(n) => Ok(TokenKind::Number(n as f64)),
                Err(_) => Err(LexError::InvalidNumber(number)),
            }
        } else if is_octal {
            // Parse octal number
            match u64::from_str_radix(&number[2..], 8) {
                Ok(n) => Ok(TokenKind::Number(n as f64)),
                Err(_) => Err(LexError::InvalidNumber(number)),
            }
        } else {
            // Parse decimal number
            match number.parse::<f64>() {
                Ok(n) => Ok(TokenKind::Number(n)),
                Err(_) => Err(LexError::InvalidNumber(number)),
            }
        }
    }

    /// Read a string literal
    fn read_string(&mut self) -> Result<TokenKind, LexError> {
        let quote = self.source[self.pos];
        self.advance(); // Skip opening quote

        let mut string = String::new();

        let mut found_closing_quote = false;

        while self.pos < self.source.len() {
            let c = self.source[self.pos];

            if c == quote {
                self.advance(); // Skip closing quote
                found_closing_quote = true;
                break;
            } else if c == '\\' {
                self.advance(); // Skip backslash
                if self.pos < self.source.len() {
                    let escaped = self.source[self.pos];
                    match escaped {
                        'n' => string.push('\n'),
                        't' => string.push('\t'),
                        'r' => string.push('\r'),
                        '\\' => string.push('\\'),
                        '"' => string.push('"'),
                        '\'' => string.push('\''),
                        _ => string.push(escaped),
                    }
                    self.advance();
                }
            } else {
                string.push(c);
                self.advance();
            }
        }

        // Check if we reached the end without finding a closing quote
        if !found_closing_quote {
            return Err(LexError::UnterminatedString);
        }

        Ok(TokenKind::String(string))
    }

    /// Read a line comment
    fn read_line_comment(&mut self) -> Result<TokenKind, LexError> {
        self.advance(); // Skip first '/'
        self.advance(); // Skip second '/'

        let mut comment = String::new();

        while self.pos < self.source.len() {
            let c = self.source[self.pos];
            if c == '\n' {
                break;
            }
            comment.push(c);
            self.advance();
        }

        Ok(TokenKind::Comment(comment))
    }

    /// Read a block comment
    fn read_block_comment(&mut self) -> Result<TokenKind, LexError> {
        self.advance(); // Skip '/'
        self.advance(); // Skip '*'

        let mut comment = String::new();

        let mut found_closing_comment = false;

        while self.pos < self.source.len() {
            let c = self.source[self.pos];

            if c == '*' && self.peek_char(1) == Some('/') {
                self.advance(); // Skip '*'
                self.advance(); // Skip '/'
                found_closing_comment = true;
                break;
            }

            comment.push(c);
            self.advance();
        }

        // Check if we reached the end without finding a closing comment
        if !found_closing_comment {
            return Err(LexError::UnterminatedComment);
        }

        Ok(TokenKind::Comment(comment))
    }

    /// Read an operator or symbol
    fn read_operator(&mut self) -> Result<TokenKind, LexError> {
        let c = self.source[self.pos];

        // Check for two-character operators
        if self.pos + 1 < self.source.len() {
            let next_c = self.source[self.pos + 1];
            let two_char_op = format!("{}{}", c, next_c);

            match two_char_op.as_str() {
                "==" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::Equal);
                }
                "!=" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::NotEqual);
                }
                "<=" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::LessThanEqual);
                }
                ">=" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::GreaterThanEqual);
                }
                "++" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::Increment);
                }
                "--" => {
                    self.advance();
                    self.advance();
                    return Ok(TokenKind::Decrement);
                }
                // "&&" => {
                //     self.advance();
                //     self.advance();
                //     return Ok(TokenKind::LogicalAnd);
                // }
                // "||" => {
                //     self.advance();
                //     self.advance();
                //     return Ok(TokenKind::LogicalOr);
                // }
                // "=>" => {
                //     self.advance();
                //     self.advance();
                //     return Ok(TokenKind::Arrow);
                // }
                // "??" => {
                //     self.advance();
                //     self.advance();
                //     return Ok(TokenKind::NullishCoalescing);
                // }
                _ => {}
            }
        }

        // Single character operators
        match c {
            '(' => {
                self.advance();
                Ok(TokenKind::LeftParen)
            }
            ')' => {
                self.advance();
                Ok(TokenKind::RightParen)
            }
            '{' => {
                self.advance();
                Ok(TokenKind::LeftBrace)
            }
            '}' => {
                self.advance();
                Ok(TokenKind::RightBrace)
            }
            '[' => {
                self.advance();
                Ok(TokenKind::LeftBracket)
            }
            ']' => {
                self.advance();
                Ok(TokenKind::RightBracket)
            }
            '.' => {
                self.advance();
                Ok(TokenKind::Dot)
            }
            ';' => {
                self.advance();
                Ok(TokenKind::Semicolon)
            }
            ',' => {
                self.advance();
                Ok(TokenKind::Comma)
            }
            ':' => {
                self.advance();
                Ok(TokenKind::Colon)
            }
            '?' => {
                self.advance();
                Ok(TokenKind::Question)
            }
            '!' => {
                self.advance();
                Ok(TokenKind::Exclamation)
            }
            // '~' => {
            //     self.advance();
            //     Ok(TokenKind::Tilde)
            // }
            '=' => {
                self.advance();
                Ok(TokenKind::Assign)
            }
            '+' => {
                self.advance();
                Ok(TokenKind::Plus)
            }
            '-' => {
                self.advance();
                Ok(TokenKind::Minus)
            }
            '*' => {
                self.advance();
                Ok(TokenKind::Star)
            }
            '/' => {
                self.advance();
                Ok(TokenKind::Slash)
            }
            '%' => {
                self.advance();
                Ok(TokenKind::Percent)
            }
            '<' => {
                self.advance();
                Ok(TokenKind::LessThan)
            }
            '>' => {
                self.advance();
                Ok(TokenKind::GreaterThan)
            }
            // '&' => {
            //     self.advance();
            //     Ok(TokenKind::BitwiseAnd)
            // }
            // '|' => {
            //     self.advance();
            //     Ok(TokenKind::BitwiseOr)
            // }
            // '^' => {
            //     self.advance();
            //     Ok(TokenKind::BitwiseXor)
            // }
            _ => Err(LexError::UnexpectedCharacter(c)),
        }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.source.len() {
            let c = self.source[self.pos];
            if c.is_whitespace() {
                if c == '\n' {
                    self.line += 1;
                    self.column = 1;
                } else {
                    self.column += 1;
                }
                self.advance();
            } else {
                break;
            }
        }
    }

    fn advance(&mut self) {
        if self.pos < self.source.len() {
            self.pos += 1;
            self.column += 1;
        }
    }

    fn peek_char(&self, offset: usize) -> Option<char> {
        if self.pos + offset < self.source.len() {
            Some(self.source[self.pos + offset])
        } else {
            None
        }
    }

    fn update_positions(&mut self, start_line: usize, start_col: usize) {
        self.line = start_line;
        self.column = start_col;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identifier() {
        let mut lexer = Lexer::new("hello world");
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(tokens[0].kind, TokenKind::Identifier("hello".to_string()));
    }
}
