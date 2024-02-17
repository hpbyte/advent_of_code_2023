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

fn extrapolate(triangle: Vec<Vec<i64>>) -> i64 {
    triangle
        .iter()
        .rev()
        .skip(1)
        .fold(0, |accu, row| row.last().unwrap() + accu)
}

fn parse(line: &str) -> Vec<Vec<i64>> {
    let mut pascals_triangle: Vec<Vec<i64>> = Vec::new();

    let mut nums = line
        .split_whitespace()
        .filter_map(|n| n.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    pascals_triangle.push(nums.clone());

    loop {
        for index in 0..nums.len() - 1 {
            nums[index] = nums[index + 1] - nums[index];
        }

        nums.pop();

        pascals_triangle.push(nums.clone());

        if nums.iter().sum::<i64>() == 0 {
            break;
        }
    }

    pascals_triangle
}

pub fn process_part1(filename: &str) -> Option<i64> {
    if let Ok(lines) = read_lines(filename) {
        let total = lines
            .filter_map(|line| line.ok())
            .map(|line| extrapolate(parse(&line)))
            .sum();

        return Some(total);
    }

    None
}

pub fn process_part2(_filename: &str) -> Option<i64> {
    Some(0)
}
