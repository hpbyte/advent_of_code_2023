use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse(line: &str) -> Hand {
    let (cards, bid) = line.split_at(5);
    let cards = cards.chars().collect();
    let bid = bid.trim().parse().ok().unwrap();

    Hand { cards, bid }
}

fn to_num(hand: Hand) -> (usize, usize) {
    let nums = hand
        .cards
        .iter()
        .map(|card| match card {
            'A' => "14".to_string(),
            'K' => "13".to_string(),
            'Q' => "12".to_string(),
            'J' => "11".to_string(),
            'T' => "10".to_string(),
            _ if card.is_digit(10) => card.to_string(),
            _ => unreachable!(),
        })
        .collect::<String>()
        .parse::<usize>()
        .unwrap_or(0);

    (nums, hand.bid)
}

pub fn process_part1(filename: &str) -> Option<u64> {
    if let Ok(lines) = read_lines(filename) {
        let parsed: Vec<(usize, usize)> = lines
            .filter_map(|line| line.ok())
            .map(|line| to_num(parse(&line)))
            .collect();

        println!("{:?}", parsed);

        return Some(0);
    }

    None
}

pub fn process_part2(_filename: &str) -> Option<u64> {
    Some(0)
}
