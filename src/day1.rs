use anyhow::{anyhow, Result};
use colored::Colorize;

const DAY1_INPUT: &str = std::include_str!("day1.input");

fn line_calibration_value(line: &str) -> Option<u32> {
    if let Some(first_digit) = line.chars().find_map(|c| c.to_digit(10)) {
        if let Some(last_digit) = line.chars().rev().find_map(|c| c.to_digit(10)) {
            return Some(first_digit * 10 + last_digit);
        }
    }

    None
}

fn sum_of_calibrations(input: &str) -> Result<u32> {
    let opt_sum: Option<u32> = input.lines().map(line_calibration_value).sum();
    opt_sum.ok_or_else(|| anyhow!("Failed to parse some lines?"))
}

pub fn solve() -> Result<()> {
    println!(
        "🎁 Solution: {}",
        sum_of_calibrations(DAY1_INPUT)?.to_string().bold()
    );
    Ok(())
}

#[cfg(test)]
mod testests {
    use super::*;

    #[test]
    fn handling_line_works() {
        assert_eq!(line_calibration_value("a1b2c"), Some(12));
    }

    #[test]
    fn example() {
        let example_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(sum_of_calibrations(example_input).unwrap(), 142);
    }
}
