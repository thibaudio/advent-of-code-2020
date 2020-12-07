use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ops::Range;

fn main() {
    if let Ok(lines) = read_lines("inputs/day5.txt") {
        let mut high_seat_id = 0;
        let mut seats = Vec::new();
        for l in lines {
            let l = l.unwrap();
            let seat_id = decode_seat(&l);
            seats.push(seat_id);
            if seat_id > high_seat_id {
                high_seat_id = seat_id;
            }
        }
        println!("Highest seat id: {}", high_seat_id);

        let mut my_seat = 0;
        for seat in 1..947 {
            if seats.contains(&seat) {
                continue;
            }
            if seats.contains(&(seat - 1)) && seats.contains(&(seat + 1)) {
                my_seat = seat;
                break;
            }
        }

        println!("My seat: {}", my_seat);

    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn decode_seat(seat_code: &String) -> usize {
    let mut row_range: Range<usize> = 0..127;
    let mut col_range: Range<usize> = 0..7;
    for (i, c) in seat_code.chars().enumerate() {
        if i < 7 {
            row_range = update_range(row_range, c);
        }
        else if i < 10 {
            col_range = update_range(col_range, c);
        }
        else {
            panic!("Incorrect seat_code");
        }
    }


    let seat_id = row_range.start * 8 + col_range.start;
    println!("{}: {} {} {}", seat_code, row_range.start, col_range.start, seat_id);
    return seat_id;
}

fn update_range(row_range: Range<usize>, c: char) -> Range<usize> {
    if c == 'F'  || c == 'L' {
        let new_bound:usize = row_range.start + ((row_range.end - row_range.start) as f64 / 2.0).floor() as usize;
        return row_range.start..new_bound;
    }
    else if c == 'B' || c == 'R' {
        let new_bound:usize = row_range.start + ((row_range.end - row_range.start) as f64 / 2.0).ceil() as usize;
        return new_bound..row_range.end;
    }
    else {
        panic!("Unkown character")
    }
}
