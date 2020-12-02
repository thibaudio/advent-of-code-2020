use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("inputs/day6.txt") {
        let mut current_group = Vec::new();
        let mut count: u32 = 0;
        let mut final_count = 0;
        let mut group_size = 0;
        for l in lines {
            let l = l.unwrap();
            if l.is_empty() {
                final_count = final_count + answers(&current_group, group_size);
                current_group.sort();
                current_group.dedup();
                count = count + current_group.len() as u32;
                current_group.clear();
                group_size = 0;
            } else {
                group_size = group_size + 1;
                for c in l.chars() {
                    current_group.push(c);
                }
            }
        }
        final_count = final_count + answers(&current_group, group_size);
        current_group.sort();
        current_group.dedup();
        count = count + current_group.len() as u32;
        println!("Total: {}", count);
        println!("Total final: {}", final_count);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn answers(current_group: &Vec<char>, group_size: usize) -> u32 {
    let mut final_count = 0;
    let mut copy = current_group.to_vec();
    copy.sort();
    copy.dedup();
    for a in copy {
        let count = current_group.iter().filter(|x| **x == a).count();
        if count == group_size {
            final_count = final_count + 1;
        }
    }
    final_count
}
