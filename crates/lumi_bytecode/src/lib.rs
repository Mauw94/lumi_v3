mod core;
mod expressions;
mod generator;
mod instruction;
mod scope;
mod statements;

pub use core::*;
pub use generator::Bytecode;
pub use generator::BytecodeGenerator;
pub use instruction::{Constant, ConstantPool, Instruction};

pub fn generate(source: &str) {
    let mut parser = lumi_parser::Parser::new(source);
    let ast = parser.parse().unwrap();
    let mut generator = generator::BytecodeGenerator::new();
    generator.generate(&ast);
}
