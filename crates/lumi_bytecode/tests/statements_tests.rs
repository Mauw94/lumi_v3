use lumi_bytecode::{BytecodeGenerator, Constant, FunctionObj, Instruction};
use lumi_parser::Parser;

#[test]
fn test_print_statement() {
    let mut parser = Parser::new("let x: int -> 5; print x;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    assert_eq!(bytecode.constants, vec![Constant::Number(5.0)]);
}

#[test]
fn test_simple_print_statement() {
    let mut parser = Parser::new("print 1 + 2;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    assert_eq!(
        bytecode.constants,
        vec![Constant::Number(1.0), Constant::Number(2.0)]
    );
}

#[test]
fn test_if_statement() {
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

    assert_eq!(
        bytecode.constants,
        vec![
            Constant::Number(42.0),
            Constant::Number(30.0),
            Constant::String("ok".to_string()),
            Constant::String("not ok".to_string())
        ]
    );
}

#[test]
fn test_fn_statement() {
    let mut parser = Parser::new(
        r#"
        fn test(x, y) {
            x + y;
        }
    "#,
    );
    let ast = parser.parse().unwrap();
    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    assert_eq!(
        bytecode.constants,
        vec![Constant::Function(FunctionObj {
            name: Some("test".to_string()),
            arity: 2,
            instructions: vec![
                Instruction::LoadVar(0),
                Instruction::LoadVar(1),
                Instruction::Add,
                Instruction::Return
            ],
            constants: vec![]
        })]
    );
}

#[test]
fn test_for_statement() {
    let mut parser = Parser::new(
        r#"
        for i in 1 to 5 step 2 {
            print i;
        }
    "#,
    );
    let ast = parser.parse().unwrap();
    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    assert_eq!(
        bytecode.constants,
        vec![
            Constant::Number(1.0),
            Constant::Number(5.0),
            Constant::Number(2.0)
        ]
    );
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

    assert_eq!(
        bytecode.constants,
        vec![
            Constant::Function(FunctionObj {
                name: Some("printSomething".to_string()),
                arity: 1,
                instructions: vec![
                    Instruction::LoadVar(0),
                    Instruction::Print,
                    Instruction::Return
                ],
                constants: vec![]
            }),
            Constant::Number(1.0),
            Constant::Number(10.0),
            Constant::Number(1.0)
        ]
    );
}
