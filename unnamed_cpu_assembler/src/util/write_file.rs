use std::{fs::File, io::Write};

use unnamed_cpu_lib::Operation;

pub fn write_file(input: Vec<Operation>) {
    let mut file = File::create("output.unce").unwrap();
    for operation in input {
        let operation = operation.to_u16_vec();
        let mut line = String::new();

        for word in operation {
            line.push_str(&format!("{:04x}", word));
        }

        file.write_all(line.as_bytes()).unwrap();
    }
}