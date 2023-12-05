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

fn extract_first_and_last_number(line: &str) -> Option<(u32, u32)> {
    let numbers: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();

    if let Some(first) = numbers.first().cloned() {
        let last = *numbers.last().unwrap_or(&0);
        Some((first, last))
    } else {
        None
    }
}

pub fn process_part1(filename: &str) -> Option<u32> {
    if let Ok(lines) = read_lines(filename) {
        let mut sum = 0;
        for line in lines {
            if let Ok(str) = line {
                if let Some((first, last)) = extract_first_and_last_number(&str) {
                    sum = sum + (first * 10) + last;
                }
            }
        }

        return Some(sum);
    }

    None
}
