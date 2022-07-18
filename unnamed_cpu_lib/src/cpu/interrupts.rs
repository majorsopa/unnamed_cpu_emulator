#[derive(Debug)]
#[repr(u16)]
pub enum Interrupt {
    Print,
}

impl Interrupt {
    pub fn from_u16(value: u16) -> Self {
        match value {
            0 => Interrupt::Print,
            _ => panic!("unimplemented interrupt: {}", value),
        }
    }
}
