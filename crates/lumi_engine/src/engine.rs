use lumi_bytecode::BytecodeGenerator;
use lumi_parser::Parser;
use lumi_semantic::analyze;
use lumi_vm::Vm;

pub struct Engine {
    bytecode_generator: BytecodeGenerator,
    vm: Vm,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            vm: Vm::new(),
            bytecode_generator: BytecodeGenerator::new(),
        }
    }

    pub fn evaluate(&mut self, source: &str) -> Result<(), String> {
        let mut parser = Parser::new(source);
        let ast = parser.parse().map_err(|e| format!("Parser error: {e}"))?;

        analyze(&ast).map_err(|e| format!("Semantic error: {e}"))?;

        let bytecode = self.bytecode_generator.generate(&ast);

        self.vm.execute(&bytecode).unwrap();

        Ok(())
    }
}
