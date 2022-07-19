use super::{RegisterAliases, Instruction};

#[derive(Debug)]
pub enum Operation {
    Nullary(Instruction),
    Unary(Instruction, Operand),
    Binary(Instruction, Operand, Operand),
}

impl Operation {
    pub fn to_u16_vec(&self) -> Vec<u16> {
        let mut result: Vec<u16> = Vec::new();
        match self {
            Operation::Nullary(instruction) => {
                result.push(*instruction as u16);
            }
            Operation::Unary(instruction, operand) => {
                result.push(*instruction as u16);
                result.push(operand.get_value());
            }
            Operation::Binary(instruction, operand1, operand2) => {
                result.push(*instruction as u16);
                result.push(operand1.get_value());
                result.push(operand2.get_value());
            }
        }
        result
    }
}

#[derive(Debug)]
pub struct Operand(u16);

impl Operand {
    pub fn get_value(&self) -> u16 {
        self.0
    }

    pub fn from_register(register: RegisterAliases) -> Self {
        Self(register as u16)
    }

    pub fn from_u16(value: u16) -> Self {
        Self(value)
    }
}
