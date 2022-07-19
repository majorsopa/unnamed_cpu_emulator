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
    pub fn load_program(&mut self, program: &[u16]) {
        for (i, instruction) in program.iter().enumerate() {
            self.ram.write_word(i as u16, *instruction);
        }
    }

    // function to load an Operation from the memory and handle the operation with `handle_operation`
    pub fn load_operation(&mut self, operation_address: u16) -> Operation {
        let (operation_length, instruction) = self.get_operation_length_and_instruction(operation_address);
        match operation_length - 1 {
            0 => Operation::Nullary(instruction),
            1 => Operation::Unary(
                instruction,
                Operand::from_u16(self.ram.read_word(operation_address + 1))
            ),
            2 => Operation::Binary(
                instruction,
                Operand::from_u16(self.ram.read_word(operation_address + 1)),
                Operand::from_u16(self.ram.read_word(operation_address + 2))
            ),
            _ => panic!("Invalid operation length"),
        }
    }

    // function to determine how far ahead to read to form an operation
    pub fn get_operation_length_and_instruction(&self, operation_address: u16) -> (u16, Instruction) {
        let instruction = Instruction::from_u16(self.ram.read_word(operation_address));
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
