// To allow for lpos/cpos names.
#![warn(clippy::similar_names)]

use std::{collections::BTreeSet, str::FromStr};

use anyhow::{bail, Result};
use colored::Colorize;
use itertools::Itertools;

const DAY3_INPUT: &str = std::include_str!("day3.input");

struct Array {
    lines: Vec<Vec<char>>,
}

impl FromStr for Array {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().map(|l| l.chars().collect_vec()).collect_vec();

        if lines.is_empty() || !lines.iter().all(|l| l.len() == lines[0].len()) {
            bail!("Malformed input")
        }

        Ok(Array { lines })
    }
}

fn is_symbol(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

fn adjacent_coords(lpos: usize, cpos: usize) -> Vec<(usize, usize)> {
    vec![
        (lpos, cpos.wrapping_sub(1)),
        (lpos.wrapping_sub(1), cpos.wrapping_sub(1)),
        (lpos.wrapping_sub(1), cpos),
        (lpos.wrapping_sub(1), cpos + 1),
        (lpos, cpos + 1),
        (lpos + 1, cpos + 1),
        (lpos + 1, cpos),
        (lpos + 1, cpos.wrapping_sub(1)),
    ]
}

impl Array {
    /// Returns the char at the given posistion. Returns '.' if out of bounds.
    fn get(&self, line: usize, pos: usize) -> char {
        self.lines
            .get(line)
            .and_then(|l| l.get(pos))
            .copied()
            .unwrap_or('.')
    }

    fn has_adjacent_symbol(&self, lpos: usize, cpos: usize) -> bool {
        adjacent_coords(lpos, cpos)
            .into_iter()
            .any(|(lpos, cpos)| is_symbol(self.get(lpos, cpos)))
    }

    fn adjacent_gears(&self, lpos: usize, cpos: usize) -> BTreeSet<(usize, usize)> {
        adjacent_coords(lpos, cpos)
            .into_iter()
            .filter(|(lpos, cpos)| self.get(*lpos, *cpos) == '*')
            .collect()
    }

    fn find_part_numbers(&self) -> Vec<(u32, BTreeSet<(usize, usize)>)> {
        enum NumberState {
            NoNumber,
            ValidNumber {
                number: u32,
                symbols: usize,
                gears: BTreeSet<(usize, usize)>,
            },
        }

        let mut result = vec![];

        for (lpos, line) in self.lines.iter().enumerate() {
            let mut state = NumberState::NoNumber;

            for (cpos, c) in line.iter().enumerate() {
                let is_digit = c.is_ascii_digit();
                let adjacent_symbol = self.has_adjacent_symbol(lpos, cpos);
                let adjacent_gears = self.adjacent_gears(lpos, cpos);

                state = match state {
                    NumberState::NoNumber => {
                        if is_digit {
                            NumberState::ValidNumber {
                                number: c.to_digit(10).unwrap(),
                                symbols: usize::from(adjacent_symbol),
                                gears: adjacent_gears,
                            }
                        } else {
                            NumberState::NoNumber
                        }
                    }
                    NumberState::ValidNumber {
                        number,
                        symbols,
                        gears,
                    } => {
                        if is_digit {
                            NumberState::ValidNumber {
                                number: number * 10 + c.to_digit(10).unwrap(),
                                symbols: symbols + usize::from(adjacent_symbol),
                                // Non-destructive set merge.
                                gears: gears.into_iter().chain(adjacent_gears).collect(),
                            }
                        } else {
                            if symbols != 0 {
                                result.push((number, gears));
                            }
                            NumberState::NoNumber
                        }
                    }
                };
            }

            if let NumberState::ValidNumber {
                number,
                symbols,
                gears,
            } = state
            {
                if symbols != 0 {
                    result.push((number, gears));
                }
            }
        }

        result
    }

    fn find_gears(&self) -> Vec<(u32, u32)> {
        self.find_part_numbers()
            .into_iter()
            .filter(|(_n, g)| !g.is_empty())
            .tuple_combinations()
            .filter_map(|((n1, g1), (n2, g2))| (!g1.is_disjoint(&g2)).then_some((n1, n2)))
            .collect()
    }
}

pub fn solve() -> Result<()> {
    let array = Array::from_str(DAY3_INPUT)?;

    println!(
        "🎁 Part 1 Solution: {}",
        array
            .find_part_numbers()
            .into_iter()
            .map(|t| t.0)
            .sum::<u32>()
            .to_string()
            .bold()
    );

    println!(
        "🎁 Part 2 Solution: {}",
        array
            .find_gears()
            .into_iter()
            .map(|(n1, n2)| n1 * n2)
            .sum::<u32>()
            .to_string()
            .bold()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn example_works() -> Result<()> {
        let array = Array::from_str(EXAMPLE)?;

        assert_eq!(
            array.find_part_numbers().iter().map(|t| t.0).collect_vec(),
            [467, 35, 633, 617, 592, 755, 664, 598]
        );

        assert_eq!(array.find_gears(), [(467, 35), (755, 598)]);

        Ok(())
    }
}
