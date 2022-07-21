#[derive(Debug, Clone, Copy)]
pub struct Stack([u16; 64]);

impl Stack {
    pub fn new() -> Self {
        Self([0; 64])
    }
    
    pub fn push(&mut self, stack_pointer: &mut u16, value: u16) {
        *self.0.get_mut(*stack_pointer as usize).expect("stack overflow") = value;
        *stack_pointer += 1;
    }

    pub fn pop(&mut self, stack_pointer: &mut u16) -> u16 {
        *stack_pointer -= 1;
        *self.0.get(*stack_pointer as usize).expect("stack underflow")
    }
}
