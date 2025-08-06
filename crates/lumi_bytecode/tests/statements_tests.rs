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

#[test]
fn test_if_statement() {
    let mut parser = Parser::new(
        r#"
        let x -> 42;
        if (x > 30) {
            print "ok";
        } else {
            print "not ok";
        }
    "#,
    );
    let ast = parser.parse().unwrap();
    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    assert_eq!(
        bytecode.constants,
        vec![
            Constant::Number(42.0),
            Constant::Number(30.0),
            Constant::String("ok".to_string()),
            Constant::String("not ok".to_string())
        ]
    );
}

#[test]
fn test_fn_statement() {
    let mut parser = Parser::new(
        r#"
        fn test(x, y) {
            x + y;
        }
    "#,
    );
    let ast = parser.parse().unwrap();
    let mut bytecode_generator = BytecodeGenerator::new();
    let bytecode = bytecode_generator.generate(&ast);

    assert_eq!(bytecode.constants, vec![]);
}
