use lumi_ast::Node;
use lumi_parser::Parser;

#[test]
fn test_block_statement() {
    let mut parser = Parser::new("{ let x -> 1; let y -> 2; }");
    let result = parser.parse();
    assert!(result.is_ok());

    if let Ok(Node::Program(program)) = result {
        if let Node::BlockStatement(block) = &program.body[0] {
            assert_eq!(block.body.len(), 2);
            if let Node::VariableDeclaration(decl) = &block.body[0] {
                assert_eq!(decl.kind, "let");
            } else {
                panic!("Expected VariableDeclaration");
            }
        } else {
            panic!("Expected BlockStatement");
        }
    }
}
