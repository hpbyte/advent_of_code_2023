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

pub fn process_part1(filename: &str) -> Option<u64> {
    if let Ok(_lines) = read_lines(filename) {
        return Some(0);
    }

    None
}

pub fn process_part2(_filename: &str) -> Option<u64> {
    Some(0)
}
