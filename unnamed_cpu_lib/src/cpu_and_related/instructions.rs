#[allow(non_camel_case_types)]


#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum Instruction {
    PUSH,                   // pushes to stack
    POP,                    // pops from stack
    JUMP,                   // jumps to address
    CALL,                   // calls address
    RET,                    // returns from call
    CMP,                    // compares two values, setting a flag if they're equal or not
    JEQ,                    // jumps to address if flag is set
    JNE,                    // jumps to address if flag is not set
    INT,                    // interrupts
    MOV,                    // moves value to register

    ADD,                    // adds two values
    SUB,                    // subtracts two values
    MUL,                    // multiplies two values
    DIV,                    // divides two values
    INC,                    // increments a value
    DEC,                    // decrements a value

    AND,                    // performs bitwise AND on two values
    NAND,                   // performs bitwise NAND on two values
    OR ,                    // performs bitwise OR on two values
    XOR,                    // performs bitwise XOR on two values
    NOT,                    // performs bitwise NOT on a value
    SHL,                    // shifts a value left by a number of bits
    SHR,                    // shifts a value right by a number of bits
}

impl Instruction {
    // for sure a better way to do this. maybe with a proc macro?
    pub fn from_u16(value: u16) -> Self {
        match value {
            0 => Self::PUSH,
            1 => Self::POP,
            2 => Self::JUMP,
            3 => Self::CALL,
            4 => Self::RET,
            5 => Self::CMP,
            6 => Self::JEQ,
            7 => Self::JNE,
            8 => Self::INT,
            9 => Self::MOV,
            10 => Self::ADD,
            11 => Self::SUB,
            12 => Self::MUL,
            13 => Self::DIV,
            14 => Self::INC,
            15 => Self::DEC,
            16 => Self::AND,
            17 => Self::NAND,
            18 => Self::OR,
            19 => Self::XOR,
            20 => Self::NOT,
            21 => Self::SHL,
            22 => Self::SHR,
            _ => panic!("invalid instruction: {}", value),
        }
    }
}
