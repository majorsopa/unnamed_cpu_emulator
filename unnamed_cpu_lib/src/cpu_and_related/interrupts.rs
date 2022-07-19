#[allow(non_camel_case_types)]


#[derive(Debug)]
#[repr(u16)]
pub enum Interrupt {
    PRINT,
    MOV_TO_RAM,
}

impl Interrupt {
    pub fn from_u16(value: u16) -> Self {
        match value {
            0 => Interrupt::PRINT,
            1 => Interrupt::MOV_TO_RAM,
            _ => panic!("unimplemented interrupt: {}", value),
        }
    }
}
