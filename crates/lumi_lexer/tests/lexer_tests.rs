use lumi_lexer::{Lexer, token::TokenKind, tokenize};

#[test]
fn test_lexer_creation() {
    let source = "let x: int -> 42;";
    let mut lexer = Lexer::new(source);

    // Test that lexer was created successfully
    let tokens = lexer.tokenize().unwrap();
    assert!(!tokens.is_empty());
    assert_eq!(tokens.last().unwrap().kind, TokenKind::Eof);
}

#[test]
fn test_identifier_tokenization() {
    let source = "hello world";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens.len(), 3); // identifier + identifier + EOF
    assert_eq!(tokens[0].kind, TokenKind::Identifier("hello".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::Identifier("world".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::Eof);
}

#[test]
fn test_string_tokenization() {
    let source = "\"hello world\"";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens.len(), 2); // string + EOF
    assert_eq!(tokens[0].kind, TokenKind::String("hello world".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::Eof);
}

#[test]
fn test_number_tokenization() {
    let source = "42";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens.len(), 2); // number + EOF
    assert_eq!(tokens[0].kind, TokenKind::Number(42.0));
    assert_eq!(tokens[1].kind, TokenKind::Eof);
}

#[test]
fn test_keyword_tokenization() {
    let keywords = vec![
        "let",
        "const",
        "fn",
        "if",
        "else",
        "return",
        "true",
        "false",
        "null",
        "undefined",
        "this",
        "super",
    ];

    for keyword in keywords {
        let tokens = tokenize(keyword).unwrap();
        assert_eq!(tokens.len(), 2); // keyword + EOF

        match &tokens[0].kind {
            TokenKind::Keyword(k) => {
                if k == "this" || k == "super" {
                    assert_eq!(k, keyword);
                } else {
                    assert_eq!(k, keyword);
                }
            }
            TokenKind::Boolean(true) => assert_eq!(keyword, "true"),
            TokenKind::Boolean(false) => assert_eq!(keyword, "false"),
            TokenKind::Null => assert_eq!(keyword, "null"),
            TokenKind::Undefined => assert_eq!(keyword, "undefined"),
            _ => panic!("Expected keyword token for '{}'", keyword),
        }
    }
}

#[test]
fn test_operator_tokenization() {
    let operators = vec![
        ("+", TokenKind::Plus),
        ("-", TokenKind::Minus),
        ("*", TokenKind::Star),
        ("/", TokenKind::Slash),
        ("=", TokenKind::Assign),
        ("==", TokenKind::Equal),
        ("->", TokenKind::Arrow),
        ("!=", TokenKind::NotEqual),
        ("++", TokenKind::Increment),
        ("--", TokenKind::Decrement),
    ];

    for (op_str, expected_kind) in operators {
        let tokens = tokenize(op_str).unwrap();
        assert_eq!(tokens.len(), 2); // operator + EOF
        assert_eq!(tokens[0].kind, expected_kind);
    }
}

#[test]
fn test_comment_tokenization() {
    // Line comment
    let source = "// this is a comment";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens.len(), 2); // comment + EOF
    assert_eq!(
        tokens[0].kind,
        TokenKind::Comment(" this is a comment".to_string())
    );

    // Block comment
    let source = "/* this is a block comment */";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens.len(), 2); // comment + EOF
    assert_eq!(
        tokens[0].kind,
        TokenKind::Comment(" this is a block comment ".to_string())
    );
}

#[test]
fn test_variable_declaration() {
    let source = "let x: int -> 42;";
    let tokens = tokenize(source).unwrap();

    assert_eq!(tokens.len(), 8); // let + identifier + colon + identifier + assign + number + semicolon + EOF
    assert_eq!(tokens[0].kind, TokenKind::Keyword("let".to_string()));
    assert_eq!(tokens[1].kind, TokenKind::Identifier("x".to_string()));
    assert_eq!(tokens[2].kind, TokenKind::Colon);
    assert_eq!(tokens[3].kind, TokenKind::Keyword("int".to_string()));
    assert_eq!(tokens[4].kind, TokenKind::Arrow);
    assert_eq!(tokens[5].kind, TokenKind::Number(42.0));
}
