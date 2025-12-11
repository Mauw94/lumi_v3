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
    vm.execute(bytecode).unwrap();

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

#[test]
fn test_executing_simple_function_many_times() {
    let start = Instant::now();

    let source = r#"
        fn add(a, b) {
            return a + b;
        }

        let result: int -> 1;
        for i in 1 to 100000 step 1 {
            result = add(result, i);
        }

        print result;
    "#;

    let mut parser = Parser::new(source);
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(bytecode).unwrap();

    let duration = start.elapsed();
    println!("Time taken to execute function calls: {:?}", duration);

    assert!(
        duration.as_millis() < 1000,
        "Benchmark failed: took too long to execute function calls"
    );
}
