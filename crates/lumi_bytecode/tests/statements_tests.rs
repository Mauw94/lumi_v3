use lumi_bytecode::{BytecodeGenerator, Constant};
use lumi_parser::Parser;

#[test]
fn test_print_statement() {
    let mut parser = Parser::new("let x: int -> 5; print x;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    assert_eq!(bytecode.constants, vec![Constant::Number(5.0)]);
}
