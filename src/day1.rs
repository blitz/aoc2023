use anyhow::{anyhow, Result};
use colored::Colorize;
use itertools::Itertools;

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

const DIGIT_NAMES: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

/// Replace the first character of digit names with the digit itself.
///
/// For example: 1two3 -> 12wo3
fn replace_digit_names(line: &str) -> String {
    let mut result = line.chars().collect_vec();

    for (digit, positions) in DIGIT_NAMES
        .iter()
        .enumerate()
        .map(|(digit, name)| (digit, line.match_indices(name).map(|m| m.0).collect_vec()))
    {
        let d = digit.to_string().chars().collect_vec()[0];

        for pos in positions {
            result[pos] = d;
        }
    }

    result.into_iter().collect()
}

fn line_calibration_value_with_strings(line: &str) -> Option<u32> {
    let digit_str = replace_digit_names(line);

    line_calibration_value(&digit_str)
}

fn sum_of_calibrations_with_strings(input: &str) -> Result<u32> {
    let opt_sum: Option<u32> = input.lines().map(line_calibration_value_with_strings).sum();
    opt_sum.ok_or_else(|| anyhow!("Failed to parse some lines?"))
}

pub fn solve() -> Result<()> {
    println!(
        "🎁 Part 1 Solution: {}",
        sum_of_calibrations(DAY1_INPUT)?.to_string().bold()
    );

    println!(
        "🎁 Part 2 Solution: {}",
        sum_of_calibrations_with_strings(DAY1_INPUT)?
            .to_string()
            .bold()
    );

    Ok(())
}

#[cfg(test)]
mod testests {
    use super::*;

    #[test]
    fn replace_digit_names_works() {
        assert_eq!(replace_digit_names("1234"), "1234".to_owned());
        assert_eq!(replace_digit_names("1one34"), "11ne34".to_owned());
        assert_eq!(
            replace_digit_names("1eightwotwo4"),
            "18igh2wo2wo4".to_owned()
        );
        assert_eq!(
            replace_digit_names("seven91eightwo"),
            "7even918igh2wo".to_owned()
        );
    }

    #[test]
    fn handling_line_works() {
        assert_eq!(line_calibration_value("a1b2c"), Some(12));
        assert_eq!(
            line_calibration_value_with_strings("seven91eightwo"),
            Some(72)
        );
    }

    #[test]
    fn example_part1() {
        let example_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        assert_eq!(sum_of_calibrations(example_input).unwrap(), 142);
    }

    #[test]
    fn example_part2() {
        let example_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        assert_eq!(
            sum_of_calibrations_with_strings(example_input).unwrap(),
            281
        );
    }
}
