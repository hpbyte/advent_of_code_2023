use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// five of a kind   AAAAA
// four of a kind   AAAA1
// full house       AAA11
// three of a kind  AAA12
// two pair         AA112
// one pair         AA123
// high card        A1234

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

fn get_ranks(hand: &Hand, j: usize) -> [usize; 5] {
    hand.cards.map(|card| match card {
        b'A' => 14,
        b'K' => 13,
        b'Q' => 12,
        b'J' => j,
        b'T' => 10,
        _ => card.wrapping_sub(b'0') as usize,
    })
}

fn sort(hands: &[Hand], j: usize) -> Vec<(usize, usize)> {
    let mut ranked: Vec<(usize, usize)> = hands
        .iter()
        .map(|hand| {
            let rank = get_ranks(hand, j);

            let mut freq = [0; 15];
            rank.iter().for_each(|&r| freq[r as usize] += 1);

            let jokers = freq[1];
            freq[1] = 0;
            freq.sort_unstable();
            freq.reverse();
            freq[0] += jokers;

            let mut key = 0;

            for f in freq.iter().take(5) {
                key = (key << 4) | f;
            }
            for r in rank {
                key = (key << 4) | r;
            }

            (key, hand.bid)
        })
        .collect();

    ranked.sort_unstable();

    ranked
}

pub fn process_part1(filename: &str) -> Option<usize> {
    if let Ok(lines) = read_lines(filename) {
        let hands = lines
            .filter_map(|line| line.ok())
            .map(|line| parse(&line))
            .collect::<Vec<Hand>>();

        let total = sort(&hands, 11)
            .iter()
            .enumerate()
            .map(|(rank, (_, bid))| (rank + 1) * bid)
            .sum();

        return Some(total);
    }

    None
}

pub fn process_part2(filename: &str) -> Option<usize> {
    if let Ok(lines) = read_lines(filename) {
        let hands = lines
            .filter_map(|line| line.ok())
            .map(|line| parse(&line))
            .collect::<Vec<Hand>>();

        let total = sort(&hands, 1)
            .iter()
            .enumerate()
            .map(|(rank, (_, bid))| (rank + 1) * bid)
            .sum();

        return Some(total);
    }

    None
}
