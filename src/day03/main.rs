use std::{fs::File, io::{BufReader, BufRead}, str::FromStr, fmt::Display};

struct Number {
    nbr: u32,
    indexes: Vec<usize>,
    counted: bool
}

struct Symbol {
    sym: char,
    index: usize
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {:?}", self.nbr, self.indexes)
    }
}

pub fn main() {
    let file = File::open("src/day03/inputs.txt").expect("Error while reading file.");
    let reader = BufReader::new(file);

    let mut numbers: Vec<Vec<Number>> = vec![];
    let mut symbols: Vec<Vec<Symbol>> = vec![];
    for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");
        
        numbers.push(parse_numbers(&line));
        symbols.push(parse_symbols(&line));
    }

    let mut sum = 0u32;
    let mut gear_sum = 0u32;
    for i in 0..symbols.len() {
        for symbol in &symbols[i] {
            let mut gear_nbr: Vec<u32> = vec![];

            for number in &mut numbers[i] {
                if number.counted {
                    continue;
                }
                if number.indexes.first().unwrap() == &(symbol.index + 1) || symbol.index > 0 && number.indexes.last().unwrap() == &(symbol.index - 1) {
                    sum += number.nbr;
                    number.counted = true;
                    if symbol.sym == '*' {
                        gear_nbr.push(number.nbr);
                    }
                }
            }

            if i > 0 {
                for number in &mut numbers[i - 1] {
                    if number.counted {
                        continue;
                    }
                    if number.indexes.first().unwrap() == &(symbol.index + 1) || number.indexes.contains(&symbol.index) || symbol.index > 0 && number.indexes.last().unwrap() == &(symbol.index - 1) {
                        sum += number.nbr;
                        number.counted = true;
                        if symbol.sym == '*' {
                            gear_nbr.push(number.nbr);
                        }
                    }
                }
            }

            if i + 1 < numbers.len() {
                for number in &mut numbers[i + 1] {
                    if number.counted {
                        continue;
                    }
                    if number.indexes.first().unwrap() == &(symbol.index + 1) || number.indexes.contains(&symbol.index) || symbol.index > 0 && number.indexes.last().unwrap() == &(symbol.index - 1) {
                        sum += number.nbr;
                        number.counted = true;
                        if symbol.sym == '*' {
                            gear_nbr.push(number.nbr);
                        }
                    }
                }
            }

            if gear_nbr.len() == 2 {
                gear_sum += gear_nbr.iter().product::<u32>();
            }
        }
    }

    println!("Total sum: {}", sum);
    println!("Total gear sum: {}", gear_sum);
}

fn parse_numbers(s: &str) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];

    let mut new_nbr = true;
    for i in 0..s.len() {
        if is_numerical(char::from_str(&s[i..i+1]).expect("String is empty.")) {
            if new_nbr {
                numbers.push(Number { nbr: s[i..i+1].parse::<u32>().expect("Numerical value expected."), indexes: vec![i], counted: false });
                new_nbr = false;
            } else {
                let nbr = numbers.last_mut().unwrap();
                nbr.nbr = 10 * nbr.nbr + s[i..i+1].parse::<u32>().expect("Numerical value expected.");
                nbr.indexes.push(i);
            }
        } else {
            new_nbr = true;
        }
    }

    numbers
}

fn parse_symbols(s: &str) -> Vec<Symbol> {
    let mut symbols: Vec<Symbol> = vec![];

    for i in 0..s.len() {
        let c = char::from_str(&s[i..i+1]).expect("String is empty.");
        if !is_numerical(c) && c != '.' {
            symbols.push(Symbol { sym: c, index: i });
        }
    }

    symbols
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