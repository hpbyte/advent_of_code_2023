use std::fs::read_to_string;

fn parse(block: &str) -> Vec<Vec<u32>> {
    block
        .split("\n")
        .skip(1)
        .filter_map(|line| {
            let nums = line
                .trim()
                .split_whitespace()
                .filter_map(|v| v.parse().ok())
                .collect::<Vec<u32>>();

            if nums.is_empty() {
                None
            } else {
                Some(nums)
            }
        })
        .collect()
}

fn get_input_maps(filename: &str) -> (Vec<u32>, Vec<Vec<Vec<u32>>>) {
    let contents = read_to_string(filename).expect("should read file");

    let mut seeds: Vec<u32> = Vec::new();
    let mut maps: Vec<Vec<Vec<u32>>> = Vec::with_capacity(7);

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
                    .collect::<Vec<u32>>();
            } else {
                maps.push(parse(block));
            }
        });

    (seeds, maps)
}

pub fn process_part1(filename: &str) -> Option<u32> {
    let (seeds, maps) = get_input_maps(filename);

    println!("{:?}", seeds);
    println!("{:?}", maps);

    None
}

pub fn process_part2(_filename: &str) -> Option<u32> {
    None
}
