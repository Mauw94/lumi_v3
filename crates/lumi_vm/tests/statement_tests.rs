use lumi_bytecode::BytecodeGenerator;
use lumi_parser::Parser;
use lumi_vm::{Value, Vm};

#[test]
fn test_print_statement() {
    let mut parser = Parser::new("let x: int -> 42; print x;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode);

    assert_eq!(vm.stack.values, vec![Value::Number(42.0)]); // The variable x should hold the value 42.0
}

#[test]
fn test_if_jump_true_statement() {
    let mut parser = Parser::new(
        r#"
        let x -> 42;
        if (x > 30) {
            print "ok";
        } else {
            print "not ok";
        }
    "#,
    );
    let ast = parser.parse().unwrap();
    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode);

    assert_eq!(vm.stack.values, vec![Value::String("ok".to_string())]);
}

#[test]
fn test_if_jump_false_statement() {
    let mut parser = Parser::new(
        r#"
        let x -> 42;
        if (x < 30) {
            print "ok";
        } else {
            print "not ok";
        }
    "#,
    );
    let ast = parser.parse().unwrap();
    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode);

    assert_eq!(vm.stack.values, vec![Value::String("not ok".to_string())]);
}