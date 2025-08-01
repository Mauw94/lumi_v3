use lumi_bytecode::{Bytecode, Constant, Instruction};
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
        //     constants:  vec![
        //     Value::Number(5.0), // Constant at index 0
        //     Value::Number(3.0), // Constant at index 1
        // ]
        constants: vec![
            Constant::Number(5.0), // Constant at index 0
            Constant::Number(3.0), // Constant at index 1
        ],
    };

    vm.execute(&bytecode);
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

    vm.execute(&bytecode);
    assert_eq!(vm.stack.values, vec![Value::Number(42.0)]); // Check if the variable holds 42.0
}
