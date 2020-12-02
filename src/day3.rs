
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut result = test_with_slope(1, 1);
    result = result * test_with_slope(3, 1);
    result = result * test_with_slope(5, 1);
    result = result * test_with_slope(7, 1);
    result = result * test_with_slope(1, 2);

    println!("Result: {}", result);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn test_with_slope(right: usize, down: usize) -> u64 {
    let mut line_index: usize = 0;
    let mut tree_count: u64 = 0;
    if let Ok(lines) = read_lines("./inputs/day3.txt") {
        for line in lines.step_by(down) {
            let line = line.unwrap();
            if line.get(line_index..line_index+1) == Some("#") {
                tree_count = tree_count + 1;
            }
            line_index = (line_index + right) % 31;
        }
    }
    println!("Found {} trees for slop (right {}, down {})", tree_count, right, down);
    tree_count
}
