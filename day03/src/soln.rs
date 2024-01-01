use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::point::*;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn construct_grid(filename: &str) -> Result<Vec<Vec<char>>, String> {
    if let Ok(lines) = read_lines(filename) {
        let grid = lines
            .filter_map(|line_result| line_result.ok())
            .map(|line| line.chars().collect())
            .collect::<Vec<Vec<char>>>();

        Ok(grid)
    } else {
        Err("error reading file".to_string())
    }
}

fn check_is_part_number(
    origin: &Point,
    current_num: &mut i32,
    is_part_number: &mut bool,
    grid: &Vec<Vec<char>>,
) {
    let rows = grid.len();
    let cols = grid.first().unwrap().len();

    for point in DIAGONAL.iter().copied().map(|d| *origin + d) {
        let x = point.x as usize;
        let y = point.y as usize;

        // ensure the indices are in bound
        if y < rows && x < cols {
            let adjacent = grid[y][x];

            // check the right side of the origin to see if the number will be continuous
            if point == RIGHT.clone() + *origin && adjacent.is_digit(10) {
                *current_num *= 10;

                // can skip the symbol check as it's a number
                continue;
            }

            // is the adjacent a symbol
            if adjacent != '.' && !adjacent.is_digit(10) {
                *is_part_number = true;
            }
        }
    }
}

pub fn process_part1(filename: &str) -> Option<i32> {
    if let Ok(grid) = construct_grid(filename) {
        let mut sum: i32 = 0;
        let mut num = 0;
        let mut is_part_number = false;

        for (y, row) in grid.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if !col.is_digit(10) {
                    if is_part_number {
                        sum += num;
                        is_part_number = false;
                    }

                    num = 0;

                    continue;
                }

                num += col.to_digit(10).unwrap_or(0) as i32;

                let origin = Point::new(x as i32, y as i32);

                check_is_part_number(&origin, &mut num, &mut is_part_number, &grid);
            }
        }

        return Some(sum);
    }

    None
}

pub fn process_part2(_filename: &str) -> Option<i32> {
    Some(0)
}
