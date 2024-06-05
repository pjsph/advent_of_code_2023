use std::{fs::File, io::{BufReader, BufRead}, collections::HashMap};

pub fn main() {
    let file = File::open("src/day05/inputs.txt").expect("Error while reading file.");
    let mut lines = BufReader::new(file).lines();

    let seeds: Vec<u32> = lines.next().expect("Line expected.").expect("String expected").split(": ").nth(1).expect("String expected.").split(" ").map(|s| s.parse::<u32>().expect("Numerical value expected.")).collect();

    let mut maps: HashMap<String, Vec<(u32, u32, u32)>> = HashMap::new();
    let mut current_key = None;
    for line_res in lines {
        let line = line_res.expect("Line expected.");

        if current_key.is_none() && line.ends_with("map:") {
            let key = line.split(" ").next().expect("String expected.").to_string();
            current_key = Some(key.clone());
            maps.insert(key, vec![]);
            continue;
        }

        if current_key.is_some() && line.is_empty() {
            current_key = None;
            continue;
        }

        if let Some(ref key) = current_key {
            let mut values = line.split(" ").map(|s| s.parse::<u32>().expect("Numerical value expected."));
            maps.get_mut(key).expect("Vector of (u32, u32, u32) expected.").push((values.next().expect("Value 0 expected."), values.next().expect("Value 1 expected."), values.next().expect("Value 2 expected.")));
        }
    }

    let full_walkthrough = |s| walkthrough_map(
        walkthrough_map(
         walkthrough_map(
          walkthrough_map(
           walkthrough_map(
            walkthrough_map(
             walkthrough_map(
             s, 
             maps.get("seed-to-soil").expect("Vec<(u32, u32, u32) expected.")), 
             maps.get("soil-to-fertilizer").expect("Vec<(u32, u32, u32) expected.")), 
             maps.get("fertilizer-to-water").expect("Vec<(u32, u32, u32) expected.")), 
             maps.get("water-to-light").expect("Vec<(u32, u32, u32) expected.")), 
             maps.get("light-to-temperature").expect("Vec<(u32, u32, u32) expected.")), 
             maps.get("temperature-to-humidity").expect("Vec<(u32, u32, u32) expected.")), 
             maps.get("humidity-to-location").expect("Vec<(u32, u32, u32) expected."));

    let locations: Vec<u32> = seeds.iter().map(|s| full_walkthrough(*s)).collect();

    println!("Min location for part1: {}", locations.iter().min().expect("Minimum not found."));

    let map_order = vec!["seed-to-soil", "soil-to-fertilizer", "fertilizer-to-water", "water-to-light", "light-to-temperature", "temperature-to-humidity", "humidity-to-location"];
    let mut i = 0;
    let mut min = 0;
    while i < seeds.len() - 1 {
        let mut output_ranges: Vec<(u32, u32)> = vec![(seeds[i], seeds[i+1])];
        for map_str in &map_order {
            let map = maps.get(*map_str).expect("Vec(u32, u32, u32) expected.");
            let mut temp_ranges: Vec<(u32, u32)> = vec![];
            for range in output_ranges {
                temp_ranges.append(&mut walkthrough_map_range(range.0, range.1, map));
            }
            output_ranges = temp_ranges;
        }

        if !output_ranges.is_empty() {
            let temp = output_ranges.iter().map(|(a, _)| a).min().expect("Output values are empty.");
            if min == 0 || *temp < min {
                min = *temp;
            }
        }

        i += 2;
    }

    println!("Min location for part2: {}", min);

    // maps.insert("test".to_string(), vec![(500, 100, 50), (1000, 50, 25), (1500, 175, 25)]);
    // println!("{:?}", walkthrough_map_range(52, 77, maps.get("test").unwrap()));
}

fn walkthrough_map(input: u32, map: &Vec<(u32, u32, u32)>) -> u32 {
    for (dest_start, source_start, range) in map {
        if input >= *source_start && (input as u64) < (*source_start as u64 + *range as u64) {
            return input - source_start + dest_start;
        }
    }

    return input;
}

fn walkthrough_map_range(input: u32, input_range: u32, map: &Vec<(u32, u32, u32)>) -> Vec<(u32, u32)> {
    let mut input_ranges: Vec<(u32, u32)> = vec![];
    let mut output_ranges: Vec<(u32, u32)> = vec![];
    for (dest_start, source_start, range) in map {
        if input >= *source_start && (input as u64 + input_range as u64) < (*source_start as u64 + *range as u64) {
            output_ranges.push((input - *source_start + *dest_start, input_range));
            input_ranges.push((input, input_range));
            break;
        }else if input >= *source_start && (input as u64) < (*source_start as u64 + *range as u64) {
            output_ranges.push((input - *source_start + *dest_start, (*source_start as u64 + *range as u64 - input as u64) as u32));
            input_ranges.push((input, (*source_start as u64 + *range as u64 - input as u64) as u32));
        } else if (input as u64 + input_range as u64) > (*source_start as u64) && (input as u64 + input_range as u64) < (*source_start as u64 + *range as u64) {
            output_ranges.push((*dest_start, (input as u64 + input_range as u64 - *source_start as u64) as u32));
            input_ranges.push((*source_start, (input as u64 + input_range as u64 - *source_start as u64) as u32));
        } else if input < *source_start && (input as u64 + input_range as u64) >= (*source_start as u64 + *range as u64) {
            output_ranges.push((*dest_start, *range));
            input_ranges.push((*source_start, *range));
        }
    }

    input_ranges.sort_by(|(a, _), (b, _)| a.cmp(b));
    let mut i = 0;
    while i < input_ranges.len() {
        if i == 0 {
            if input_ranges[0].0 > input {
                output_ranges.push((input, input_ranges[0].0));
            }
        } else if i == input_ranges.len() - 1 {
            if (input_ranges[i].0 as u64 + input_ranges[i].1 as u64) < (input as u64 + input_range as u64) {
                output_ranges.push((input_ranges[i].0 + input_ranges[i].1, (input as u64 + input_range as u64 - input_ranges[i].0 as u64 - input_ranges[i].1 as u64) as u32));
            }
        } 

        if i < input_ranges.len() - 1 {
            if (input_ranges[i].0 as u64 + input_ranges[i].1 as u64) < (input_ranges[i+1].0 as u64) {
                output_ranges.push((input_ranges[i].0 + input_ranges[i].1, input_ranges[i+1].0 - input_ranges[i].0 - input_ranges[i].1))
            }
        }

        i += 1;
    }

    output_ranges
}