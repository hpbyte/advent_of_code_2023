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

fn traverse(map: Map) -> u32 {
    let goal = "ZZZ";
    let mut curr = "AAA";
    let mut steps: u32 = 0;
    let mut i: usize = 0;
    let instructions = map.instructions.as_bytes();

    loop {
        // repeat the instrction from the start
        if i >= instructions.len() {
            i = 0;
        }

        curr = match instructions[i] as char {
            'L' => map.nodes.get(curr).unwrap().0.as_str(),
            'R' => map.nodes.get(curr).unwrap().1.as_str(),
            _ => panic!("invalid instruction!"),
        };

        i += 1;
        steps += 1;

        if curr == goal {
            break;
        }
    }

    steps
}

pub fn process_part1(filename: &str) -> Option<u32> {
    if let Ok(parsed) = parse(filename) {
        return Some(traverse(parsed));
    }

    None
}

pub fn process_part2(_filename: &str) -> Option<u32> {
    Some(0)
}
