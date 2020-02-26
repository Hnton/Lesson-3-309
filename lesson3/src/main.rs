use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut f = BufReader::new(File::open("input1.txt").unwrap());

    let mut s = String::new();
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

    let array_clone = arr.clone();
    println!("Input:     {:?}", array_clone);

    let resource_number = array_clone[0].len();
    //RESOURCE CLONE
    let _resource_number_clone = resource_number.clone();

    println!("Resources: {}", _resource_number_clone);

    let processes = (array_clone.len() - 1) / 2;
    //PROCESS CLONE
    let processes_clone = processes.clone();

    println!("Processes: {}", processes_clone);

    let available = &array_clone[0];
    // AVAILABLE CLONE
    let mut available_clone = available.clone();

    println!("Available: {:?}", available_clone);

    let _resources: Vec<Vec<i32>> = array_clone[1..processes_clone + 1]
        .iter()
        .cloned()
        .collect();

    // RESOURCE CLONE
    let mut _resources_clone = _resources.clone();

    println!("Resource: {:?}", _resources_clone);

    let _max: Vec<Vec<i32>> = array_clone[processes_clone + 1..array_clone.len()]
        .iter()
        .cloned()
        .collect();

    //MAX CLONE
    let _max_clone = _max.clone();

    println!("Max Res:  {:?}", _max);




    println!("{:?}", _max_clone[2][3] - _resources_clone[2][3]);
    let need = _max_clone[2][3] - _resources_clone[2][3];
   
    println!("{:?}", need);

    let mut count = processes_clone;

    let mut running = 0;

    while count !=0 
    {
        println!("process Clone: {}", processes_clone);
        // println!("count: {}", count);
        for i in 0..processes_clone
        {

            if resource_number >= running
            {

                let mut exec = true;
                for j in 0.._resource_number_clone 
                {
                    // println!("Index:{} Available:{:?}",j, available_clone[j]);
                    // println!("Index:{} Max:{:?}", i,_max_clone[i][j] );
                    if _max_clone[i][j] - _resources_clone[i][j] > available_clone[j]
                    {
                        println!("{:?}", available_clone);

                        println!("Process {} NOT running", i +1);
                        exec = false;

                        running = running + 1;
                        break;
                        
                    }

                    if exec == true
                    {
                        println!("Process {} is Running", i +1);

                        for j in 0.._resource_number_clone
                        {
                            available_clone[j] = available_clone[j] + _resources_clone[i][j]; 


                            
                        }
                        println!("{:?}", available_clone);
                        
                        count = count - 1; 
                        break;
                    }
                }

            } 
        }
    }



    //Creating the Allocation Needed
    /* let _allocation = &array_clone[0];

    //Created what is needed
    let needed = &array_clone[0];

    //While loop goes through and checks to see if needed is greater than 0
    // Will keep going through until no more is needed
    // Then if once its 0 it will return all the processes
    while let available{
        if let needed > 0 {
            _max - _allocation;
            needed--;
        }
        else let{
            return processes;
        }
    } */
}
