use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::open("file.txt").expect("Can't open file");
    let mut file_content = String::new();

    file.read_to_string(&mut file_content)
        .expect("Can't read file");

    println!("File contents:\n\n{}", file_content);
}
