use unnamed_cpu_lib::{
    UnnamedVM,
    Operation,
    Instruction,
    RegisterAliases,
    Operand, Interrupt,
};


const MEMORY_AMOUNT: u16 = 128;

fn main() {
    let mut vm = UnnamedVM::new(MEMORY_AMOUNT);

    let program = [
        // move 0 to scratch0 to just print the start of the memory
        Operation::Binary(
            Instruction::MOV,
            Operand::from_register(RegisterAliases::Scratch0),
            Operand::from_u16(0),
        ),

        // push scratch0 to the stack
        Operation::Unary(
            Instruction::PUSH,
            Operand::from_register(RegisterAliases::Scratch0),
        ),

        // move the length of the string needing printing, for testing it'll be 8, to scratch0
        Operation::Binary(
            Instruction::MOV,
            Operand::from_register(RegisterAliases::Scratch0),
            Operand::from_u16(10),
        ),

        // push scratch0 to the stack
        Operation::Unary(
            Instruction::PUSH,
            Operand::from_register(RegisterAliases::Scratch0),
        ),

        // move print interrupt as u16 to scratch0
        Operation::Binary(
            Instruction::MOV,
            Operand::from_register(RegisterAliases::Scratch0),
            Operand::from_u16(Interrupt::Print as u16),
        ),

        // push scratch0 to the stack
        Operation::Unary(
            Instruction::PUSH,
            Operand::from_register(RegisterAliases::Scratch0),
        ),

        // call an interrupt
        Operation::Nullary(
            Instruction::INT,
        ),
    ];

    let program_slice: &[u16] = &operation_slice_to_u16_vec(&program);

    vm.load_program(program_slice);
}

fn operation_slice_to_u16_vec(operations: &[Operation]) -> Vec<u16> {
    let mut result = Vec::new();
    for operation in operations {
        result.extend(operation.to_u16_vec());
    }
    result
}
