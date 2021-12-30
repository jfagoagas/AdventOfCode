use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn parse_input(filename: impl AsRef<Path>) -> Vec<u64> {
    // Open new file. Expect panics if the value is an error
    let file = File::open(filename).expect("no such file");
    // New buffer reading from file
    let buf = BufReader::new(file);
    // New vector to store
    let numbers: Vec<u64> = buf
        .lines() // New iterator
        // applies a function to each element in an iterable and returns the resulting iterable to the next function
        .map(|line| line.unwrap().parse::<u64>().unwrap())
        // consumes the iterator and collect the resulting value
        .collect();

    return numbers;
}

fn star1(sonar_report: &Vec<u64>) -> u64 {
    // First depth
    let mut measure = sonar_report[0];
    // Increase depth count
    let mut depth_count = 0;
    for value in sonar_report {
        if value > &measure {
            depth_count += 1;
        }
        measure = *value;
    }
    return depth_count;
}

fn star2(sonar_report: &Vec<u64>) -> usize {
    // Another way to iterate a Vector
    sonar_report
        .iter()
        // Iterate every 3 elements
        .skip(3)
        //  Iterator pair
        .zip(sonar_report)
        // Apply some filter
        .filter(|(a, b)| a > b)
        // Count if true
        .count()
}

fn main() {
    // Argument[1]
    let filename = std::env::args().nth(1).expect("no pattern given");
    // Read input file
    // let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    let sonar_report = parse_input(filename);
    println!("Star 1: {}", star1(&sonar_report));
    println!("Star 2: {}", star2(&sonar_report));
}
