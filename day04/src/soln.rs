use std::collections::HashSet;
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

fn calculate_points(card: &str) -> Result<i32, &'static str> {
    let card = match card.find(':') {
        Some(index) => &card[(index + 1)..],
        None => return Err("Card prefix not found"),
    };

    let parts: Vec<&str> = card.split("|").collect();
    if parts.len() != 2 {
        return Err("Invalid card format");
    }

    let winning_numbers =
        parts[0]
            .trim()
            .split_whitespace()
            .fold(HashSet::new(), |mut accu, num| {
                accu.insert(num);
                accu
            });

    let count = parts[1]
        .trim()
        .split_whitespace()
        .filter(|num| winning_numbers.contains(num))
        .count() as u32;

    if count == 0 {
        return Ok(0);
    }

    Ok(2_i32.pow(count - 1))
}

pub fn process_part1(filename: &str) -> Option<i32> {
    if let Ok(lines) = read_lines(filename) {
        let sum = lines
            .filter_map(|line_result| line_result.ok())
            .map(|line| match calculate_points(&line) {
                Ok(points) => points,
                Err(_) => 0,
            })
            .sum();

        return Some(sum);
    }

    None
}

pub fn process_part2(_filename: &str) -> Option<i32> {
    Some(0)
}
