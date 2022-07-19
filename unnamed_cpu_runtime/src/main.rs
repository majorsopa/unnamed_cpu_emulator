use unnamed_cpu_lib::{
    UnnamedVM,
    Operation,
    Instruction,
    RegisterAliases,
    Operand,
    Interrupt,
};


const MEMORY_AMOUNT: u16 = 512;
const TO_PRINT: &str = "Hey there!";

fn main() {
    let mut vm = UnnamedVM::new(MEMORY_AMOUNT);

    let mov_character_to_scratch0_and_push_operation = |character: char| {
        vec![
            Operation::Binary(
                Instruction::MOV,
                Operand::from_register(RegisterAliases::Scratch0),
                Operand::from_u16(character as u16),
            ),
            Operation::Unary(
                Instruction::PUSH,
                Operand::from_register(RegisterAliases::Scratch0),
            ),
        ]
    };
    // for every character in "Hello world!", call `mov_character_to_scratch0_and_push_operation` and append it to a Vec
    let mut mov_hello_world_to_stack_operations: Vec<Operation> = vec![];
    for character in TO_PRINT.chars().rev() {
        mov_hello_world_to_stack_operations.extend(mov_character_to_scratch0_and_push_operation(character));
    }

    // set up stack for writing the charatcers to memory
    let set_up_stack_for_writing_operations: Vec<Operation> = vec![
        Operation::Binary(
            Instruction::MOV,
            Operand::from_register(RegisterAliases::Scratch0),
            Operand::from_u16(100),
        ),
        Operation::Unary(
            Instruction::PUSH,
            Operand::from_register(RegisterAliases::Scratch0),
        ),
        Operation::Binary(
            Instruction::MOV,
            Operand::from_register(RegisterAliases::Scratch0),
            Operand::from_u16(TO_PRINT.len() as u16),
        ),
        Operation::Unary(
            Instruction::PUSH,
            Operand::from_register(RegisterAliases::Scratch0),
        ),
        Operation::Binary(
            Instruction::MOV,
            Operand::from_register(RegisterAliases::Scratch0),
            Operand::from_u16(Interrupt::MOV_TO_RAM as u16),
        ),
        Operation::Unary(
            Instruction::PUSH,
            Operand::from_register(RegisterAliases::Scratch0),
        ),
        Operation::Nullary(
            Instruction::INT,
        ),
    ];

    let print_operations: Vec<Operation> = vec![
        // move address to start printing from to scratch0
        Operation::Binary(
            Instruction::MOV,
            Operand::from_register(RegisterAliases::Scratch0),
            Operand::from_u16(100),
        ),

        // push scratch0 to the stack
        Operation::Unary(
            Instruction::PUSH,
            Operand::from_register(RegisterAliases::Scratch0),
        ),

        // move the length of the string needing printing to scratch0
        Operation::Binary(
            Instruction::MOV,
            Operand::from_register(RegisterAliases::Scratch0),
            Operand::from_u16(TO_PRINT.len() as u16),
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
            Operand::from_u16(Interrupt::PRINT as u16),
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

    let program_slice: &[u16] = &operation_slice_to_u16_vec(&{
        let mut program: Vec<Operation> = vec![];
        program.extend(mov_hello_world_to_stack_operations);
        program.extend(set_up_stack_for_writing_operations);
        program.extend(print_operations);
        program
    });

    vm.write_program(program_slice);
    vm.run(0, program_slice.len() as u16);
}

fn operation_slice_to_u16_vec(operations: &[Operation]) -> Vec<u16> {
    let mut result = Vec::new();
    for operation in operations {
        result.extend(operation.to_u16_vec());
    }
    result
}
