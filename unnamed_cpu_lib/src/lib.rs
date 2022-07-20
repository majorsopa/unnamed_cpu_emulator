mod memory;
mod cpu_and_related;

use memory::{Ram, Memory};
use cpu_and_related::Cpu;
pub use cpu_and_related::{
    Operation,
    Instruction,
    RegisterAliases,
    Operand,
    Interrupt,
};

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
    pub fn write_program(&mut self, program: &[u16]) {
        for (i, instruction) in program.iter().enumerate() {
            self.ram.write_word(i as u16 * 2, *instruction);
        }
    }

    // loads the program and runs it
    pub fn run(&mut self, start_address: u16, program_length: u16) {
        *self.cpu.get_instruction_pointer_mut() = start_address;

        while self.cpu.get_instruction_pointer() < start_address * 2 + program_length * 2 {
            let (inc_amount, operation) = self.load_operation(self.cpu.get_instruction_pointer());
            *self.cpu.get_instruction_pointer_mut() += inc_amount;
            self.handle_operation(operation)
        }
    }

    // function to load an Operation from the memory and handle the operation with `handle_operation`
    // returns the amount to increment the instruction pointer by and the Operation
    fn load_operation(&mut self, operation_address: u16) -> (u16, Operation) {
        let (operation_length, instruction) = self.get_operation_length_and_instruction(operation_address);
        let ret_operation = match operation_length - 1 {
            0 => Operation::Nullary(instruction),
            1 => Operation::Unary(
                instruction,
                Operand::from_u16(self.ram.read_word(operation_address + 2)),
            ),
            2 => Operation::Binary(
                instruction,
                Operand::from_u16(self.ram.read_word(operation_address + 2)),
                Operand::from_u16(self.ram.read_word(operation_address + 4)),
            ),
            _ => panic!("Invalid operation length"),
        };
        (operation_length * 2, ret_operation)
    }

    // function to determine how far ahead to read to form an operation
    fn get_operation_length_and_instruction(&self, operation_address: u16) -> (u16, Instruction) {
        let instruction = Instruction::from_u16(self.ram.read_word(operation_address));
        (
            get_instruction_length(instruction),
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
            Interrupt::PRINT => {
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
            Interrupt::MOV_TO_RAM => {
                // top-down of the stack for moving to RAM
                /*
                length of data to move (in u16's)
                address of where to put the data
                data word 0
                data word 1
                ...
                */
                let length = self.cpu.cpu_pop();
                let address = self.cpu.cpu_pop();
                for i in 0..length {
                    self.ram.write_word(address + i as u16, self.cpu.cpu_pop());
                }
            },
        }
    }
}

pub fn get_instruction_length(instruction: Instruction) -> u16 {
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
    }
}
