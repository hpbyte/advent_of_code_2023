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

    let nodes_str_lines: Vec<_> = nodes_str.lines().collect();
    let mut nodes: HashMap<String, (String, String)> =
        HashMap::with_capacity(nodes_str_lines.len());

    for line in nodes_str_lines {
        nodes.insert(
            line[0..3].to_string(),
            (line[7..10].to_string(), line[12..15].to_string()),
        );
    }

    Ok(Map {
        instructions: instructions.to_string(),
        nodes,
    })
}

fn traverse(map: &Map, start: &str) -> u64 {
    let mut curr = start;
    let mut steps: u64 = 0;
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

        if curr.ends_with("Z") {
            break;
        }
    }

    steps
}

fn traverse_multi(map: &Map) -> u64 {
    map.nodes
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|node| traverse(map, node))
        .reduce(lcm)
        .unwrap_or(0)
}

fn gcd(a: u64, b: u64) -> u64 {
    let remainder = a % b;
    if remainder == 0 {
        return b;
    }
    return gcd(b, remainder);
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcd(a, b))
}

pub fn process_part1(filename: &str) -> Option<u64> {
    if let Ok(parsed) = parse(filename) {
        return Some(traverse(&parsed, "AAA"));
    }

    None
}

pub fn process_part2(filename: &str) -> Option<u64> {
    if let Ok(parsed) = parse(filename) {
        return Some(traverse_multi(&parsed));
    }

    None
}
