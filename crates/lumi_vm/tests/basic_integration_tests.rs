use lumi_bytecode::BytecodeGenerator;
use lumi_parser::Parser;
use lumi_vm::{Value, Vm};

#[test]
fn test_value_is_number() {
    let mut parser = Parser::new("42");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode);

    assert_eq!(vm.stack.values, vec![Value::Number(42.0)]);
}

#[test]
fn test_value_is_string() {
    let mut parser = Parser::new("\"Hello, World!\"");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode);

    assert_eq!(
        vm.stack.values,
        vec![Value::String("Hello, World!".to_string())]
    );
}

#[test]
fn test_value_is_boolean() {
    let mut parser = Parser::new("true");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode);

    assert_eq!(vm.stack.values, vec![Value::Boolean(true)]);
}

// TODO: do some benchmark tests
