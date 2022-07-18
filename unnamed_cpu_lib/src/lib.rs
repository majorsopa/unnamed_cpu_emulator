mod memory;
mod cpu;

use memory::Ram;
use cpu::{
    Cpu,
    Operation
};

pub struct UnnamedVM {
    cpu: Cpu,
    ram: Ram,
}

impl UnnamedVM {
    pub fn new(ram_size: usize) -> Self {
        Self {
            cpu: Cpu::new(),
            ram: Ram::new(ram_size),
        }
    }

    pub fn take_instruction(&mut self, operation: Operation) {
        self.cpu.perform_operation(operation);
    }
}
