use lumi_bytecode::{BytecodeGenerator, Constant, Instruction};
use lumi_parser::Parser;

#[test]
fn test_generate_number_literal() {
    let mut parser = Parser::new("42");
    let ast = parser.parse().unwrap();

    let mut generator = BytecodeGenerator::new();
    generator.generate(&ast);

    assert_eq!(generator.instructions, vec![Instruction::PushConst(0)]);
    assert_eq!(generator.constants.values, vec![Constant::Number(42.0)]);
}

#[test]
fn test_generate_string_literal() {
    let mut parser = Parser::new("\"hello world\"");
    let ast = parser.parse().unwrap();

    let mut generator = BytecodeGenerator::new();
    generator.generate(&ast);

    assert_eq!(generator.instructions, vec![Instruction::PushConst(0)]);
    assert_eq!(
        generator.constants.values,
        vec![Constant::String("hello world".to_string())]
    );
}
