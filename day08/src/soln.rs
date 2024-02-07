use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Map {
    instructions: String,
    nodes: HashMap<String, (String, String)>,
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse(filename: &str) -> Map {
    let mut nodes = HashMap::new();
    nodes.insert(
        String::from("AAA"),
        (String::from("AAA"), String::from("AAA")),
    );

    Map {
        instructions: String::from("LLR"),
        nodes,
    }
}
pub fn process_part1(filename: &str) -> Option<u32> {
    Some(0)
}

pub fn process_part2(_filename: &str) -> Option<u32> {
    Some(0)
}
