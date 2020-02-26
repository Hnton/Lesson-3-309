use std::fs::File;
use std::io::{BufRead, BufReader};
use std::convert::TryInto;

fn main() {
    // CHANGE TO INPUT1, INPUT2 or INPUT 3
    let mut f = BufReader::new(File::open("input2.txt").unwrap());
    let mut s = String::new();

    // Reads the file and puts each line into its own vector inside of a vector
    f.read_line(&mut s).unwrap();
    let arr: Vec<Vec<i32>> = f
        .lines()
        .map(|l| {
            l.unwrap()
                .split(char::is_whitespace)
                .map(|number| number.parse().unwrap())
                .collect()
        })
        .collect();

    //VECTOR CLONE
    let array_clone = arr.clone();
    // println!("Input:     {:?}", array_clone);

    let resource_number = array_clone[0].len();
    //RESOURCE CLONE
    let _resource_number_clone = resource_number.clone();

    // Prints out how many resources are in vector
    println!("Resources: {}", _resource_number_clone);

    let processes = (array_clone.len() - 1) / 2;
    //PROCESS CLONE
    let processes_clone = processes.clone();

    // Prints out how many processes are in vector
    println!("Processes: {}", processes_clone);

    let available = &array_clone[0];
    // AVAILABLE CLONE
    let mut available_clone = available.clone();
    
    // Prints out available from file
    println!("Available: {:?}", available_clone);

    // iterates over vector and gets first half of numbers for resources
    let _resources: Vec<Vec<i32>> = array_clone[1..processes_clone + 1]
        .iter()
        .cloned()
        .collect();

    // RESOURCE CLONE
    let mut _resources_clone = _resources.clone();

    // Prints out resources
    println!("Resource: {:?}", _resources_clone);

    // Iterates over vector and gets second half of number for max resources
    let _max: Vec<Vec<i32>> = array_clone[processes_clone + 1..array_clone.len()]
        .iter()
        .cloned()
        .collect();

    //MAX CLONE
    let _max_clone = _max.clone();

    // Prints out Max Resources
    println!("Max Res:  {:?}", _max);

    println!("\n\n");

    // Count of how many processes there are
    let mut count = processes_clone;

    // Count of how many are not running yet
    let mut not_running = 0;

    let mut run: Vec<i32> = Vec::new();

    // While loop to keep looping until no processes are not running
    while count !=0 
    {
        // println!("count: {}", count);
        // Goes through all process
        for i in 0..processes_clone
        {
            // Only goes in if not running process are greater than or equal to number of resources
            if resource_number >= not_running
            {
                let mut exec = true;
                // Goes through all resources
                for j in 0.._resource_number_clone 
                {
                    // println!("Index:{} Available:{:?}",j, available_clone[j]);
                    // println!("Index:{} Max:{:?}", i,_max_clone[i][j] );

                    // If the max - allocated > available
                    if _max_clone[i][j] - _resources_clone[i][j] > available_clone[j]
                    {
                        // println!("Available {:?}", available_clone);

                        // println!("Process {} NOT running", i +1);
                         exec = false;

                        not_running = not_running + 1;
                        break;
                        
                    }
                    // if max - allocated < available and process can start
                    if exec == true
                    {
                        println!("Process {} is Running", i + 1);

                        let inc = i + 1;
                        run.push(inc.try_into().unwrap());
                        println!("run index {:?}",run );

                        for j in 0.._resource_number_clone
                        {
                            available_clone[j] = available_clone[j] + _resources_clone[i][j]; 
                        }
                        println!("Available: {:?}", available_clone);
                        
                        count = count - 1; 
                        break;
                    }
                }

            } 
        }
    }
}
