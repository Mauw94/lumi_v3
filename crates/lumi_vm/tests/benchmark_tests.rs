use std::time::Instant;

use lumi_bytecode::BytecodeGenerator;
use lumi_parser::Parser;
use lumi_vm::Vm;

#[test]
fn test_executing_many_variable_declarations() {
    let start = Instant::now();

    let mut source = String::new();
    for i in 0..1000 {
        source.push_str(&format!("let {}: int -> {};\n", i, i));
    }

    let mut parser = Parser::new(&source);
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode);

    let duration = start.elapsed();
    println!(
        "Time taken to execute variable declarations: {:?}",
        duration
    );

    assert!(
        duration.as_millis() < 1000,
        "Benchmark failed: took too long to execute variable declarations"
    );
}
