use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    read_passwords();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_passwords() {
    if let Ok(lines) = read_lines("./inputs/day4.txt") {
        let mut current_passport: String = String::from("");
        let mut valid_passport_count: u32 = 0;
        for line in lines {
            let line = line.unwrap();
            if line.is_empty() {
                if current_passport != "" && test_passport(&current_passport){
                    valid_passport_count = valid_passport_count + 1;
                }
                current_passport = String::from("");
            }
            else {
                current_passport = current_passport + " " + &line;
            }
        }
        if current_passport != "" && test_passport(&current_passport){
            valid_passport_count = valid_passport_count + 1;
        }
        println!("Found {} valid passports", valid_passport_count);
    }
}

fn test_passport(passport: &String) -> bool {
    println!("Testing passport: {}", passport);

    let birth_year: String = get_passport_property("byr", passport);
    let birth_year = match birth_year.parse::<u32>() {
        Err(_) => return false,
        Ok(v) => {
            if v < 1920 || v > 2002 { return false }
            else { v }
        }
    };
    let issue_year = get_passport_property("iyr", passport);
    let issue_year = match issue_year.parse::<u32>() {
        Err(_) => return false,
        Ok(v) => {
            if v < 2010 || v > 2020 { return false }
            else { v }
        }
    };
    let expiration_year = get_passport_property("eyr", passport);
    let expiration_year  = match expiration_year.parse::<u32>() {
        Err(_) => return false,
        Ok(v) => {
            if v < 2020 || v > 2030 { return false }
            else { v }
        }
    };
    let height = get_passport_property("hgt", passport);
    if height.contains("in")  {
        let height = height.trim_end_matches("in").parse::<u32>().unwrap();
        if height < 59 || height > 76 {
            return false;
        }
    } else if height.contains("cm") {
        let height = height.trim_end_matches("cm").parse::<u32>().unwrap();
        if height < 150 || height > 193 {
            return false;
        }
    } else {
        return false;
    }
    let hair_color = get_passport_hair_color(passport);
    if hair_color == ""  {
        return false;
    }
    let eye_color = get_passport_property("ecl", passport);
    if eye_color != "amb" &&
        eye_color != "blu" &&
        eye_color != "brn" &&
        eye_color != "gry" &&
        eye_color != "grn" &&
        eye_color != "hzl" &&
        eye_color != "oth" {
        return false;
    }
    let passport_id = get_passport_id(passport);
    if passport_id == ""  {
        return false;
    }
    let country_id = get_passport_property("cid", passport);

    println!("Valid");
    true
}

fn get_passport_property(property: &str, passport: &String) -> String {
    let re: Regex = Regex::new(&format!(r"{}:([^\s]+)", property)).unwrap();
    let cap = match re.captures(passport) {
        None => return String::from(""),
        Some(v) => v
    };
    String::from(cap.get(1).map_or("", |m| m.as_str()))
}

fn get_passport_hair_color(passport: &String) -> String {
    let re: Regex = Regex::new(r"hcl:(#[0-9a-f]{6})(?:\s|$)").unwrap();
    let cap = match re.captures(passport) {
        None => return String::from(""),
        Some(v) => v
    };
    String::from(cap.get(1).map_or("", |m| m.as_str()))
}

fn get_passport_id(passport: &String) -> String {
    let re: Regex = Regex::new(r"pid:([0-9]{9})(?:\s|$)").unwrap();
    let cap = match re.captures(passport) {
        None => return String::from(""),
        Some(v) => v
    };
    String::from(cap.get(1).map_or("", |m| m.as_str()))
}
