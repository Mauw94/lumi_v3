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
    vm.execute(bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![]); // No value should be on the stack since x is not initialized
}

#[test]
fn store_and_load_variable_test() {
    let mut parser = Parser::new("let x: int -> 42; x;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![Value::Number(42.0)]); // The variable x should hold the value 42.0
}

#[test]
fn store_and_load_multiple_variables_test() {
    let mut parser = Parser::new("let x: int -> 42; let y: int -> 58; x; y;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(bytecode).unwrap();

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
    vm.execute(bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![Value::Number(100.0)]); // The result of x + y should be 100.0
}

#[test]
fn subtract_two_number_variables() {
    let mut parser = Parser::new("let x: int -> 42; let y: int -> 58; x - y;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![Value::Number(-16.0)]); // The result of x + y should be 100.0
}

#[test]
fn multiply_two_number_variables() {
    let mut parser = Parser::new("let x: int -> 42; let y: int -> 58; x * y;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![Value::Number(2436.0)]); // The result of x * y should be 2436.0
}

#[test]
fn divide_two_number_variables() {
    let mut parser = Parser::new("let x: int -> 84; let y: int -> 2; x / y;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![Value::Number(42.0)]); // The result of x / y should be 42.0
}

#[test]
fn test_reassigning_variables() {
    let source = r#"
        let x: int -> 42;
        let y -> 2;
        x -> 22;
        y -> 3;
        x * y;
    "#;
    let mut parser = Parser::new(source);
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(bytecode).unwrap();

    // The result of x * y should be 66.0.
    assert_eq!(vm.stack.values, vec![Value::Number(66.0)]);
}

#[test]
fn test_declaring_reassinging_variables() {
    let source = r#"
        let x: int -> 42;
        let y -> 2;
        x -> 22;
        let z -> 5;
        x + z;
        
    "#;
    let mut parser = Parser::new(source);
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(bytecode).unwrap();

    // The result of z and x + y should be 22.0 + 5.0 = 27.0
    assert_eq!(vm.stack.values, vec![Value::Number(27.0)]);
}
