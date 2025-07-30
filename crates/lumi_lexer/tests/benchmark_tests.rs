use std::time::Instant;

use lumi_lexer::tokenize;

#[test]
fn test_lexer_performance_simple() {
    let source = "let x: int -> 42;";
    let start = Instant::now();

    for _ in 0..1000 {
        let _tokens = tokenize(source).unwrap();
    }

    let duration = start.elapsed();
    println!("Simple lexing took: {:?} for 1000 iterations", duration);

    // Should complete in reasonable time (less than 1 second)
    assert!(duration.as_millis() < 1000);
}

#[test]
fn test_lexer_performance_complex() {
    let source = r#"
        fn fibonacci(n) {
            if (n <= 1) {
                return n;
            }
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
        
        let result = fibonacci(10);
        console.log('Fibonacci of 10 is ${result}');
    "#;

    let start = Instant::now();

    for _ in 0..100 {
        let _tokens = tokenize(source).unwrap();
    }

    let duration = start.elapsed();
    println!("Complex lexing took: {:?} for 100 iterations", duration);

    // Should complete in reasonable time (less than 1 second)
    assert!(duration.as_millis() < 1000);
}

#[test]
fn test_lexer_performance_large_source() {
    let mut source = String::new();
    for i in 0..1000 {
        source.push_str(&format!("let const = {} = {};", i, i));
    }

    let start = Instant::now();
    let tokens = tokenize(&source).unwrap();
    let duration = start.elapsed();

    println!(
        "Large source lexing took: {:?} for {} tokens",
        duration,
        tokens.len()
    );

    // Should have many tokens
    assert!(tokens.len() > 3000);

    // Should complete in reasonable time (less than 1 second)
    assert!(duration.as_millis() < 1000);
}

#[test]
fn test_lexer_memory_usage() {
    let source = "let x: int -> 42;";
    let tokens = tokenize(source).unwrap();

    let token_count = tokens.len();
    let estimated_memory = token_count * std::mem::size_of::<lumi_lexer::Token>();

    println!(
        "Estimated memory usage: {} bytes for {} tokens",
        estimated_memory, token_count
    );

    // Should be reasonable memory usage (less than 1MB for small source)
    assert!(estimated_memory < 1_000_000);
}
