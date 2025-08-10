use lumi_bytecode::{BytecodeGenerator, Constant, FunctionObj, Instruction};
use lumi_parser::Parser;

#[test]
fn test_call_expression() {
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

    assert_eq!(
        bytecode.constants,
        vec![
            Constant::Function(FunctionObj {
                name: Some("test".to_string()),
                arity: 2,
                chunk: vec![
                    Instruction::LoadVar(0),
                    Instruction::LoadVar(1),
                    Instruction::Add,
                    Instruction::Return
                ],
                constants: vec![]
            }),
            Constant::Number(1.0),
            Constant::Number(2.0)
        ]
    );
}
