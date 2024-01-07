use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse(card: &str) -> Result<(HashSet<String>, Vec<String>), &'static str> {
    let card = match card.find(':') {
        Some(index) => &card[(index + 1)..],
        None => return Err("Card prefix not found"),
    };

    let parts: Vec<&str> = card.split("|").collect();
    if parts.len() != 2 {
        return Err("Invalid card format");
    }

    let winning_numbers = parts[0].trim().split_whitespace().map(str::to_string).fold(
        HashSet::new(),
        |mut accu, num| {
            accu.insert(num);
            accu
        },
    );

    let numbers = parts[1]
        .trim()
        .split_whitespace()
        .map(str::to_string)
        .collect::<Vec<String>>();

    Ok((winning_numbers, numbers))
}

fn find_match_count(parsed_card: &(HashSet<String>, Vec<String>)) -> u32 {
    let (winning_numbers, numbers) = parsed_card;
    numbers
        .iter()
        .filter(|num| winning_numbers.contains(*num))
        .count() as u32
}

fn calculate_points(count: u32) -> u32 {
    if count == 0 {
        return 0;
    }

    2_u32.pow(count - 1)
}

pub fn process_part1(filename: &str) -> Option<u32> {
    if let Ok(lines) = read_lines(filename) {
        let sum = lines
            .filter_map(|line_result| line_result.ok())
            .filter_map(|line| parse(&line).ok())
            .map(|card| calculate_points(find_match_count(&card)))
            .sum::<u32>();

        return Some(sum);
    }

    None
}

pub fn process_part2(filename: &str) -> Option<u32> {
    if let Ok(lines) = read_lines(filename) {
        let total_cards = lines
            .filter_map(|line_result| line_result.ok())
            .filter_map(|line| parse(&line).ok())
            .map(|card| find_match_count(&card))
            .enumerate()
            .fold(
                HashMap::new(),
                |mut accu: HashMap<usize, u32>, (index, match_count)| {
                    let card_no = index + 1;

                    // no of iterations
                    accu.entry(card_no)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);

                    for _ in 0..accu[&card_no] {
                        for index in 0..(match_count) as usize {
                            let key = card_no + index + 1;
                            accu.entry(key).and_modify(|count| *count += 1).or_insert(1);
                        }
                    }

                    accu
                },
            )
            .into_values()
            .sum::<u32>();

        return Some(total_cards);
    }

    None
}
