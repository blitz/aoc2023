use std::str::FromStr;

use anyhow::{anyhow, Result};
use array2d::Array2D;
use itertools::Itertools;

const DAY13_INPUT: &str = include_str!("day13.input");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Field {
    Ash,
    Rock,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Maze {
    array: Array2D<Field>,
}

fn mirror_point(list: &[Vec<Field>]) -> Option<usize> {
    assert!(!list.is_empty());

    for i in 1..list.len() {
        let (before, after) = list.split_at(i);
        let len = std::cmp::min(before.len(), after.len());

        if &before.iter().rev().take(len).collect::<Vec<_>>()
            == &after[0..len].iter().collect::<Vec<_>>()
        {
            return Some(i);
        }
    }

    None
}

impl Maze {
    fn mirror_row(&self) -> Option<usize> {
        mirror_point(&self.array.as_rows())
    }

    fn mirror_col(&self) -> Option<usize> {
        mirror_point(&self.array.as_columns())
    }

    fn mirror_score(&self) -> usize {
        self.mirror_row()
            .map(|v| v * 100)
            .or_else(|| self.mirror_col())
            .unwrap_or(0)
    }
}

impl FromStr for Maze {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Maze {
            array: Array2D::from_rows(
                &s.lines()
                    .map(|l| {
                        l.chars()
                            .map(|c| match c {
                                '.' => Ok(Field::Ash),
                                '#' => Ok(Field::Rock),
                                c => Err(anyhow!("Invalid input character {c}")),
                            })
                            .collect::<Result<Vec<Field>>>()
                    })
                    .collect::<Result<Vec<Vec<Field>>>>()?,
            )
            .map_err(|_| anyhow!("Failed to build array"))?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Input {
    mazes: Vec<Maze>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input_groups = s.lines().group_by(|l| !l.is_empty());

        Ok(Input {
            mazes: input_groups
                .into_iter()
                .filter_map(|(not_empty, lines)| {
                    not_empty
                        .then(|| Maze::from_str(&lines.collect::<Vec<_>>().join("\n")).unwrap())
                })
                .collect::<Vec<Maze>>(),
        })
    }
}

pub fn solve() -> Result<()> {
    let input = Input::from_str(DAY13_INPUT)?;

    println!(
        "🎁 Part 1 Solution: {}",
        input.mazes.iter().map(|l| l.mirror_score()).sum::<usize>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() -> Result<()> {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
        let input = Input::from_str(input)?;

        assert_eq!(
            input.mazes.iter().map(|l| l.mirror_score()).sum::<usize>(),
            405
        );

        Ok(())
    }
}
