use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn main() {
    part1();
    part2();
}

fn part2() {
    let file = File::open("src/day02/inputs.txt").expect("Error while reading file.");
    let reader = BufReader::new(file);
    
    let mut sum = 0u32;
    for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");

        let mut first_split = line.split(": ");

        let records = first_split.nth(1).expect("Records not found.").split("; ");
        let mut max_blue = 0u32;
        let mut max_red = 0u32;
        let mut max_green = 0u32;
        for record in records {
            let draws = record.split(", ");
            for draw in draws {
                let mut draw_split = draw.split(" ");
                let number = draw_split.next().expect("Numerical value not found.").parse::<u32>().expect("Numerical value expected.");
                let color = draw_split.next().expect("Color not found.");

                if color == "red" && number > max_red {
                    max_red = number;
                    continue;
                }

                if color == "blue" && number > max_blue {
                    max_blue = number;
                    continue;
                }

                if color == "green" && number > max_green {
                    max_green = number;
                    continue;
                }
            }
        }
        
        sum += max_blue * max_green * max_red;
    }

    println!("Total sum for part2: {}", sum);
}

fn part1() {
    let file = File::open("src/day02/inputs.txt").expect("Error while reading file.");
    let reader = BufReader::new(file);
    
    let mut sum = 0u32;
    for line_res in reader.lines() {
        let line = line_res.expect("Error while reading line.");

        let mut first_split = line.split(": ");
        let game_id = first_split.next().expect("Game id not found.").split(" ").nth(1).expect("Numerical value not found.").parse::<u32>().expect("Numerical value expected.");

        let records = first_split.next().expect("Records not found.").split("; ");
        let mut record_possible = true;
        for record in records {
            let draws = record.split(", ");
            let mut draw_possible = true;
            for draw in draws {
                let mut draw_split = draw.split(" ");
                let number = draw_split.next().expect("Numerical value not found.").parse::<u32>().expect("Numerical value expected.");
                let color = draw_split.next().expect("Color not found.");

                if color == "red" && number > 12 {
                    draw_possible = false;
                    break;
                }

                if color == "blue" && number > 14 {
                    draw_possible = false;
                    break;
                }

                if color == "green" && number > 13 {
                    draw_possible = false;
                    break;
                }
            }

            if !draw_possible {
                record_possible = false;
                break;
            }
        }
        
        if record_possible {
            sum += game_id;
        }
    }

    println!("Total sum for part1: {}", sum);
}