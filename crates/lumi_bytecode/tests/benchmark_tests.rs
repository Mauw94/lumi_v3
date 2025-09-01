use std::time::Instant;

use lumi_bytecode::BytecodeGenerator;
use lumi_parser::Parser;

#[test]
fn test_declaring_many_variables() {
    let start = Instant::now();

    let mut source = String::new();
    for i in 0..1000 {
        source.push_str(&format!("let {}: int -> {};\n", i, i));
    }

    let mut parser = Parser::new(&source);
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    bytecode_generator.generate(&ast);

    let duration = start.elapsed();
    println!("Time taken to declare variables: {:?}", duration);

    // Should complete within a reasonable time frame
    assert!(
        duration.as_millis() < 1000,
        "Benchmark failed: took too long to declare variables"
    );
}

#[test]
fn test_declaring_many_nested_scopes() {
    let start = Instant::now();

    let mut source = String::new();
    source.push_str("let x: int -> 0;\n");
    for i in 0..100 {
        source.push_str("{\n");
        source.push_str(&format!("let x: int -> {};\n", i));
    }
    for _ in 0..100 {
        source.push_str("}\n");
    }

    let mut parser = Parser::new(&source);
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    bytecode_generator.generate(&ast);

    let duration = start.elapsed();
    println!("Time taken to declare nested scopes: {:?}", duration);

    // Should complete within a reasonable time frame
    assert!(
        duration.as_millis() < 1000,
        "Benchmark failed: took too long to declare nested scopes"
    );
}

#[test]
fn test_long_for_loop() {
    let start = Instant::now();

    let mut parser = Parser::new(
        r#"
        for i in 1 to 1000000 step 1 {
            print i;
        }
        "#,
    );

    let ast = parser.parse().unwrap();
    let mut bytecode_generator = BytecodeGenerator::new();
    bytecode_generator.generate(&ast);

    let duration = start.elapsed();
    println!("Time takent to generate long for loop: {:?}", duration);

    // Should complete within a reasonable time frame
    assert!(
        duration.as_millis() < 1000,
        "Benchmark failed: took too long to generate long for loop"
    );
}
