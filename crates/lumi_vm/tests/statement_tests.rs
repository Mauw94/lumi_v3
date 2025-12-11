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
    vm.execute(bytecode).unwrap();

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
    vm.execute(bytecode).unwrap();

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
    vm.execute(bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![Value::String("not ok".to_string())]);
}

#[test]
fn test_for_statement_with_function_call() {
    let mut parser = Parser::new(
        r#"
        fn printSomething(x) {
            print x;
        }
        for i in 1 to 10 step 1 {
            printSomething(i);
        }
    "#,
    );
    let ast = parser.parse().unwrap();
    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![]);
}

#[test]
fn test_calling_fn_twice() {
    let mut parser = Parser::new(
        r#"
        fn printSomething(x) {
            print x;
        }
        printSomething(2);
        printSomething(5);
    "#,
    );
    let ast = parser.parse().unwrap();
    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(bytecode).unwrap();

    assert_eq!(
        vm.stack.values,
        vec![Value::Number(2.0), Value::Number(5.0)]
    );
}
