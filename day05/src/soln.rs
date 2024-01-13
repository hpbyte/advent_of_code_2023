use std::fs::read_to_string;
use std::io::Error;

fn parse(block: &str) -> Vec<Vec<u64>> {
    block
        .split("\n")
        .skip(1)
        .map(|line| {
            line.trim()
                .split_whitespace()
                .filter_map(|v| v.parse().ok())
                .collect::<Vec<u64>>()
        })
        .filter(|v| !v.is_empty())
        .collect()
}

fn get_input_maps(filename: &str) -> Result<(Vec<u64>, Vec<Vec<Vec<u64>>>), Error> {
    let contents = read_to_string(filename)?;

    let mut seeds: Vec<u64> = Vec::new();
    let mut maps: Vec<Vec<Vec<u64>>> = Vec::with_capacity(7);

    contents
        .split("\n\n")
        .enumerate()
        .for_each(|(index, block)| {
            if index == 0 {
                seeds = block
                    .trim()
                    .split_whitespace()
                    .skip(1)
                    .filter_map(|v| v.parse().ok())
                    .collect::<Vec<u64>>();
            } else {
                maps.push(parse(block));
            }
        });

    Ok((seeds, maps))
}

fn walk(src: &u64, map: &Vec<Vec<u64>>) -> u64 {
    for range in map {
        if range.len() != 3 {
            panic!("range should have 3 numbers: dest, src and len");
        }
        let src_start = range[1];
        let src_end = range[1] + range[2] - 1;
        let dest_start = range[0];

        if *src >= src_start && *src <= src_end {
            return *src - src_start + dest_start;
        }
    }

    *src
}

pub fn process_part1(filename: &str) -> Option<u64> {
    let (seeds, maps) = match get_input_maps(filename) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("error reading input data: {}", e);
            return None;
        }
    };

    seeds
        .iter()
        .map(|&seed| maps.iter().fold(seed, |accu, map| walk(&accu, map)))
        .min()
}

fn get_seeds_from_range(range: Vec<u64>) -> Vec<u64> {
    range
        .chunks_exact(2)
        .flat_map(|chunk| {
            if chunk.len() != 2 {
                panic!("seed range format is invalid!");
            }

            chunk[0]..(chunk[0] + chunk[1])
        })
        .collect::<Vec<u64>>()
}

pub fn process_part2(filename: &str) -> Option<u64> {
    let (seeds_range, maps) = match get_input_maps(filename) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("error reading input data: {}", e);
            return None;
        }
    };

    get_seeds_from_range(seeds_range)
        .iter()
        .map(|&seed| maps.iter().fold(seed, |accu, map| walk(&accu, map)))
        .min()
}
