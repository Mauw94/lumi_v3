use lumi_bytecode::{BytecodeGenerator, Constant};
use lumi_parser::Parser;

#[test]
fn store_empty_variable_test() {
    let mut parser = Parser::new("let x: int;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    assert_eq!(bytecode.constants, vec![Constant::Number(0.0)]); // Default value for int should be in the constant pool.
}

#[test]
fn store_variable_test() {
    let mut parser = Parser::new("let x: int -> 42; x;");
    let ast = parser.parse().unwrap();

    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    assert_eq!(bytecode.instructions.len(), 3); // Should have instructions for variable declaration and loading
    assert_eq!(bytecode.constants.len(), 1); // Should have one constant for the number 42
}
