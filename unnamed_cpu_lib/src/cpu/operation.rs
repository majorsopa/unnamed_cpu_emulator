use super::{RegisterAliases, Instruction};

pub enum Operation<'a> {
    Nullary(Instruction),
    Unary(Instruction, &'a mut RegisterAliases),
    Binary(Instruction, &'a mut RegisterAliases, &'a mut Operand<'a>),
}

#[derive(Debug)]
pub enum Operand<'a> {
    Register(&'a mut RegisterAliases),
    Literal(u16),
    Address(u16),
}

impl Operand<'_> {
    pub fn get_value(&self) -> u16 {
        match self {
            Operand::Register(register) => **register as u16,
            Operand::Literal(literal) => *literal,
            Operand::Address(address) => *address,
        }
    }
}
