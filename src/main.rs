extern crate humansize;
use humansize::{file_size_opts as options, FileSize};

fn main() {
    let size = 1000;
    println!("Size is {}", size.file_size(options::DECIMAL).unwrap());

    println!("Size is {}", size.file_size(options::BINARY).unwrap());

    println!("Size is {}", size.file_size(options::CONVENTIONAL).unwrap());
}
