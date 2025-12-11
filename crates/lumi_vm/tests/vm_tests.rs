use lumi_bytecode::{Bytecode, Constant, FunctionObj, Instruction};
use lumi_vm::{Value, Vm};

#[test]
fn test_execute_basic_add() {
    let mut vm = Vm::new();
    let bytecode = Bytecode {
        instructions: vec![
            Instruction::PushConst(0), // Push first constant
            Instruction::PushConst(1), // Push second constant
            Instruction::Add,          // Add the two values
        ],
        constants: vec![
            Constant::Number(5.0), // Constant at index 0
            Constant::Number(3.0), // Constant at index 1
        ],
    };

    vm.execute(bytecode).unwrap();
    assert_eq!(vm.stack.values, vec![Value::Number(8.0)]); // Check if the result is 8.0
}

#[test]
fn test_store_and_load_var_variable() {
    let mut vm = Vm::new();
    let bytecode = Bytecode {
        instructions: vec![
            Instruction::PushConst(0), // Push constant 42
            Instruction::StoreVar(0),  // Store it in variable index 0
            Instruction::LoadVar(0),   // Load variable index 0
        ],
        constants: vec![Constant::Number(42.0)], // Constant at index 0
    };

    vm.execute(bytecode).unwrap();
    assert_eq!(vm.stack.values, vec![Value::Number(42.0)]); // Check if the variable holds 42.0
}

#[test]
fn test_print_statement() {
    let mut vm = Vm::new();
    let bytecode = Bytecode {
        instructions: vec![
            Instruction::PushConst(0), // Push constant 42
            Instruction::StoreVar(0),  // Store it in variable index 0
            Instruction::LoadVar(0),
            Instruction::Print,
        ],
        constants: vec![Constant::Number(42.0)], // Constant at index 0
    };

    vm.execute(bytecode).unwrap();
    assert_eq!(vm.stack.values, vec![Value::Number(42.0)]); // Check if the variable holds 42.0
}

// TODO: extend vm tests to test more instructions etc (print, if, fn)
#[test]
fn test_fn_statement() {
    // Example
    r#"
        fn test(x, y) {
            x + y;
        }

        test(1, 2);
    "#;
    let mut vm = Vm::new();
    let bytecode = Bytecode {
        instructions: vec![
            Instruction::PushConst(0),
            Instruction::PushConst(1),
            Instruction::PushConst(2),
            Instruction::CallFn("test".to_string()),
        ],
        constants: vec![
            Constant::Function(FunctionObj {
                name: Some("test".to_string()),
                arity: 2,
                instructions: vec![
                    Instruction::LoadVar(0),
                    Instruction::LoadVar(1),
                    Instruction::Add,
                    Instruction::Return,
                ],
                constants: vec![],
            }),
            Constant::Number(1.0),
            Constant::Number(2.0),
        ],
    };

    vm.execute(bytecode).unwrap();
    assert_eq!(vm.stack.values, vec![Value::Number(3.0)]);
}
