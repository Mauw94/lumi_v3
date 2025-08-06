use lumi_bytecode::BytecodeGenerator;
use lumi_parser::Parser;

// NOTE: doesn't work yet
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

    println!("{:?}", bytecode.instructions);
    // assert_eq!(bytecode.constants, vec![]);
}
