#[derive(Debug, Clone, Copy)]
pub struct Stack([u16; 64]);

impl Stack {
    pub fn new() -> Stack {
        Stack([0; 64])
    }
    
    pub fn push(&mut self, stack_pointer: &mut u16, value: u16) {
        self.0[*stack_pointer as usize] = value;
        *stack_pointer += 1;
    }

    pub fn pop(&mut self, stack_pointer: &mut u16) -> u16 {
        *stack_pointer -= 1;
        self.0[*stack_pointer as usize]
    }
}
