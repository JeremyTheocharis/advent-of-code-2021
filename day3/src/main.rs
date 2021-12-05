// This program reads in a a file "diagnostic report" and calculates gamma and epsilon rate

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    let mut diagnostic_report_binary_vector: Vec<String> = Vec::new();
    
    // occurance_array_for_ones stores the occurance if ones in the diagnostic report
    // (12 bits)
    let mut occurance_array_for_ones: [u32; 12] = [0; 12];

    // Open file and store it into vector
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(current_line) = line {
                diagnostic_report_binary_vector.push(current_line.parse::<String>().unwrap());
            }
        }
    }

    // Calculate gamma rate

    // Iterate through the diagnostic report vector
    for i in 0..diagnostic_report_binary_vector.len() {
        // Iterate through the 12 bits of the current diagnostic report
        for j in 0..12 {
            // If the current bit is 1, increment the occurance of ones
            if diagnostic_report_binary_vector[i].chars().nth(j).unwrap() == '1' {
                occurance_array_for_ones[j] += 1;
            }
        }
    }

    let elements_in_diagnostic_report = diagnostic_report_binary_vector.len();

    // calculate gamma rate by dividing the occurance of ones by the total number of elements in the diagnostic report
    for i in 0..12 {
        let current_gamma_rate = (occurance_array_for_ones[i] as f32) / (elements_in_diagnostic_report as f32);
        // Print debug information
        println!("{} {} {} {}", i, current_gamma_rate, occurance_array_for_ones[i], elements_in_diagnostic_report);
        if current_gamma_rate > 0.5 {
            gamma_rate.push_str("1");
        } else {
            gamma_rate.push_str("0");
        }
    }

    // Epsilon is gammas binary complement
    for i in 0..12 {
        if gamma_rate.chars().nth(i).unwrap() == '1' {
            epsilon_rate.push('0');
        } else {
            epsilon_rate.push('1');
        }
    }

    // Convert epsilon rate to decimal
    let mut decimal_epsilon_rate = 0;
    for i in 0..12 {
        if epsilon_rate.chars().nth(i).unwrap() == '1' {
            decimal_epsilon_rate += 2_u32.pow((11-i) as u32); //  11-i because we want to start from the right
        }
    }

    // Convert gamma rate to decimal
    let mut decimal_gamma_rate = 0;
    for i in 0..12 {
        if gamma_rate.chars().nth(i).unwrap() == '1' {
            decimal_gamma_rate += 2_u32.pow((11-i) as u32); // 11-i because we want to start from the right
        }
    }

    // Print results in string and decimal form
    println!("Gamma rate: {}", gamma_rate);
    println!("Epsilon rate: {}", epsilon_rate);
    println!("Decimal epsilon rate: {}", decimal_epsilon_rate);
    println!("Decimal gamma rate: {}", decimal_gamma_rate);

    // Print product of gamma and epsilon
    println!("Product of gamma and epsilon: {}", decimal_gamma_rate * decimal_epsilon_rate);

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
