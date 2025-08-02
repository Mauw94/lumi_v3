use lumi_bytecode::BytecodeGenerator;
use lumi_parser::Parser;
use lumi_semantic::analyze;
use lumi_vm::Vm;

pub fn run(source: &str) {
    let mut parser = Parser::new(source);
    let ast = parser.parse().unwrap();

    let semantic_result = analyze(&ast);
    match semantic_result {
        Ok(_) => {
            let mut bytecode_generator = BytecodeGenerator::new();
            let bytecode = bytecode_generator.generate(&ast);

            let mut vm = Vm::new();
            vm.execute(&bytecode);
        }
        Err(e) => println!("{:?}", e),
    }
}
