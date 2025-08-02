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
