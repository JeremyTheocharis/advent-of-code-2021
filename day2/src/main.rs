// This program reads a file containing commands for a submarine and executes them. As a result
// it prints the horizontal and vertical position of the submarine.

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Initialize the submarine to the origin.
    let mut horizontal = 0;
    let mut depth = 0;
    let mut command_vector: Vec<String> = Vec::new();

    // Read the file containing the commands.
    // Create depth i32 vector

    // Open file and store it into vector
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(current_line) = line {
                command_vector.push(current_line.parse::<String>().unwrap());
            }
        }
    }

    // Loop through the commands and execute them.
    for command in command_vector {
        // Split the command into a command and a value using whitespace as a delimiter
        let command_and_value: Vec<&str> = command.split_whitespace().collect();

        // Get the command
        let command = command_and_value[0];

        // Get the value
        let value = command_and_value[1].parse::<i32>().unwrap();

        // Switch statement to execute the command.
        match command {
            "forward" => horizontal += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => println!("Invalid command!"),
        }
    }

    // Print the final position of the submarine.
    println!("Final position: ({}, {})", horizontal, depth);

    // Print the product of horizontal and depth.
    println!("Product: {}", horizontal * depth);
}

// Source of read_lines: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
