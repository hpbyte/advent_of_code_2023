use std::fs::read_to_string;
use std::io::Error;

fn parse(filename: &str) -> Result<Vec<Vec<u32>>, Error> {
    let contents = read_to_string(filename)?;

    let races: Vec<Vec<u32>> = contents
        .split("\n")
        .map(|line| {
            line.split_whitespace()
                .filter_map(|word| word.parse::<u32>().ok())
                .collect()
        })
        .collect();

    Ok(races)
}

pub fn process_part1(_filename: &str) -> Option<u32> {
    None
}

pub fn process_part2(_filename: &str) -> Option<u32> {
    None
}
