use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file = File::create("myfile.txt").expect("File couldn't be created");

    file.write(b"Grumpy wizards make toxic brew for the evil queen and Jack")
        .expect("The file couldn't be written to");
}
