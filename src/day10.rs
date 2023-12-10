use std::{collections::BTreeSet, ops::Range, str::FromStr};

use anyhow::Result;
use itertools::Itertools;

const DAY10_INPUT: &str = include_str!("day10.input");

#[derive(Debug, Clone)]
struct Input {
    data: Vec<char>,
    columns: usize,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = vec![];
        let mut opt_columns = None;

        for l in s.lines() {
            let chars = l.chars().collect::<Vec<_>>();

            if let Some(cols) = opt_columns {
                assert_eq!(cols, chars.len());
            } else {
                opt_columns = Some(chars.len());
            }

            data.extend_from_slice(&chars);
        }

        Ok(Self {
            data,
            columns: opt_columns.unwrap(),
        })
    }
}

impl Input {
    // Out of bounds accesses return 'ground': .
    fn get(&self, col: isize, row: isize) -> char {
        if col < 0 || row < 0 {
            '.'
        } else {
            self.data
                .get((col as usize) + (row as usize) * self.columns)
                .copied()
                .unwrap_or('.')
        }
    }

    fn rows(&self) -> Range<isize> {
        0..((self.data.len() / self.columns) as isize)
    }

    fn cols(&self) -> Range<isize> {
        0..(self.columns as isize)
    }

    fn successors(&self, col: isize, row: isize) -> Option<((isize, isize), (isize, isize))> {
        match self.get(col, row) {
            '|' => Some(((col, row - 1), (col, row + 1))),
            '-' => Some(((col - 1, row), (col + 1, row))),
            'L' => Some(((col, row - 1), (col + 1, row))),
            'J' => Some(((col, row - 1), (col - 1, row))),
            '7' => Some(((col - 1, row), (col, row + 1))),
            'F' => Some(((col, row + 1), (col + 1, row))),
            'S' => None,
            '.' => None,
            tile => panic!("invalid tile: {tile}"),
        }
    }

    fn start_point(&self) -> (isize, isize) {
        let position = self.data.iter().position(|c| *c == 'S').unwrap();

        (
            (position % self.columns) as isize,
            (position / self.columns) as isize,
        )
    }

    fn steps_to_point(
        &self,
        initial_point: (isize, isize),
        destination: (isize, isize),
    ) -> Option<usize> {
        let mut set: BTreeSet<(isize, isize)> = BTreeSet::default();
        set.insert(initial_point);

        let mut steps = 0;

        let mut seen: BTreeSet<(isize, isize)> = set.clone();

        loop {
            if set.contains(&destination) {
                return Some(steps);
            }

            let next_set = BTreeSet::from_iter(
                set.iter()
                    .filter_map(|p| self.successors(p.0, p.1))
                    .flat_map(|(p1, p2)| [p1, p2])
                    .filter(|p| !seen.contains(p)),
            );

            seen.extend(next_set.iter());

            if next_set == set {
                break;
            }

            set = next_set;
            steps += 1;
        }

        None
    }

    fn solve_part1(&self) -> usize {
        let start = self.start_point();

        // TODO This is extremely inefficient. It's better to only
        // start with the tiles around the start point.
        let (_c, steps) = self
            .cols()
            .cartesian_product(self.rows())
            .filter_map(|c| self.steps_to_point(c, start).map(|steps| (c, steps)))
            .max_by_key(|(_c, steps)| *steps)
            .unwrap();

        steps
    }
}

pub fn solve() -> Result<()> {
    let input = Input::from_str(DAY10_INPUT)?;

    println!("🎁 Part 1 Solution: {}", input.solve_part1());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() -> Result<()> {
        let input = Input::from_str(
            "abc
deS",
        )?;

        assert_eq!(input.get(0, 0), 'a');
        assert_eq!(input.get(1, 0), 'b');
        assert_eq!(input.get(0, 1), 'd');

        assert_eq!(input.start_point(), (2, 1));

        assert_eq!(input.rows(), (0..2));
        assert_eq!(input.cols(), (0..3));

        Ok(())
    }

    #[test]
    fn example_works() -> Result<()> {
        Ok(())
    }
}
