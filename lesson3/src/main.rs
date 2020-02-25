use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut f = BufReader::new(File::open("input1.txt").unwrap());

    let mut s = String::new();
    f.read_line(&mut s).unwrap();

    let arr: Vec<Vec<i32>> = f.lines()
        .map(|l| l.unwrap().split(char::is_whitespace)
             .map(|number| number.parse().unwrap())
             .collect())
        .collect();


    println!("{:?}", arr);
}