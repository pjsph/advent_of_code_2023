use std::{fs::File, io::{BufReader, BufRead}, fmt::{Display, Debug}};

pub fn main() {
    part1();
    part2();
}

struct Card {
    id: u32,
    scratched: Vec<u32>,
    expected: Vec<u32>,
    copies: u32
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}x {:?} {:?}", self.id, self.copies, self.expected, self.scratched)
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Card").field("id", &self.id).field("scratched", &self.scratched).field("expected", &self.expected).field("copies", &self.copies).finish()
    }
}

fn part2() {
    let file = File::open("src/day04/inputs.txt").expect("Error while reading file.");
    let reader = BufReader::new(file);

    let mut cards: Vec<Card> = vec![];
    for line_res in reader.lines() {
        let mut scratched: Vec<u32> = vec![];
        let mut expected: Vec<u32> = vec![];

        let line = line_res.expect("Error while reading line.");

        let mut general_split = line.split(": ");
        let card_id = general_split.next().expect("String expected.").split(" ").filter(|s| !s.is_empty()).nth(1).expect("String expected.").parse::<u32>().expect("Numerical value expected.");

        let mut split = general_split.next().expect("String expected.").split(" | ");

        expected.append(&mut split.next()
                                   .expect("String expected.")
                                   .split(" ")
                                   .filter(|s| !s.is_empty())
                                   .map(|s| s.parse::<u32>().expect("Numerical value expected."))
                                   .collect());

        scratched.append(&mut split.next()
                                   .expect("String expected.")
                                   .split(" ")
                                   .filter(|s| !s.is_empty())
                                   .map(|s| s.parse::<u32>().expect("Numerical value expected."))
                                   .collect());

        cards.push(Card {id: card_id, scratched, expected, copies: 1});
    }

    let mut i = 0usize;
    while i < cards.len() {
        let mut winning_numbers_count = 0usize;
        for s in &cards[i].scratched {
            if cards[i].expected.contains(s) {
                winning_numbers_count += 1;
            }
        }
        for j in 1..(winning_numbers_count+1) {
            cards[i+j].copies += cards[i].copies;
        }
        
        i += 1;
    }

    let total_cards = cards.iter().map(|c| c.copies).sum::<u32>();
    println!("Total number of cards: {}", total_cards);
}

fn part1() {
    let file = File::open("src/day04/inputs.txt").expect("Error while reading file.");
    let reader = BufReader::new(file);

    let mut sum = 0u32;
    for line_res in reader.lines() {
        let mut scratched: Vec<u32> = vec![];
        let mut expected: Vec<u32> = vec![];

        let line = line_res.expect("Error while reading line.");

        let mut general_split = line.split(": ");
        let _game_id = general_split.next();

        let mut split = general_split.next().expect("String expected.").split(" | ");

        expected.append(&mut split.next()
                                   .expect("String expected.")
                                   .split(" ")
                                   .filter(|s| !s.is_empty())
                                   .map(|s| s.parse::<u32>().expect("Numerical value expected."))
                                   .collect());

        scratched.append(&mut split.next()
                                   .expect("String expected.")
                                   .split(" ")
                                   .filter(|s| !s.is_empty())
                                   .map(|s| s.parse::<u32>().expect("Numerical value expected."))
                                   .collect());

        let mut winning_numbers = 0u32;
        for s in scratched {
            if expected.contains(&s) {
                winning_numbers += 1;
            }
        }

        if winning_numbers != 0 {
            sum += 2u32.pow(winning_numbers - 1)
        }

    }

    println!("Total sum: {}", sum);
}