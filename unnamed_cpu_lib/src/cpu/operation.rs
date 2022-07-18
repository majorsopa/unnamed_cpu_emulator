use super::{RegisterAliases, Instruction};

#[derive(Debug)]
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
            Self::Register(register) => *register as u16,
            Self::Literal(literal) => *literal,
            Self::Address(address) => *address,
        }
    }

    pub fn from_register(register: RegisterAliases) -> Self {
        Self::Register(register)
    }

    pub fn from_literal(literal: u16) -> Self {
        Self::Literal(literal)
    }

    pub fn from_address(address: u16) -> Self {
        Self::Address(address)
    }
}
