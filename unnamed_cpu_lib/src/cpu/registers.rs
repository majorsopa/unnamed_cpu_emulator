/*
order of registers:
0: accumulator
1: scratch
2: counter
3: scratch

4: stack pointer
*/
pub struct Registers([u16; 16]);

impl Registers {
    pub fn new() -> Registers {
        Registers([0; 16])
    }
    
    pub fn set_register(&mut self, register: u16, value: u16) {
        self.0[register as usize] = value;
    }

    pub fn get_register(&self, register: u16) -> u16 {
        self.0[register as usize]
    }

    pub fn get_mut_register(&mut self, register: u16) -> &mut u16 {
        &mut self.0[register as usize]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RegisterAliases {
    // scratchable
    Accumulator,
    Scratch0,
    Counter,
    Scratch1,
    
    // don't fw these
    StackPointer,
    ReturnAddress,
    InstructionPointer,
}
