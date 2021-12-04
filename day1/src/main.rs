use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    // Define variable depth_increased
    let mut depth_increased = 0;

    // Define variable last_depth
    let mut last_depth = -1;


    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(depth) = line {
                // println!("{}", depth);

                // Set last_depth to the first value
                if last_depth == -1 {
                    last_depth = depth.parse::<i32>().unwrap();
                } else {
                    // If the current depth is greater than the last depth, increase the depth_increased
                    if depth.parse::<i32>().unwrap() > last_depth {
                        depth_increased += 1;
                    }
                    // Set last_depth to the current depth
                    last_depth = depth.parse::<i32>().unwrap();
                }
            }
        }
        
    }
    println!("{}", depth_increased);

}
// Source of read_lines: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
