use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    println!("File: {}", filename);

    let contents = fs::read_to_string(filename).expect("Error reading file");

    println!("Text:\n{}", contents);
}
