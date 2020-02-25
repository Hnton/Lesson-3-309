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

    let array_clone = arr.clone();
    println!("{:?}", array_clone);

    let resource = array_clone[0].len();
    //RESOURCE CLONE
    let _resource_clone = resource.clone();

    println!("Resources: {}", _resource_clone);

    let processes = (array_clone.len() - 1)/2;
    //PROCESS CLONE
    let processes_clone = processes.clone();

    println!("Processes: {}", processes_clone);

    let available = &array_clone[0];
    // AVAILABLE CLONE
    let available_clone = available.clone();

    println!("Available: {:?}", available_clone);




}