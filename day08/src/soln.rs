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

fn traverse_bfs(map: &Map, start: &str) -> u64 {
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

// greatest common divisor
fn gcd(a: u64, b: u64) -> u64 {
    let remainder = a % b;
    if remainder == 0 {
        return b;
    }
    return gcd(b, remainder);
}

// least common multiple
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

pub fn process_part1(filename: &str) -> Option<u64> {
    if let Ok(parsed) = parse(filename) {
        return Some(traverse_bfs(&parsed, "AAA"));
    }

    None
}

/**
 * PART2:
 * we start from all the nodes that ends with 'A'
 * and simultaneously traverse until all the paths reach to nodes ends with 'Z'
 * the tricky part here is that in order for all the paths to finish at the same time,
 * some of them has to be repeated several times until other paths can catch up.
 * and these paths are cyclical.
 * https://www.reddit.com/r/adventofcode/comments/18did3d/2023_day_8_part_1_my_input_maze_plotted_using/
 *
 *
 * here, we don't actually repeat but compute the least common multiple of all the path lengths
 * and this gives us the one path length
 */
pub fn process_part2(filename: &str) -> Option<u64> {
    if let Ok(map) = parse(filename) {
        let steps = map
            .nodes
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|node| traverse_bfs(&map, node))
            .reduce(lcm)
            .unwrap_or(0);

        return Some(steps);
    }

    None
}
