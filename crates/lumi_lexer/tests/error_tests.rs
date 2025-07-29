use lumi_lexer::{error::LexError, tokenize};

#[test]
fn test_unterminated_string() {
    let source = "\"hello world";
    let result = tokenize(source);

    assert!(result.is_err());
    match result.unwrap_err() {
        LexError::UnterminatedString => {}
        _ => panic!("Expected UnterminatedString error"),
    }
}

#[test]
fn test_unterminated_comment() {
    let source = "/* this is a comment";
    let result = tokenize(source);

    assert!(result.is_err());
    match result.unwrap_err() {
        LexError::UnterminatedComment => {},
        _ => panic!("Expected unterminatedComment error")
    }
}