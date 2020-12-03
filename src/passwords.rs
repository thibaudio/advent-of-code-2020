use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+) (\w): (.+)").unwrap();
    }
    let mut count: u32 = 0;
    if let Ok(lines) = read_lines("./day2input.txt") {
        for line in lines {
            let line = line.unwrap();
            println!("Checking line: {}", line);
            let cap = RE.captures(&line).unwrap();

            let min = cap.get(1).map_or("",  |m| m.as_str());
            let max = cap.get(2).map_or("",  |m| m.as_str());
            let chara = cap.get(3).map_or("",  |m| m.as_str());
            let password = cap.get(4).map_or("",  |m| m.as_str());
            let c = password.matches(chara).count();
            println!("Found password: {}\nmin: {}\nmax: {}\nchara: {}", password, min, max, chara);
            if c >= min.parse().unwrap() && c <= max.parse().unwrap() {
                count = count + 1;
            }



        }
    }
    println!("Number of valid passwords: {}", count);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
