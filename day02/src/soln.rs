use std::collections::HashMap;
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

fn parse_color_and_count(cube_set: &str) -> Result<(&str, i32), String> {
    let parts: Vec<&str> = cube_set.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("the cube set format is wrong!".to_string());
    }

    let count = parts[0];
    let color = parts[1];

    match count.parse::<i32>() {
        Ok(count) => Ok((color, count)),
        Err(_) => Err("failed to parse number".to_string()),
    }
}

fn parse_game_id(game: &str) -> Result<u32, u32> {
    let parts: Vec<&str> = game.split_whitespace().collect();
    if parts.len() != 2 {
        return Err(0);
    }

    let game = parts[1];

    match game.parse::<u32>() {
        Ok(game) => Ok(game),
        Err(_) => Err(0),
    }
}

fn find_possible_game(line: &str) -> u32 {
    let splitted = line.split(":").collect::<Vec<&str>>();

    for set in splitted[1].split(";") {
        let mut cube_counts: HashMap<&str, i32> =
            HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

        for cube_set in set.split(",") {
            if let Ok((color, count)) = parse_color_and_count(cube_set) {
                let current_count = cube_counts.entry(color).or_insert(0);
                *current_count -= count;

                if *current_count < 0 {
                    return 0;
                }
            }
        }
    }

    parse_game_id(splitted[0]).unwrap_or(0)
}

pub fn process_part1(filename: &str) -> Option<u32> {
    if let Ok(lines) = read_lines(filename) {
        let sum: u32 = lines
            .filter_map(|line_result| match line_result {
                Ok(line) => Some(find_possible_game(&line)),
                Err(_) => None,
            })
            .sum();

        return Some(sum);
    }

    None
}

pub fn process_part2(filename: &str) -> Option<u32> {
    Some(0)
}
