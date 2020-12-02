use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            ^(?P<min>\d)-(?P<max>\d) (?P<char>\c): (?P<password>.*)$
            ").unwrap();
    }
    let count: u32 = 0;
    if let Ok(lines) = read_lines("./hosts") {
        for line in lines {
            RE.captures(&line.unwrap()).and_then(|cap| {
                let min: usize = cap.name("min").as_str().parse().unwrap();
                let max: usize = cap.name("max").as_str().parse().unwrap();
                let chara: char = cap.name("char").as_str().parse().unwrap();
                let password: String = cap.name("password").as_str();
                let c = password.matches(chara).count();
                if c >= min && c <= max {
                    count = count + 1;
                }

            })

        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
