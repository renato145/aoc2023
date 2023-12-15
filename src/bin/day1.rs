//! Day 1: Trebuchet?!

use std::fs;

fn get_calibration(input: &str) -> u32 {
    let first_digit = input.chars().find_map(|c| c.to_digit(10)).unwrap();
    let last_digit = input.chars().rev().find_map(|c| c.to_digit(10)).unwrap();
    first_digit * 10 + last_digit
}

fn solve(input: &str) -> u32 {
    input.lines().map(get_calibration).sum::<u32>()
}

fn main() {
    let problem = fs::read_to_string("src/bin/day1.txt").expect("Failed to read file.");
    println!("{}", solve(&problem));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let example = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(solve(example), 142);
    }
}
