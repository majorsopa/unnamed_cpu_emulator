use super::{RegisterAliases, Instruction};

pub enum Operation {
    Nullary(Instruction),
    Unary(Instruction, Operand),
    Binary(Instruction, Operand, Operand),
}

#[derive(Debug)]
pub enum Operand {
    Register(RegisterAliases),
    Literal(u16),
    Address(u16),
}

impl Operand {
    pub fn get_value(&self) -> u16 {
        match self {
            Operand::Register(register) => *register as u16,
            Operand::Literal(literal) => *literal,
            Operand::Address(address) => *address,
        }
    }
}
