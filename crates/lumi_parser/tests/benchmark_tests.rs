use std::time::Instant;

use lumi_parser::Parser;

#[test]
fn test_parser_performance_simple() {
    let mut parser = Parser::new("let x: int -> 42;");
    let start = Instant::now();

    for _ in 0..1000 {
        let result = parser.parse();
        assert!(result.is_ok());
    }

    let duration = start.elapsed();
    println!("Simple parsing took: {:?} for 1000 iterations", duration);

    // Should complete in reasonable time (less than 1 second)
    assert!(duration.as_millis() < 1000);
}

#[test]
fn test_parser_performance_complex() {
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
    let mut parser = Parser::new(source);

    for _ in 0..1000 {
        let result = parser.parse();
        assert!(result.is_ok());
    }

    let duration = start.elapsed();
    println!("Complex parsing took: {:?} for 1000 iterations", duration);

    // Should complete in reasonable time (less than 1 second)
    assert!(duration.as_millis() < 1000);
}

#[test]
fn test_parser_performance_large_source() {
    let mut source = String::new();
    for i in 0..1000 {
        source.push_str(&format!("let const = {} = {};", i, i));
    }

    let start = Instant::now();
    let mut parser = Parser::new(&source);
    let result = parser.parse();
    assert!(result.is_ok());

    let duration = start.elapsed();
    println!(
        "Large source parsing took: {:?} for {} tokens",
        duration,
        source.len()
    );

    // Should complete in reasonable time (less than 1 second)
    assert!(duration.as_millis() < 1000);
}
