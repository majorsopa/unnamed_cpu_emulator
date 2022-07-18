mod memory;
mod cpu;

use memory::{Ram, Memory};
use cpu::{
    Cpu, 
    Interrupt,
    // Operation,
};
pub use cpu::Operation;  // for testing purposes

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

    pub fn take_instruction(&mut self, operation: Operation) {
        // returns true if there is an interrupt
        if self.cpu.perform_operation(operation) {
            self.handle_interrupt();
        }
    }

    fn handle_interrupt(&mut self) {
        match Interrupt::from_u16(self.cpu.cpu_pop()) {
            Interrupt::Print => {
                // top down of the stack for printing
                /*
                length of string to print
                address of string to print
                */
                let length = self.cpu.cpu_pop();
                let address = self.cpu.cpu_pop();
                let mut string = String::new();
                for i in 0..length {
                    string.push(self.ram.read_byte(address + i as u16) as char);
                }
                print!("{}", string);
            },
        }
    }
}
