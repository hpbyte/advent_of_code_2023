use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DIGITS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn extract_number(line: &str) -> u32 {
    let digits: Vec<u32> = line
        .as_bytes()
        .iter()
        .filter_map(|b| {
            if b.is_ascii_digit() {
                return Some(b.wrapping_sub(b'0') as u32);
            }

            None
        })
        .collect();

    let first = digits.first().unwrap();
    let last = digits.last().unwrap();

    first * 10 + last
}

/**
 * convert written form to digit
 * e.g. one -> 1, two -> 2
 */
fn convert_and_extract_number(line: &str) -> u32 {
    let mut line = line.as_bytes();

    let first = 'outer: loop {
        if line[0].is_ascii_digit() {
            break line[0].wrapping_sub(b'0') as u32;
        }

        for (index, digit) in DIGITS.iter().enumerate() {
            if line.starts_with(digit) {
                break 'outer (index + 1) as u32;
            }
        }

        line = &line[1..];
    };

    let last = 'outer: loop {
        if line[line.len() - 1].is_ascii_digit() {
            break line[line.len() - 1].wrapping_sub(b'0') as u32;
        }

        for (index, digit) in DIGITS.iter().enumerate() {
            if line.ends_with(digit) {
                break 'outer (index + 1) as u32;
            }
        }

        line = &line[..line.len() - 1];
    };

    first * 10 + last
}

pub fn process_part1(filename: &str) -> Option<u32> {
    if let Ok(lines) = read_lines(filename) {
        let sum: u32 = lines
            .filter_map(|line_result| match line_result {
                Ok(line) => Some(extract_number(&line)),
                Err(_) => None,
            })
            .sum();

        return Some(sum);
    }

    None
}

pub fn process_part2(filename: &str) -> Option<u32> {
    if let Ok(lines) = read_lines(filename) {
        let sum: u32 = lines
            .filter_map(|line_result| match line_result {
                Ok(line) => Some(convert_and_extract_number(&line)),
                Err(_) => None,
            })
            .sum();

        return Some(sum);
    }

    None
}
