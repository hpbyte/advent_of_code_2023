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

fn extract_numbers(line: &str) -> Vec<u32> {
    line.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn extract_first_and_last_number(numbers: Vec<u32>) -> Option<(u32, u32)> {
    if let Some(first) = numbers.first().cloned() {
        let last = *numbers.last().unwrap_or(&0);
        Some((first, last))
    } else {
        None
    }
}

fn text_to_number(line: &str) -> String {
    let mappings: Vec<(&str, &str)> = vec![
        ("zero", "0"),
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut result = line.to_string();
    let mut temp_result = String::new();

    for c in result.chars() {
        temp_result.push(c);
        if let Some((word, digit)) = mappings
            .iter()
            .find(|(word, _)| temp_result.ends_with(word))
        {
            temp_result = temp_result[..temp_result.len() - word.len()].to_string() + digit;
        }
    }

    result = temp_result;

    result
}

pub fn process_part1(filename: &str) -> Option<u32> {
    if let Ok(lines) = read_lines(filename) {
        let mut sum = 0;
        for line in lines {
            if let Ok(str) = line {
                if let Some((first, last)) = extract_first_and_last_number(extract_numbers(&str)) {
                    sum = sum + (first * 10) + last;
                }
            }
        }

        return Some(sum);
    }

    None
}

pub fn process_part2(filename: &str) -> Option<u32> {
    if let Ok(lines) = read_lines(filename) {
        let mut sum = 0;
        for line in lines {
            if let Ok(str) = line {
                if let Some((first, last)) =
                    extract_first_and_last_number(extract_numbers(text_to_number(&str).as_str()))
                {
                    sum = sum + (first * 10) + last;
                }
            }
        }

        return Some(sum);
    }

    None
}
