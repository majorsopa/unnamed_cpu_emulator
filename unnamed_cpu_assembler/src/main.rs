mod util;

use util::{
    read_file,
    clean_line,
    parse_operation, write_file,
};

const FILE_NAME: &str = "in.unc";

fn main() {
    let file_contents = read_file(FILE_NAME);
    
    let mut operation_vec = Vec::new();

    for line in file_contents.lines() {
        if let Some(operation) = parse_operation(&*clean_line(&*line)) {
            operation_vec.push(operation);
        }
    }

    write_file(operation_vec);
}
