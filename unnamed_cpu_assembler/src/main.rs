mod util;

use util::{
    read_file,
    clean_line,
    parse_operation,
};

const FILE_NAME: &str = "in.unc";

fn main() {
    let file_contents = read_file(FILE_NAME);
    
    for line in file_contents.lines() {
        let operation = parse_operation(&*clean_line(&*line));
        println!("{:?}", operation);
    }
}
