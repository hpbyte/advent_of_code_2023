use std::collections::HashMap;
use std::fs::read_to_string;
use std::io;

#[derive(Debug)]
struct Map {
    instructions: String,
    nodes: HashMap<String, (String, String)>,
}

fn parse(filename: &str) -> Result<Map, io::Error> {
    let result = read_to_string(filename).unwrap();

    let (instructions, nodes_str) = result.split_once("\n\n").ok_or(io::Error::new(
        io::ErrorKind::InvalidData,
        "Invalid file format",
    ))?;

    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    nodes_str.lines().for_each(|line| {
        let (node, elements) = line.split_once(" = ").unwrap();

        let (left, right) = elements.split_once(", ").unwrap();

        nodes.insert(
            node.to_string(),
            (left.replace("(", ""), right.replace(")", "")),
        );
    });

    Ok(Map {
        instructions: instructions.to_string(),
        nodes,
    })
}

pub fn process_part1(filename: &str) -> Option<u32> {
    if let Ok(result) = parse(filename) {
        println!("{:?}", result);
        return Some(0);
    }

    None
}

pub fn process_part2(_filename: &str) -> Option<u32> {
    Some(0)
}
