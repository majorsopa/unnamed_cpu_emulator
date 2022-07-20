use std::fs;

pub fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name)
        .unwrap_or_else(|_| panic!("can't read file {}", file_name))
}