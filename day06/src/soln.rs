use std::fs::read_to_string;

enum ParsedResult {
    TupleVec(Vec<(u64, u64)>),
    VecNums(Vec<u64>),
}

fn parse(filename: &str, zipped: bool) -> Result<ParsedResult, &'static str> {
    if let Ok(contents) = read_to_string(filename) {
        let races: Vec<Vec<u64>> = contents
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .filter_map(|v| v.parse::<u64>().ok())
                    .collect()
            })
            .collect();

        if races.len() != 2 {
            return Err("a race should have time and distance records!");
        }

        if zipped {
            let races = races[0]
                .iter()
                .zip(races[1].iter())
                .map(|(&time, &distance)| (time, distance))
                .collect();

            return Ok(ParsedResult::TupleVec(races));
        } else {
            let races: Result<Vec<u64>, &'static str> = races
                .iter()
                .map(|race_vec| {
                    race_vec
                        .iter()
                        .map(|n| n.to_string())
                        .collect::<String>()
                        .parse::<u64>()
                        .map_err(|_| "failed to parse the number concatenation")
                })
                .collect();

            return Ok(ParsedResult::VecNums(races?));
        }
    }

    Err("something went wrong during parsing")
}

fn find_total_ways_to_win((race_time, race_distance): (u64, u64)) -> u64 {
    let mut total_ways = 0;

    for hold_time in 1..race_time {
        let distance = hold_time * (race_time - hold_time);
        if distance > race_distance {
            total_ways += 1;
        }
    }

    total_ways
}

pub fn process_part1(filename: &str) -> Option<u64> {
    if let Ok(ParsedResult::TupleVec(parsed)) = parse(filename, true) {
        let multiplied = parsed
            .iter()
            .map(|race| find_total_ways_to_win(*race))
            .fold(1, |accu, wins| accu * wins);

        return Some(multiplied);
    }

    None
}

pub fn process_part2(filename: &str) -> Option<u64> {
    if let Ok(ParsedResult::VecNums(parsed)) = parse(filename, false) {
        let multiplied = find_total_ways_to_win((parsed[0], parsed[1]));

        return Some(multiplied);
    }

    None
}
