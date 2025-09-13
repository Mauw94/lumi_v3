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
    vm.execute(&bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![Value::Number(42.0)]);
}

#[test]
fn test_value_is_string() {
    let mut parser = Parser::new("\"Hello, World!\"");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode).unwrap();

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
    vm.execute(&bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![Value::Boolean(true)]);
}

#[test]
fn test_variables_and_print() {
    let mut parser = Parser::new(
        r#"
        let x: int -> 5;
        let y: int -> 15;

        print x * y;
    "#,
    );
    let ast = parser.parse().unwrap();
    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![Value::Number(75.0)]);
}

#[test]
fn test_fn_and_call_fn() {
    let mut parser = Parser::new(
        r#"
        fn test(x, y) {
            x + y;
        }

        test(1, 2);
    "#,
    );
    let ast = parser.parse().unwrap();
    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![Value::Number(3.0)]);
}

#[test]
fn test_fn_with_passed_vars_and_call_fn() {
    let mut parser = Parser::new(
        r#"
        fn test(x, y) {
            x + y;
        }

        let x: int -> 2;
        let y -> 5;
        test(x, y);
    "#,
    );
    let ast = parser.parse().unwrap();
    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![Value::Number(7.0)]);
}

#[test]
fn test_fn_with_return_statement() {
    let mut parser = Parser::new(
        r#"
        fn test(n) {
            return n + 1;
        }
        
        print test(5);
    "#,
    );
    let ast = parser.parse().unwrap();
    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    let mut vm = Vm::new();
    vm.execute(&bytecode).unwrap();

    assert_eq!(vm.stack.values, vec![Value::Number(6.0)]);
}

// TODO: doesn't work
// #[test]
// fn test_fn_fib() {
//     let mut parser = Parser::new(
//         r#"
//         fn fib(n) {
//             if n < 2 {
//                 return 1;
//             } else {
//                 return fib(n - 1) + fib(n - 2);
//             }
//         }

//         print fib(7);
//     "#,
//     );
//     let ast = parser.parse().unwrap();
//     let mut bytecode_generator = BytecodeGenerator::new();
//     let bytecode = bytecode_generator.generate(&ast);

//     println!("{:?}", bytecode.instructions);
//     let mut vm = Vm::new();
//     vm.execute(&bytecode).unwrap();

//     assert_eq!(vm.stack.values, vec![Value::Number(13.0)]);
// }
