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

struct Game(u32, u32, u32);

fn parse_count(count: &str) -> u32 {
    count.parse::<u32>().unwrap_or(0)
}

fn clean(input: &str) -> String {
    input.replace(",", "").replace(";", "")
}

fn parse(line: &str) -> Game {
    line.split(":")
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<Vec<&str>>()
        .chunks_exact(2)
        .fold(Game(0, 0, 0), |Game(r, g, b), chunk| {
            let count = parse_count(chunk[0]);
            let color = clean(chunk[1]);

            match color.as_str() {
                "red" => Game(r.max(count), g, b),
                "green" => Game(r, g.max(count), b),
                "blue" => Game(r, g, b.max(count)),
                _ => unreachable!(),
            }
        })
}

pub fn process_part1(filename: &str) -> Option<u32> {
    let res: u32 = read_lines(filename)
        .ok()?
        .enumerate()
        .filter_map(|(game_index, line_result)| {
            line_result.ok().and_then(|line| {
                let Game(r, g, b) = parse(&line);

                (r <= 12 && g <= 13 && b <= 14).then_some((game_index + 1) as u32)
            })
        })
        .sum();

    Some(res)
}

pub fn process_part2(filename: &str) -> Option<u32> {
    let res: u32 = read_lines(filename)
        .ok()?
        .filter_map(|line_result| {
            line_result.ok().and_then(|line| {
                let Game(r, g, b) = parse(&line);
                Some((r * g * b) as u32)
            })
        })
        .sum();

    Some(res)
}
