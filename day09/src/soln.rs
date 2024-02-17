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

fn extrapolate(triangle: Vec<Vec<i64>>, last: bool) -> i64 {
    triangle
        .iter()
        .rev()
        .skip(1)
        .fold(0, |accu, row| match last {
            true => row.last().unwrap() + accu,
            false => row.first().unwrap() - accu,
        })
}

fn parse(line: &str) -> Vec<Vec<i64>> {
    let mut pascals_triangle: Vec<Vec<i64>> = Vec::new();

    let mut nums = line
        .split_whitespace()
        .filter_map(|n| n.parse::<i64>().ok())
        .collect::<Vec<i64>>();

    pascals_triangle.push(nums.clone());

    while !nums.iter().all(|&n| n == 0) {
        for index in 0..nums.len() - 1 {
            nums[index] = nums[index + 1] - nums[index];
        }

        nums.pop();

        pascals_triangle.push(nums.clone());
    }

    pascals_triangle
}

pub fn process_part1(filename: &str) -> Option<i64> {
    read_lines(filename)
        .ok()?
        .filter_map(|line| line.ok())
        .map(|line| extrapolate(parse(&line), true))
        .sum::<i64>()
        .into()
}

pub fn process_part2(filename: &str) -> Option<i64> {
    read_lines(filename)
        .ok()?
        .filter_map(|line| line.ok())
        .map(|line| extrapolate(parse(&line), false))
        .sum::<i64>()
        .into()
}
