use std::fs::read_to_string;

fn parse(filename: &str) -> Result<Vec<(u32, u32)>, &'static str> {
    if let Ok(contents) = read_to_string(filename) {
        let races: Vec<Vec<u32>> = contents
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|v| v.parse::<u32>().ok())
                    .collect()
            })
            .collect();

        if races.len() != 2 {
            return Err("a race should have time and distance records!");
        }

        let races = races[0]
            .iter()
            .zip(races[1].iter())
            .map(|(&time, &distance)| (time, distance))
            .collect();

        return Ok(races);
    }

    Err("something went wrong during parsing")
}

pub fn process_part1(filename: &str) -> Option<u32> {
    if let Ok(parsed) = parse(filename) {
        println!("{:?}", parsed);

        return Some(0);
    }

    None
}

pub fn process_part2(_filename: &str) -> Option<u32> {
    None
}
