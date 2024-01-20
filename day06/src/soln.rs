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

fn find_total_ways_to_win((race_time, race_distance): (u32, u32)) -> u32 {
    let mut total_ways = 0;

    for hold_time in 1..race_time {
        let distance = hold_time * (race_time - hold_time);
        if distance > race_distance {
            total_ways += 1;
        }
    }

    total_ways
}

pub fn process_part1(filename: &str) -> Option<u32> {
    if let Ok(parsed) = parse(filename) {
        let multiplied = parsed
            .iter()
            .map(|race| find_total_ways_to_win(*race))
            .fold(1, |accu, wins| accu * wins);

        return Some(multiplied);
    }

    None
}

pub fn process_part2(_filename: &str) -> Option<u32> {
    Some(0)
}
