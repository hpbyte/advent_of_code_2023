use std::env;

mod point;
mod soln;

use soln::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let part = args.get(1).expect("please provide the part no. [1/2]");

    let sample1 = String::from("./sample-1.input");
    let sample2 = String::from("./sample-2.input");

    let filename = match part.as_str() {
        "1" => args.get(2).unwrap_or(&sample1),
        "2" => args.get(2).unwrap_or(&sample2),
        _ => {
            eprintln!("Invalid part no. Please provide 1 or 2.");
            return;
        }
    };

    if let Some(ans) = match part.as_str() {
        "1" => process_part1(filename),
        "2" => process_part2(filename),
        _ => Some(0),
    } {
        println!(
            "The sum of all of the part numbers in the engine schematic: {}",
            ans
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_work() {
        let ans = process_part1("./sample-1.input").unwrap();
        assert_eq!(ans, 11159);
    }

    #[test]
    fn part_2_should_work() {
        let ans = process_part2("./sample-2.input").unwrap();
        assert_eq!(ans, 0);
    }
}
