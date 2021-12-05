use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main() {
    // Create depth i32 vector
    let mut depth: Vec<i32> = Vec::new();

    // Open file and store it into vector
    if let Ok(lines) = read_lines("./input.txt") {
        for line in lines {
            if let Ok(current_depth) = line {
                    depth.push(current_depth.parse::<i32>().unwrap());
                }
            }
        }
        
    let depth_increased = depth_increased(depth);
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

// This function takes a depth vector and return the number of times the depth has increased
fn depth_increased(depth: Vec<i32>) -> i32 {
    let mut depth_increased = 0;
    let mut last_depth = -1;

    for depth in depth {
        if last_depth == -1 {
            last_depth = depth;
        } else {
            if depth > last_depth {
                depth_increased += 1;
            }
            last_depth = depth;
        }
    }
    return depth_increased;
}
