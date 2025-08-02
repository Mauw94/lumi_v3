use lumi_bytecode::BytecodeGenerator;
use lumi_parser::Parser;
use lumi_vm::{Value, Vm};

#[test]
fn store_empty_variable_test() {
    let mut parser = Parser::new("let x: int;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode);

    assert_eq!(vm.stack.values, vec![]); // No value should be on the stack since x is not initialized
}

#[test]
fn store_and_load_variable_test() {
    let mut parser = Parser::new("let x: int -> 42; x;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode);

    assert_eq!(vm.stack.values, vec![Value::Number(42.0)]); // The variable x should hold the value 42.0
}

#[test]
fn store_and_load_multiple_variables_test() {
    let mut parser = Parser::new("let x: int -> 42; let y: int -> 58; x; y;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode);

    assert_eq!(
        vm.stack.values,
        vec![Value::Number(42.0), Value::Number(58.0)]
    );
}

#[test]
fn add_two_number_variables() {
    let mut parser = Parser::new("let x: int -> 42; let y: int -> 58; x + y;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode);

    assert_eq!(vm.stack.values, vec![Value::Number(100.0)]); // The result of x + y should be 100.0
}

#[test]
fn subtract_two_number_variables() {
    let mut parser = Parser::new("let x: int -> 42; let y: int -> 58; x - y;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode);

    assert_eq!(vm.stack.values, vec![Value::Number(-16.0)]); // The result of x + y should be 100.0
}

#[test]
fn multiply_two_number_variables() {
    let mut parser = Parser::new("let x: int -> 42; let y: int -> 58; x * y;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode);

    assert_eq!(vm.stack.values, vec![Value::Number(2436.0)]); // The result of x * y should be 2436.0
}

#[test]
fn divide_two_number_variables() {
    let mut parser = Parser::new("let x: int -> 84; let y: int -> 2; x / y;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode);

    assert_eq!(vm.stack.values, vec![Value::Number(42.0)]); // The result of x / y should be 42.0
}
