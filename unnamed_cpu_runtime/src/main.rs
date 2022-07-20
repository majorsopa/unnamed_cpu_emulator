use std::fs::read_to_string;

use unnamed_cpu_lib::UnnamedVM;


const MEMORY_AMOUNT: u16 = 512;
const INPUT_FILE: &str = "out.unce";

fn main() {
    let mut vm = UnnamedVM::new(MEMORY_AMOUNT);

    // read the input file 
    let program_slice = &string_to_u16_vec(&*read_to_string(INPUT_FILE).expect("Could not read input file"));

    vm.write_program(program_slice);
    vm.run(0, program_slice.len() as u16);
}

fn string_to_u16_vec(string: &str) -> Vec<u16> {
    let mut vec = Vec::new();
    // each u16 is 4 characters long
    for i in 0..string.len() / 4 {
        let mut word = String::new();
        word.push_str(&string[i * 4..(i * 4) + 4]);
        vec.push(u16::from_str_radix(&*word, 16).unwrap());
    }
    vec
}

// was only needed for debugging
/*
fn operation_slice_to_u16_vec(operations: &[Operation]) -> Vec<u16> {
    let mut result = Vec::new();
    for operation in operations {
        result.extend(operation.to_u16_vec());
    }
    result
}
*/
