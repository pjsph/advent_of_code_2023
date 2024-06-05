use std::fs::File;
use std::io::{prelude::*, BufReader, Error};
use std::str::FromStr;

pub fn main() {
    let file = File::open("src/day01/inputs.txt").expect("Error while reading file.");
    let reader = BufReader::new(file);
    
    let mut sum = 0u32;
    for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");

        let mut first = Err(Error::new(std::io::ErrorKind::NotFound, "Numerical value not found."));
        for i in 0..line.len() {
            let c = char::from_str(&line[i..i+1]).expect("String is empty.");
            if is_numerical(c) {
                first = Ok(c.to_digit(10).expect("Failed to convert char to int."));
                break;
            }

            let parsed = parse_number(&line[i..line.len()]);
            if let Ok(nbr) = parsed {
                first = Ok(nbr);
                break;
            }
        }

        let mut last = Err(Error::new(std::io::ErrorKind::NotFound, "Numerical value not found."));
        for i in (0..line.len()).rev() {
            let c = char::from_str(&line[i..i+1]).expect("String is empty.");
            if is_numerical(c) {
                last = Ok(c.to_digit(10).expect("Failed to convert char to int."));
                break;
            }

            let parsed = parse_number(&line[i..line.len()]);
            if let Ok(nbr) = parsed {
                last = Ok(nbr);
                break;
            }
        }

        if first.is_ok() && last.is_ok() {
            sum += first.unwrap() * 10 + last.unwrap();
        } else {
            println!("Numerical values not found.");
        }
    }

    println!("Total sum: {}", sum);
}

fn is_numerical(c: char) -> bool {
    return c == '0' ||
           c == '1' ||
           c == '2' ||
           c == '3' ||
           c == '4' ||
           c == '5' ||
           c == '6' ||
           c == '7' ||
           c == '8' ||
           c == '9';
}

fn parse_number(s: &str) -> Result<u32, ()> {
    if s.starts_with("one") {
        return Ok(1);
    }
    if s.starts_with("two") {
        return Ok(2);
    }
    if s.starts_with("three") {
        return Ok(3);
    }
    if s.starts_with("four") {
        return Ok(4);
    }
    if s.starts_with("five") {
        return Ok(5);
    }
    if s.starts_with("six") {
        return Ok(6);
    }
    if s.starts_with("seven") {
        return Ok(7);
    }
    if s.starts_with("eight") {
        return Ok(8);
    }
    if s.starts_with("nine") {
        return Ok(9);
    }

    Err(())
}