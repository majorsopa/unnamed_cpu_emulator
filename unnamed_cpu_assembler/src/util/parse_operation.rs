use unnamed_cpu_lib::{
    Operation,
    Operand,
    Instruction,
    RegisterAliases,
    get_instruction_length,
};

// returns the line without unneeded whitespace and comments
pub fn clean_line(line: &str) -> String {
    let mut clean_line = line.trim_start().to_string();  // the line with extra whitespace removed
    for character in line.chars() {
        if character == ';' {
            break;
        } else {
            clean_line.push(character);
        }
    }
    clean_line.trim_end().to_string()  // the line with comments removed
}

pub fn parse_operation(line: &str) -> Operation {
    let clean_line = clean_line(line);
    let mut clean_line_peekable = clean_line.chars().peekable();

    // gets the instruction
    let mut instruction = String::new();
    while clean_line_peekable.peek().is_some() {
        let character = clean_line_peekable.next().unwrap();  // maybe make unsafe unchecked in the future
        if character == ' ' {
            break;
        } else {
            instruction.push(character);
        }
    }
    // match the instruction
    let instruction: Instruction = match &*instruction {  // maybe expand out to function if needed elsewhere
        "push" => Instruction::PUSH,
        "pop" => Instruction::POP,
        "jmp" => Instruction::JUMP,
        "call" => Instruction::CALL,
        "ret" => Instruction::RET,
        "cmp" => Instruction::CMP,
        "jeq" => Instruction::JEQ,
        "jne" => Instruction::JNE,
        "int" => Instruction::INT,
        "mov" => Instruction::MOV,

        "add" => Instruction::ADD,
        "sub" => Instruction::SUB,
        "mul" => Instruction::MUL,
        "div" => Instruction::DIV,
        "inc" => Instruction::INC,
        "dec" => Instruction::DEC,

        "and" => Instruction::AND,
        "nand" => Instruction::NAND,
        "or" => Instruction::OR,
        "xor" => Instruction::XOR,
        "not" => Instruction::NOT,
        "shl" => Instruction::SHL,
        "shr" => Instruction::SHR,

        _ => panic!("invalid instruction: {}", instruction),
    };

    let operand_amount = get_instruction_length(instruction) - 1;

    // gets the operands
    let mut operands: Vec<Operand> = Vec::new();
    for _ in 0..operand_amount {
        let mut operand_str = String::new();
        while clean_line_peekable.peek().is_some() {
            let character = clean_line_peekable.next().unwrap();  // maybe make unsafe unchecked in the future
            if character == ' ' {
                break;
            } else {
                operand_str.push(character);
            }
        }
        if let Some(operand) = match_register(&*operand_str) {
            operands.push(operand);
            continue;
        }
        // confirmed to not be a register anymore (could still be a [register])
        if let Some(operand) = operand_str.strip_prefix('[') {
            let operand = operand.strip_suffix(']').expect("error with a register address. unmatched `[`");
            if let Some(operand1) = match_register(operand) {
                operands.push(operand1);
                continue;
            } else {
                panic!("invalid register: {}", operand_str);
            }
        }
        // confirmed to not be a register or [register] anymore
        // could still be an address, string literal, or integer literal
        if let Ok(operand) = match_address(&*operand_str) {
            operands.push(Operand::from_u16(operand));
            continue;
        }
        // not an address, so must be a literal (string or integer)
        if let Some(operand_vec) = match_string(&*operand_str) {
            operands.extend(operand_vec);
        } else {
            operands.push(match_integer(&*operand_str));  // will panic if invalid
        }
    }
    match operands.len() {
        0 => Operation::Nullary(instruction),
        1 => Operation::Unary(instruction, operands[0]),
        2 => Operation::Binary(instruction, operands[0], operands[1]),
        _ => panic!("invalid operand amount: {}", operands.len()),
    }
}

fn match_register(operand: &str) -> Option<Operand> {
    match operand {
        "ax" => Some(Operand::from_register(RegisterAliases::Accumulator)),
        "bx" => Some(Operand::from_register(RegisterAliases::Scratch0)),
        "cx" => Some(Operand::from_register(RegisterAliases::Counter)),
        "dx" => Some(Operand::from_register(RegisterAliases::Scratch1)),
        "sp" => Some(Operand::from_register(RegisterAliases::StackPointer)),
        "ra" => Some(Operand::from_register(RegisterAliases::ReturnAddress)),
        "ip" => Some(Operand::from_register(RegisterAliases::InstructionPointer)),

        _ => None,
    }
}

// takes a string literal
// example: `"Hello world!"`
// returns a vector of Operands of each character in the string as u16
fn match_string(operand: &str) -> Option<Vec<Operand>> {
    let mut operands: Vec<Operand> = Vec::new();
    if operand.is_empty() {
        return None;
    }
    let mut operand = operand.chars();
    if operand.next().unwrap() != '"' {
        return None;
    }
    for character in operand {
        if character == '"' {
            break;
        } else {
            operands.push(Operand::from_u16(character as u16));
        }
    }
    Some(operands)
}

// match an integer literal
// will panic if the literal is not a valid integer
// that's okay because it's the last thing to be checked
fn match_integer(operand: &str) -> Operand {
    let operand = operand.parse::<u16>().unwrap();
    Operand::from_u16(operand)
}

// converts a hex address to a u16
// expects something like `0x1234` not `1234`
fn match_address(operand: &str) -> Result<u16, std::num::ParseIntError> {
    u16::from_str_radix(&operand[2..], 16)
}
