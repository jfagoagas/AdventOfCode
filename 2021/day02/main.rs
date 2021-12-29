use clap::Parser;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

// Args
#[derive(Parser, Debug)]
#[clap(about, author)]
struct Args {
    // Input puzzle
    #[clap(short, long, default_value = "input.txt")]
    input_path: String,
}

// Needed to implement trait Debug, to use std::fmt traits
#[derive(Debug)]
struct Command {
    position: String,
    value: u64,
}

fn star1(sonar_report: &Vec<Command>) -> u64 {
    let mut horizontal_position = 0;
    let mut depth_position = 0;
    for instruction in sonar_report {
        // Extract &str from String
        match instruction.position.as_str() {
            "forward" => horizontal_position += instruction.value,
            "up" => depth_position -= instruction.value,
            "down" => depth_position += instruction.value,
            _ => println!("Instruction not covered yet"),
        }
    }
    depth_position * horizontal_position
}

fn star2(sonar_report: &Vec<Command>) -> u64 {
    let mut horizontal_position = 0;
    let mut depth_position = 0;
    let mut aim_position = 0;
    for instruction in sonar_report {
        // Extract &str from String
        match instruction.position.as_str() {
            "forward" => {
                horizontal_position += instruction.value;
                depth_position += aim_position * instruction.value;
            }
            "up" => aim_position -= instruction.value,
            "down" => aim_position += instruction.value,
            _ => println!("Instruction not covered yet"),
        }
    }
    depth_position * horizontal_position
}

fn main() {
    let args = Args::parse();
    // Filename as an argument
    // let filename = std::env::args().nth(1).expect("no pattern given");
    // Read input file
    let sonar_report = parse_input(args.input_path);
    println!("Star 1: {}", star1(&sonar_report));
    println!("Star 2: {}", star2(&sonar_report));
}

fn parse_input(filename: impl AsRef<Path>) -> Vec<Command> {
    // Open new file. Expect panics if the value is an error
    let file = File::open(filename).expect("no such file");
    // New buffer reading from file
    let reader = BufReader::new(file);
    // New vector to store our information
    let mut instructions: Vec<Command> = Vec::new();

    // Read line and un unwrap String from Result<String, std::io::Error>
    for line in reader.lines().map(|l| l.unwrap()) {
        // Split this string by whitespaces
        let mut it = line.split_whitespace();
        let cmd = Command {
            // Unwrap to recover value from Option<&str>
            position: String::from(it.next().unwrap()),
            // Parse unwrapped value to u64, and also unwrap it
            value: String::from(it.next().unwrap()).parse::<u64>().unwrap(),
        };
        // Append to vector
        instructions.push(cmd);
    }
    // Same as return instructions;
    instructions
}

