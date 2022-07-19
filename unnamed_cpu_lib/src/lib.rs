mod memory;
mod cpu;

use memory::{Ram, Memory};
use cpu::Cpu;
pub use cpu::{
    Operation,
    Instruction,
    RegisterAliases,
    Operand,
    Interrupt,
};  // for testing purposes

pub struct UnnamedVM {
    cpu: Cpu,
    ram: Ram,
}

impl UnnamedVM {
    pub fn new(ram_size: u16) -> Self {
        Self {
            cpu: Cpu::new(),
            ram: Ram::new(ram_size),
        }
    }

    // function to load a program into the memory
    pub fn write_program(&mut self, program: &[u16]) {  // this works perfectly
        for (i, instruction) in program.iter().enumerate() {
            self.ram.write_word(i as u16 * 2, *instruction);
        }
    }

    // loads the program and runs it
    pub fn run(&mut self, start_address: u16, program_length: u16) {
        let mut start_address_mut = start_address;
        let mut operations: Vec<Operation> = Vec::new();

        while start_address_mut <= start_address + program_length {
            operations.push(self.load_operation(&mut start_address_mut));
        }

        for operation in operations {
            println!("{:?}", operation);
            self.handle_operation(operation);
        }
    }

    // function to load an Operation from the memory and handle the operation with `handle_operation`
    fn load_operation(&mut self, operation_address: &mut u16) -> Operation {  // this does not work perfectly
        let (operation_length, instruction) = self.get_operation_length_and_instruction(*operation_address);
        //println!("{:?}", instruction);
        let ret_operation = match operation_length - 1 {
            0 => Operation::Nullary(instruction),
            1 => Operation::Unary(
                instruction,
                Operand::from_u16(self.ram.read_word(*operation_address + 1))
            ),
            2 => Operation::Binary(
                instruction,
                Operand::from_u16(self.ram.read_word(*operation_address + 1)),
                Operand::from_u16(self.ram.read_word(*operation_address + 2))
            ),
            _ => panic!("Invalid operation length"),
        };
        *operation_address += operation_length;
        ret_operation
    }

    // function to determine how far ahead to read to form an operation
    fn get_operation_length_and_instruction(&self, operation_address: u16) -> (u16, Instruction) {
        // todo: make this not think every operation is a push
        let instruction = Instruction::from_u16(self.ram.read_word(operation_address * 2));
        println!("{:?}", self.ram.read_word(operation_address));
        (
            match instruction {
                Instruction::RET => 1,
                Instruction::INT => 1,

                Instruction::PUSH => 2,
                Instruction::POP => 2,
                Instruction::JUMP => 2,
                Instruction::CALL => 2,
                Instruction::JEQ => 2,
                Instruction::JNE => 2,
                Instruction::INC => 2,
                Instruction::DEC => 2,
                Instruction::NOT => 2,
                
                Instruction::CMP => 3,
                Instruction::MOV => 3,
                Instruction::ADD => 3,
                Instruction::SUB => 3,
                Instruction::MUL => 3,
                Instruction::DIV => 3,
                Instruction::AND => 3,
                Instruction::NAND => 3,
                Instruction::OR => 3,
                Instruction::XOR => 3,
                Instruction::SHL => 3,
                Instruction::SHR => 3,
            },
            instruction,
        )
    }

    fn handle_operation(&mut self, operation: Operation) {
        // returns true if there is an interrupt
        if self.cpu.perform_operation(operation) {
            self.handle_interrupt();
        }
    }

    fn handle_interrupt(&mut self) {
        match Interrupt::from_u16(self.cpu.cpu_pop()) {
            Interrupt::Print => {
                // top-down of the stack for printing
                /*
                length of string to print
                address of string to print
                */
                let length = self.cpu.cpu_pop();
                let address = self.cpu.cpu_pop();
                for i in 0..length {
                    print!("{}", self.ram.read_byte(address + i as u16) as char);
                }
            },
        }
    }
}
