use lumi_bytecode::BytecodeGenerator;
use lumi_parser::Parser;
use lumi_vm::{Value, Vm};

#[test]
fn test_plus_assign_expression() {
    let mut parser = Parser::new(
        r#"
        let x: int -> 2;
        x += 1;

        x;
    "#,
    );
    let ast = parser.parse().unwrap();
    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![Value::Number(3.0)]);
}

#[test]
fn test_minus_assign_expression() {
    let mut parser = Parser::new(
        r#"
        let x: int -> 2;
        x -= 1;

        x;
    "#,
    );
    let ast = parser.parse().unwrap();
    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![Value::Number(1.0)]);
}
