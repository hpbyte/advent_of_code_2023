use std::env;

mod soln;

use soln::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    if let Some(ans) = process_part1(filename) {
        println!("The sum of all of the calibration values: {}", ans);
    }

    if let Some(ans) = process_part2(filename) {
        println!("The sum of all of the calibration values: {}", ans);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_work() {
        let ans = process_part1("./sample-1.input").unwrap();
        assert_eq!(ans, 142);
    }

    #[test]
    fn part_2_should_work() {
        let ans = process_part2("./sample-2.input").unwrap();
        assert_eq!(ans, 281);
    }
}
