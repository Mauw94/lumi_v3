mod generator;
mod instruction;

pub use generator::BytecodeGenerator;
pub use instruction::{Constant, ConstantPool, Instruction};

// TODO: error handling
pub fn generate(source: &str)  {
    let mut parser = lumi_parser::Parser::new(source);
    let ast = parser.parse().unwrap();
    let mut generator = generator::BytecodeGenerator::new();
    generator.generate(&ast);
}