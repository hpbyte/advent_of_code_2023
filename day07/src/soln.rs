use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
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
    let cards = cards.as_bytes().try_into().unwrap();
    let bid = bid.trim().parse().ok().unwrap();

    Hand { cards, bid }
}

fn substitute(hand: &mut Hand) {
    for card in &mut hand.cards {
        *card = match card {
            b'A' => b'>',
            b'K' => b'=',
            b'Q' => b'<',
            b'J' => b';',
            b'T' => b':',
            _ => *card,
        }
    }
}

pub fn process_part1(filename: &str) -> Option<u64> {
    if let Ok(lines) = read_lines(filename) {
        let mut hands = lines
            .filter_map(|line| line.ok())
            .map(|line| parse(&line))
            .collect::<Vec<Hand>>();

        hands.iter_mut().for_each(|hand| substitute(hand));

        println!("after: {:?}", hands);

        return Some(0);
    }

    None
}

pub fn process_part2(_filename: &str) -> Option<u64> {
    Some(0)
}
