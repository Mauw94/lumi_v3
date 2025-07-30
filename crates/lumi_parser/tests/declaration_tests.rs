use lumi_ast::Node;
use lumi_parser::Parser;

#[test]
fn test_var_declaration() {
    let mut parser = Parser::new("let x: int -> 42;");
    let result = parser.parse();

    if let Ok(Node::Program(program)) = result {
        if let Node::VariableDeclaration(decl) = &program.body[0] {
            assert_eq!(decl.kind, "let");
            assert_eq!(decl.declarations.len(), 1);
        } else {
            panic!("Expected a variable declaration node");
        }
    }
}

#[test]
fn test_multiple_var_declaration() {
    let mut parser = Parser::new("let x: int -> 42, y -> 41, z: int -> 40, k -> \"hello world\";");
    let result = parser.parse();

    if let Ok(Node::Program(program)) = result {
        if let Node::VariableDeclaration(decl) = &program.body[0] {
            assert_eq!(decl.kind, "let");
            assert_eq!(decl.declarations.len(), 4);
        } else {
            panic!("Expected a variable declaration node");
        }
    }
}
