use lumi_lexer::{token::TokenKind, tokenize};

#[test]
fn test_simple_program() {
    let source = r#"
        let x -> 42;
        let y: str -> "hello";
        let z -> x + y;
    "#;

    let tokens = tokenize(source).unwrap();
    println!("Tokens: {:?}", tokens);
    // Should contain the expected tokens
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();

    assert!(token_kinds.contains(&&TokenKind::Keyword("let".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Keyword("str".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("x".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("y".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("z".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Number(42.0)));
    assert!(token_kinds.contains(&&TokenKind::String("hello".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Plus));
    assert!(token_kinds.contains(&&TokenKind::Arrow));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
}

#[test]
fn test_function_declaration() {
    let source = r#"
        fn add(a, b) {
            return a + b;
        }
    "#;

    let tokens = tokenize(source).unwrap();
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();

    assert!(token_kinds.contains(&&TokenKind::Keyword("fn".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("add".to_string())));
    assert!(token_kinds.contains(&&TokenKind::LeftParen));
    assert!(token_kinds.contains(&&TokenKind::Identifier("a".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Comma));
    assert!(token_kinds.contains(&&TokenKind::Identifier("b".to_string())));
    assert!(token_kinds.contains(&&TokenKind::RightParen));
    assert!(token_kinds.contains(&&TokenKind::LeftBrace));
    assert!(token_kinds.contains(&&TokenKind::Keyword("return".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Plus));
    assert!(token_kinds.contains(&&TokenKind::RightBrace));
}
#[test]
fn test_if_statement() {
    let source = r#"
        if (x > 0) {
            console.log("positive");
        } else {
            console.log("negative");
        }
    "#;

    let tokens = tokenize(source).unwrap();
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();

    assert!(token_kinds.contains(&&TokenKind::Keyword("if".to_string())));
    assert!(token_kinds.contains(&&TokenKind::LeftParen));
    assert!(token_kinds.contains(&&TokenKind::Identifier("x".to_string())));
    assert!(token_kinds.contains(&&TokenKind::GreaterThan));
    assert!(token_kinds.contains(&&TokenKind::Number(0.0)));
    assert!(token_kinds.contains(&&TokenKind::RightParen));
    assert!(token_kinds.contains(&&TokenKind::LeftBrace));
    assert!(token_kinds.contains(&&TokenKind::Keyword("else".to_string())));
    assert!(token_kinds.contains(&&TokenKind::RightBrace));
}

#[test]
fn test_for_loop() {
    let source = r#"
        for i -> 1 to 5 step 1 {
            console.log(i);
        }
    "#;

    let tokens = tokenize(source).unwrap();
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();

    assert!(token_kinds.contains(&&TokenKind::Keyword("for".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Keyword("to".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Keyword("step".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("i".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Number(1.0)));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
    assert!(token_kinds.contains(&&TokenKind::Arrow));
    assert!(token_kinds.contains(&&TokenKind::Number(5.0)));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
    assert!(token_kinds.contains(&&TokenKind::LeftBrace));
    assert!(token_kinds.contains(&&TokenKind::RightBrace));
}

#[test]
fn test_array_literal() {
    let source = r#"
        let arr -> [1, 2, 3, "hello", true];
    "#;

    let tokens = tokenize(source).unwrap();
    let token_kinds: Vec<&TokenKind> = tokens.iter().map(|t| &t.kind).collect();

    assert!(token_kinds.contains(&&TokenKind::Keyword("let".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Identifier("arr".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Arrow));
    assert!(token_kinds.contains(&&TokenKind::LeftBracket));
    assert!(token_kinds.contains(&&TokenKind::Number(1.0)));
    assert!(token_kinds.contains(&&TokenKind::Comma));
    assert!(token_kinds.contains(&&TokenKind::Number(2.0)));
    assert!(token_kinds.contains(&&TokenKind::Comma));
    assert!(token_kinds.contains(&&TokenKind::Number(3.0)));
    assert!(token_kinds.contains(&&TokenKind::Comma));
    assert!(token_kinds.contains(&&TokenKind::String("hello".to_string())));
    assert!(token_kinds.contains(&&TokenKind::Comma));
    assert!(token_kinds.contains(&&TokenKind::Boolean(true)));
    assert!(token_kinds.contains(&&TokenKind::RightBracket));
    assert!(token_kinds.contains(&&TokenKind::Semicolon));
}
