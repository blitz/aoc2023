use std::{collections::BTreeSet, str::FromStr};

use anyhow::{anyhow, Result};
use array2d::Array2D;
use itertools::Itertools;

const DAY11_INPUT: &str = include_str!("day11.input");

#[derive(Debug, PartialEq, Eq, Clone)]
struct Input {
    array: Array2D<bool>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<bool>>>();

        Ok(Self {
            array: Array2D::from_rows(&lines).map_err(|_| anyhow!("Failed to create array"))?,
        })
    }
}

fn manhattan_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

impl Input {
    #[must_use]
    fn empty_rows(&self) -> BTreeSet<usize> {
        self.array
            .as_rows()
            .into_iter()
            .enumerate()
            .filter_map(|(i, row)| row.iter().all(|c| !c).then_some(i))
            .collect::<BTreeSet<_>>()
    }

    #[must_use]
    fn empty_cols(&self) -> BTreeSet<usize> {
        self.array
            .as_columns()
            .into_iter()
            .enumerate()
            .filter_map(|(i, col)| col.iter().all(|c| !c).then_some(i))
            .collect::<BTreeSet<_>>()
    }

    #[must_use]
    fn expand_rows(&self) -> Input {
        let empty_rows = self.empty_rows();

        let expanded_rows = self
            .array
            .as_rows()
            .into_iter()
            .enumerate()
            .flat_map(|(i, row)| {
                if empty_rows.contains(&i) {
                    vec![row.clone(), row]
                } else {
                    vec![row]
                }
            })
            .collect::<Vec<_>>();

        Self {
            array: Array2D::from_rows(&expanded_rows).unwrap(),
        }
    }

    #[must_use]
    fn expand_cols(&self) -> Input {
        // TODO This could rotate the array and re-use expand_rows.

        let empty_cols = self.empty_cols();

        let expanded_cols = self
            .array
            .as_columns()
            .into_iter()
            .enumerate()
            .flat_map(|(i, col)| {
                if empty_cols.contains(&i) {
                    vec![col.clone(), col]
                } else {
                    vec![col]
                }
            })
            .collect::<Vec<_>>();

        Self {
            array: Array2D::from_columns(&expanded_cols).unwrap(),
        }
    }

    #[must_use]
    fn expand(&self) -> Input {
        self.expand_rows().expand_cols()
    }

    #[must_use]
    fn galaxies(&self) -> Vec<(usize, usize)> {
        self.array
            .enumerate_row_major()
            .filter_map(|(c, b)| (*b).then_some(c))
            .collect()
    }

    fn expand_coords(&self, coords: &[(usize, usize)], factor: usize) -> Vec<(usize, usize)> {
        let empty_rows = self.empty_rows();
        let empty_cols = self.empty_cols();

        coords
            .iter()
            .copied()
            .map(|c| {
                let expand_rows = empty_rows
                    .iter()
                    .copied()
                    .filter(|empty_row| *empty_row < c.0)
                    .count();
                let expand_cols = empty_cols
                    .iter()
                    .copied()
                    .filter(|empty_col| *empty_col < c.1)
                    .count();

                (c.0 + expand_rows * factor, c.1 + expand_cols * factor)
            })
            .collect()
    }
}

pub fn solve() -> Result<()> {
    let input = Input::from_str(DAY11_INPUT)?;

    println!(
        "🎁 Part 1 Solution: {}",
        input
            .expand()
            .galaxies()
            .into_iter()
            .tuple_combinations()
            .map(|(c1, c2)| manhattan_distance(c1, c2))
            .sum::<usize>()
    );

    println!(
        "🎁 Part 2 Solution: {}",
        input
            .expand_coords(&input.galaxies(), 1000000 - 1)
            .into_iter()
            .tuple_combinations()
            .map(|(c1, c2)| manhattan_distance(c1, c2))
            .sum::<usize>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() -> Result<()> {
        let input = ".#.#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        let input = Input::from_str(input)?;

        eprintln!("{:?}", &input.array);
        assert!(*input.array.get(2, 0).unwrap());

        Ok(())
    }

    #[test]
    fn distance_works() {
        assert_eq!(manhattan_distance((0, 10), (0, 2)), 8);
        assert_eq!(manhattan_distance((0, 0), (1, 1)), 2);
    }
}
