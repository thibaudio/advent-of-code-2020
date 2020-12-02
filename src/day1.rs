use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

fn main() {
    // File hosts must exist in current path before this produces output
    let numbers = read_lines("./inputs/day1.txt");
    for (index, number) in numbers.iter().enumerate() {
        for (other_index, other_number) in numbers.iter().by_ref().enumerate().filter(|&(i, _)| i != index).map(|(i, v)| (i, v)) {
            for other_other_number in numbers.iter().by_ref().enumerate().filter(|&(i, _)| i != index && i != other_index).map(|(_, v)| v) {
                if number + other_number + other_other_number == 2020 {
                    println!("{} * {} * {} = {}", number, other_number, other_other_number, number * other_number * other_other_number);
                }
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines(filename: impl AsRef<Path>) -> Vec<u32> {
    let file = File::open(filename).expect("file not found");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|l| l.parse().unwrap())
        .collect()
}
