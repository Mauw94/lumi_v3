use lumi_bytecode::BytecodeGenerator;
use lumi_parser::Parser;
use lumi_semantic::analyze;
use lumi_vm::Vm;

pub struct Engine;

impl Engine {
    pub fn new() -> Self {
        Self {}
    }

    pub fn evaluate(&self, source: &str) -> Result<(), String> {
        let mut parser = Parser::new(source);
        let ast = parser.parse().map_err(|e| format!("Parser error: {e}"))?;

        analyze(&ast).map_err(|e| format!("Semantic error: {e}"))?;

        let mut bytecode_generator = BytecodeGenerator::new();
        let bytecode = bytecode_generator.generate(&ast);

        let mut vm = Vm::new();
        vm.execute(&bytecode).unwrap();

        Ok(())
    }
}
