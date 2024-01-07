use std::env;

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

    match part.as_str() {
        "1" => {
            if let Some(ans) = process_part1(filename) {
                println!("The total points worth: {}", ans);
            }
        }
        "2" => {
            if let Some(ans) = process_part2(filename) {
                println!("The total number of scratchcards: {}", ans);
            }
        }
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_should_work() {
        let ans = process_part1("./sample-1.input").unwrap();
        assert_eq!(ans, 13);
    }

    #[test]
    fn part_2_should_work() {
        let ans = process_part2("./sample-2.input").unwrap();
        assert_eq!(ans, 30);
    }
}
