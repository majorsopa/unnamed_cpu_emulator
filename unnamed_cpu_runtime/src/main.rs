use unnamed_cpu_lib::{
    UnnamedVM,
    Operation,
    Instruction,
    RegisterAliases,
    Operand, Interrupt,
};


const MEMORY_AMOUNT: u16 = 16;

fn main() {
    let mut vm = UnnamedVM::new(MEMORY_AMOUNT);

    // move 0 to scratch0 to just print the start of the memory
    vm.take_instruction(Operation::Binary(
        Instruction::MOV,
        Operand::from_register(RegisterAliases::Scratch0),
        Operand::from_literal(0),
    ));

    // push scratch0 to the stack
    vm.take_instruction(Operation::Unary(
        Instruction::PUSH,
        Operand::from_register(RegisterAliases::Scratch0),
    ));

    // move the length of the string needing printing, for testing it'll be 8, to scratch0
    vm.take_instruction(Operation::Binary(
        Instruction::MOV,
        Operand::from_register(RegisterAliases::Scratch0),
        Operand::from_literal(8),
    ));

    // push scratch0 to the stack
    vm.take_instruction(Operation::Unary(
        Instruction::PUSH,
        Operand::from_register(RegisterAliases::Scratch0),
    ));

    // move print interrupt as u16 to scratch0
    vm.take_instruction(Operation::Binary(
        Instruction::MOV,
        Operand::from_register(RegisterAliases::Scratch0),
        Operand::from_literal(Interrupt::Print as u16),
    ));

    // push scratch0 to the stack
    vm.take_instruction(Operation::Unary(
        Instruction::PUSH,
        Operand::from_register(RegisterAliases::Scratch0),
    ));

    // call an interrupt
    vm.take_instruction(Operation::Nullary(
        Instruction::INT,
    ));
}
