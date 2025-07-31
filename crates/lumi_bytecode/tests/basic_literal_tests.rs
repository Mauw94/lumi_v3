use lumi_bytecode::{BytecodeGenerator, Constant, Instruction};
use lumi_parser::Parser;

// TODO: extend parser first, now this will fail if the parser does not support literals
#[test]
fn test_generate_number_literal() {
    let mut parser = Parser::new("42");
    let ast = parser.parse().unwrap();
    println!("{:#?}", ast);
    let mut generator = BytecodeGenerator::new();
    generator.generate(&ast);
    assert_eq!(generator.instructions, vec![Instruction::PushConst(0)]);
    assert_eq!(generator.constants.values, vec![Constant::Number(42.0)]);
}