use super::{
    Registers, 
    Stack, 
    Flags, 
    Operation, 
    Instruction, 
    RegisterAliases, 
    FlagAliases,
};

pub struct Cpu {
    registers: Registers,
    stack: Stack,
    flags: Flags,
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: Registers::new(),
            stack: Stack::new(),
            flags: Flags::new(),
        }
    }

    pub fn cpu_pop(&mut self) -> u16 {
        self.stack.pop(&mut self.registers.get_mut_register(RegisterAliases::StackPointer as u16))
    }

    // the main implementation of the CPU
    pub fn perform_operation(&mut self, operation: Operation) -> bool {
        match operation {
            Operation::Nullary(instruction) => {
                match instruction {
                    Instruction::RET => {
                        self.ret();
                    },
                    Instruction::INT => {
                        self.int();
                        return true;
                    },
                    _ => {
                        panic!("unimplemented nullary instruction: {:?}", instruction);
                    },
                }
            },
            Operation::Unary(instruction, register) => {
                // implemenation of all unary instructions
                match instruction {
                    Instruction::PUSH => {
                        self.push(register.get_value());
                    },
                    Instruction::POP => {
                        self.pop(register.get_value());
                    },
                    Instruction::INC => {
                        self.inc(register.get_value());
                    },
                    Instruction::DEC => {
                        self.dec(register.get_value());
                    },
                    Instruction::JUMP => {
                        self.jump(register.get_value());
                    },
                    Instruction::CALL => {
                        self.call(register.get_value());
                    },
                    Instruction::JEQ => {
                        self.jeq(register.get_value());
                    },
                    Instruction::JNE => {
                        self.jne(register.get_value());
                    },
                    Instruction::NOT => {
                        self.not(register.get_value());
                    },
                    _ => {
                        panic!("unimplemented unary instruction: {:?}", instruction);
                    },
                }
            }
            Operation::Binary(instruction, operand, operand2) => {
                match instruction {
                    Instruction::CMP => {
                        self.cmp(operand.get_value(), operand2.get_value());
                    },
                    Instruction::ADD => {
                        self.add(operand.get_value(), operand2.get_value());
                    },
                    Instruction::SUB => {
                        self.sub(operand.get_value(), operand2.get_value());
                    },
                    Instruction::MUL => {
                        self.mul(operand.get_value(), operand2.get_value());
                    },
                    Instruction::DIV => {
                        self.div(operand.get_value(), operand2.get_value());
                    },
                    Instruction::AND => {
                        self.and(operand.get_value(), operand2.get_value());
                    },
                    Instruction::NAND => {
                        self.nand(operand.get_value(), operand2.get_value());
                    },
                    Instruction::OR => {
                        self.or(operand.get_value(), operand2.get_value());
                    },
                    Instruction::XOR => {
                        self.xor(operand.get_value(), operand2.get_value());
                    },
                    Instruction::SHL => {
                        self.shl(operand.get_value(), operand2.get_value());
                    },
                    Instruction::SHR => {
                        self.shr(operand.get_value(), operand2.get_value());
                    },
                    Instruction::MOV => {
                        self.mov(operand.get_value(), operand2.get_value());
                    },
                    _ => {
                        panic!("unimplemented binary instruction: {:?}", instruction);
                    },
                }
            }
        }
        false
    }

    fn dec(&mut self, oprd: u16) {
        *self.registers.get_mut_register(oprd) -= 1;
    }

    fn inc(&mut self, oprd: u16) {
        *self.registers.get_mut_register(oprd) += 1;
    }

    fn pop(&mut self, oprd: u16) {
        self.registers.set_register(
            oprd,
            self.stack.pop(
                &mut self.registers.get_register(RegisterAliases::StackPointer as u16),
            ),
        )
    }

    fn push(&mut self, oprd: u16) {
        self.stack.push(
            &mut self.registers.get_register(RegisterAliases::StackPointer as u16),
            self.registers.get_register(oprd),
        )
    }

    fn cmp(&mut self, oprd: u16, oprd2: u16) {
        self.flags.set_flag_to_bool(
            FlagAliases::Zero as u16,
            self.registers.get_register(oprd) == oprd2,
        );
    }

    fn jump(&mut self, oprd: u16) {
        self.registers.set_register(
            RegisterAliases::InstructionPointer as u16,
            self.registers.get_register(oprd),
        );
    }

    fn call(&mut self, oprd: u16) {
        self.stack.push(
            &mut self.registers.get_register(RegisterAliases::StackPointer as u16),
            self.registers.get_register(RegisterAliases::InstructionPointer as u16),
        );
        self.registers.set_register(
            RegisterAliases::InstructionPointer as u16,
            self.registers.get_register(oprd),
        );
    }

    fn ret(&mut self) {
        let popped = self.stack.pop(
            &mut self.registers.get_register(RegisterAliases::StackPointer as u16),
        );
        self.jump(popped)
    }

    fn jeq(&mut self, oprd: u16) {
        if self.flags.get_flag(FlagAliases::Zero as u16) {
            self.jump(oprd);
        }
    }

    fn int(&mut self) {
        self.registers.set_register(
            RegisterAliases::InstructionPointer as u16,
            self.stack.pop(
                &mut self.registers.get_register(RegisterAliases::StackPointer as u16),
            ),
        );
    }

    fn jne(&mut self, oprd: u16) {
        if !self.flags.get_flag(FlagAliases::Zero as u16) {
            self.jump(oprd);
        }
    }

    fn not(&mut self, oprd: u16) {
        self.registers.set_register(
            oprd,
            !self.registers.get_register(oprd),
        );
    }

    fn add(&mut self, oprd: u16, oprd2: u16) {
        self.registers.set_register(
            oprd,
            self.registers.get_register(oprd) + self.registers.get_register(oprd2),
        );
    }

    fn sub(&mut self, oprd: u16, oprd2: u16) {
        self.registers.set_register(
            oprd,
            self.registers.get_register(oprd) - self.registers.get_register(oprd2),
        );
    }

    fn mul(&mut self, oprd: u16, oprd2: u16) {
        self.registers.set_register(
            oprd,
            self.registers.get_register(oprd) * self.registers.get_register(oprd2),
        );
    }

    // leaves remainder in oprd2
    fn div(&mut self, oprd: u16, oprd2: u16) {
        self.registers.set_register(
            oprd,
            self.registers.get_register(oprd) / self.registers.get_register(oprd2),
        );
        self.registers.set_register(
            oprd2,
            self.registers.get_register(oprd) % self.registers.get_register(oprd2),
        );
    }

    fn and(&mut self, oprd: u16, oprd2: u16) {
        self.registers.set_register(
            oprd,
            self.registers.get_register(oprd) & self.registers.get_register(oprd2),
        );
    }

    fn nand(&mut self, oprd: u16, oprd2: u16) {
        self.registers.set_register(
            oprd,
            !(self.registers.get_register(oprd) & self.registers.get_register(oprd2)),
        );
    }

    fn or(&mut self, oprd: u16, oprd2: u16) {
        self.registers.set_register(
            oprd,
            self.registers.get_register(oprd) | self.registers.get_register(oprd2),
        );
    }

    fn xor(&mut self, oprd: u16, oprd2: u16) {
        self.registers.set_register(
            oprd,
            self.registers.get_register(oprd) ^ self.registers.get_register(oprd2),
        );
    }

    fn shl(&mut self, oprd: u16, oprd2: u16) {
        self.registers.set_register(
            oprd,
            self.registers.get_register(oprd) << self.registers.get_register(oprd2),
        );
    }

    fn shr(&mut self, oprd: u16, oprd2: u16) {
        self.registers.set_register(
            oprd,
            self.registers.get_register(oprd) >> self.registers.get_register(oprd2),
        );
    }

    fn mov(&mut self, oprd: u16, oprd2: u16) {
        self.registers.set_register(oprd, self.registers.get_register(oprd2));
    }
}
