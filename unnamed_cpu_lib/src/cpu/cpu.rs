use super::{Registers, Stack, Flags};

pub struct Cpu {
    registers: Registers,
    stack: Stack,
    flags: Flags,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            stack: Stack::new(),
            flags: Flags::new(),
        }
    }
}
