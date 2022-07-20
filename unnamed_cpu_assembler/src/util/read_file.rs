use std::fs;

pub fn read_file(file_name: &str) -> String {
    let contents = fs::read_to_string(file_name)
        .expect(&format!("can't read file {}", file_name));
    contents
}