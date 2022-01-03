// This program reads in a a file "diagnostic report" and calculates gamma and epsilon rate

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const AMOUNT_OF_BITS: usize = 12;

fn main() {

    let mut temp_diagnostic_report_binary_vector: Vec<String> = Vec::new();

    // Open file and store it into vector
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(current_line) = line {
                temp_diagnostic_report_binary_vector.push(current_line.parse::<String>().unwrap());
            }
        }
    }

    let diagnostic_report_binary_vector = temp_diagnostic_report_binary_vector;

    // Calculate gamma and epsilon rate
    let (gamma_rate, epsilon_rate) = calculate_gamma_and_epsilon_rate(diagnostic_report_binary_vector.clone());

    // Converting gamma and epsilon rate to decimal
    let decimal_epsilon_rate = convert_binary_string_to_decimal(epsilon_rate.clone());
    let decimal_gamma_rate = convert_binary_string_to_decimal(gamma_rate.clone());

    // Part 2 of Day 3

    // Get oxygen generator and CO2 scrubber rating
    let (oxygen_generator_rating, co2_scrubber_rating) = calculate_oxygen_generator_and_co2_scrubber_rating(diagnostic_report_binary_vector.clone());
    
    // Convert oxygen generator and CO2 scrubber rating to decimal
    let decimal_oxygen_generator_rating = convert_binary_string_to_decimal(oxygen_generator_rating.clone());
    let decimal_co2_scrubber_rating = convert_binary_string_to_decimal(co2_scrubber_rating.clone());

    // Print results in string and decimal form
    println!("Gamma rate: {}", gamma_rate);
    println!("Epsilon rate: {}", epsilon_rate);
    println!("Decimal epsilon rate: {}", decimal_epsilon_rate);
    println!("Decimal gamma rate: {}", decimal_gamma_rate);

    // Print product of gamma and epsilon
    println!("Product of gamma and epsilon: {}", decimal_gamma_rate * decimal_epsilon_rate);

    println!("Oxygen generator rating: {}", oxygen_generator_rating);
    println!("Decimal oxygen generator rating: {}", decimal_oxygen_generator_rating);
    println!("CO2 scrubber rating: {}", co2_scrubber_rating);
    println!("Decimal CO2 scrubber rating: {}", decimal_co2_scrubber_rating);

    // Print product of oxygen generator and CO2 scrubber rating
    println!("Product of oxygen generator and CO2 scrubber rating: {}", decimal_oxygen_generator_rating * decimal_co2_scrubber_rating);
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

// This function takes diagnostic_report_binary_vector and returns oxygen generator and CO2
// scrubber rating
fn calculate_oxygen_generator_and_co2_scrubber_rating(temp_diagnostic_report_binary_vector: Vec<String>) -> (String, String) {
  
    // Define return variables
    let mut oxygen_generator_rating: String = String::new();
    let mut co2_scrubber_rating: String = String::new();

    // Copy temp_diagnostic_report_binary_vector into diagnostic_report_binary_vector
    let mut diagnostic_report_binary_vector = temp_diagnostic_report_binary_vector.clone();


    // Calculate oxygen generator rating first

    // Iterate through diagnostic_report_binary_vector bit by bit
    // There are AMOUNT_OF_BITS bits
    for j in 0..AMOUNT_OF_BITS {
        // Print debug information
        println!("j: {}", j);

        // elements_in_diagnostic_report is used to determine whether a bit is common or not
        let elements_in_diagnostic_report = diagnostic_report_binary_vector.len();
    
        // print all elements
        for i in 0..elements_in_diagnostic_report {
            println!("diagnostic_report_binary_vector[{}]: {}", i, diagnostic_report_binary_vector[i]);
        }
        println!("There are {} elements left", elements_in_diagnostic_report);
        // Get the amount of occurences of 1 in the current bit for all values in the vector
        let mut amount_of_1_in_current_bit = 0;
        for i in 0..diagnostic_report_binary_vector.len() {
            if diagnostic_report_binary_vector[i].chars().nth(j).unwrap() == '1' {
                amount_of_1_in_current_bit += 1;
            }
        }

        // Print debug information
        println!("amount_of_1_in_current_bit: {}", amount_of_1_in_current_bit);
        
        
        // If the amount of 1 in the current bit devided by the amount of elements in the vector is larger than 0.5, then it is the most common

        if amount_of_1_in_current_bit as f32 / elements_in_diagnostic_report as f32 >= 0.5 {
            // Remove all elements in the vector that have the current bit set to 0
            diagnostic_report_binary_vector = diagnostic_report_binary_vector.into_iter().filter(|x| x.chars().nth(j).unwrap() == '1').collect();
        } else {
            // Remove all elements in the vector that have the current bit set to 1
            diagnostic_report_binary_vector = diagnostic_report_binary_vector.into_iter().filter(|x| x.chars().nth(j).unwrap() == '0').collect();

        }
        // If there is only one number left in the vector, it is the oxygen scrubber rating
        if diagnostic_report_binary_vector.len() == 1 {
            // Get the index of the oxygen scrubber rating
            let oxygen_generator_rating_index = 0; // there is only one element left
            // Get the oxygen scrubber rating
            oxygen_generator_rating = diagnostic_report_binary_vector[oxygen_generator_rating_index].clone();

            println!("Got oxygen_generator_rating: {}", oxygen_generator_rating);
            break;
        }

    }

    // Calculate CO2 scrubber rating
    // Reset diagnostic_report_binary_vector
    diagnostic_report_binary_vector = temp_diagnostic_report_binary_vector.clone();

    // Iterate through diagnostic_report_binary_vector bit by bit
    // There are AMOUNT_OF_BITS bits
    for j in 0..AMOUNT_OF_BITS {
        // Print debug information
        println!("j: {}", j);

        // elements_in_diagnostic_report is used to determine whether a bit is common or not
        let elements_in_diagnostic_report = diagnostic_report_binary_vector.len();
        
        println!("There are {} elements left", elements_in_diagnostic_report);
        // Get the amount of occurences of 1 in the current bit for all values in the vector
        let mut amount_of_1_in_current_bit = 0;
        for i in 0..diagnostic_report_binary_vector.len() {
            if diagnostic_report_binary_vector[i].chars().nth(j).unwrap() == '1' {
                amount_of_1_in_current_bit += 1;
            }
        }

        // Print debug information
        println!("amount_of_1_in_current_bit: {}", amount_of_1_in_current_bit);
        
        
        // If the amount of 1 in the current bit devided by the amount of elements in the vector is smaller than 0.5, then it is the least common

        if (amount_of_1_in_current_bit as f32 / elements_in_diagnostic_report as f32) < 0.5 {
            // Remove all elements in the vector that have the current bit set to 0
            diagnostic_report_binary_vector = diagnostic_report_binary_vector.into_iter().filter(|x| x.chars().nth(j).unwrap() == '1').collect();
        } else {
            // Remove all elements in the vector that have the current bit set to 1
            diagnostic_report_binary_vector = diagnostic_report_binary_vector.into_iter().filter(|x| x.chars().nth(j).unwrap() == '0').collect();

        }
        // If there is only one number left in the vector, it is the oxygen scrubber rating
        if diagnostic_report_binary_vector.len() == 1 {
            // Get the index of the co2 scrubber rating
            let co2_scrubber_rating_index = 0; // there is only one element left
            // Get the co2 scrubber rating
            co2_scrubber_rating = diagnostic_report_binary_vector[co2_scrubber_rating_index].clone();

            println!("Got co2_scrubber_rating: {}", co2_scrubber_rating);
            break;
        }

    }

    return (oxygen_generator_rating.to_string(),co2_scrubber_rating.to_string());
}

// This function converts a binary string to decimal
fn convert_binary_string_to_decimal(binary_string: String) -> u32 {
    let mut decimal_value = 0;
    for i in 0..binary_string.len() {
        if binary_string.chars().nth(i).unwrap() == '1' {
            decimal_value += 2_u32.pow((binary_string.len()-i-1) as u32); // -1 because we want to start from the right
        }
    }
    decimal_value
}

// This function takes diagnostic_report_binary_vector and returns gamma rate and epsilon rate
fn calculate_gamma_and_epsilon_rate(diagnostic_report_binary_vector: Vec<String>) -> (String, String) {
    // occurance_array_for_ones stores the occurance if ones in the diagnostic report
    // (AMOUNT_OF_BITS bits)
    let mut occurance_array_for_ones: [u32; AMOUNT_OF_BITS] = [0; AMOUNT_OF_BITS];

    // Iterate through the diagnostic report vector
    for i in 0..diagnostic_report_binary_vector.len() {
        // Iterate through the AMOUNT_OF_BITS bits of the current diagnostic report
        for j in 0..AMOUNT_OF_BITS {
            // If the current bit is 1, increment the occurance of ones
            if diagnostic_report_binary_vector[i].chars().nth(j).unwrap() == '1' {
                occurance_array_for_ones[j] += 1;
            }
        }
    }

    let elements_in_diagnostic_report = diagnostic_report_binary_vector.len();

    // calculate gamma rate by dividing the occurance of ones by the total number of elements in the diagnostic report
    let mut gamma_rate = String::new();
    for i in 0..AMOUNT_OF_BITS {
        let current_gamma_rate = (occurance_array_for_ones[i] as f32) / (elements_in_diagnostic_report as f32);

        if current_gamma_rate > 0.5 {
            gamma_rate.push_str("1");
        } else {
            gamma_rate.push_str("0");
        }
    }

    // Epsilon is gammas binary complement
    let mut epsilon_rate = String::new();
    for i in 0..AMOUNT_OF_BITS {
        if gamma_rate.chars().nth(i).unwrap() == '1' {
            epsilon_rate.push('0');
        } else {
            epsilon_rate.push('1');
        }
    }

    return (gamma_rate, epsilon_rate);
}
