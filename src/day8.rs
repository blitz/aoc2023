pub(crate) use std::{collections::BTreeMap, fmt::Display, str::FromStr};

use anyhow::{anyhow, Result};
use regex::Regex;

const DAY8_INPUT: &str = include_str!("day8.input");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(anyhow!("Invalid direction: {s}")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Label {
    name: [char; 3],
}

const AAA: Label = Label {
    name: ['A', 'A', 'A'],
};

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name.iter().collect::<String>())
    }
}

impl Label {
    fn is_start_node(&self) -> bool {
        self.name[2] == 'A'
    }

    fn is_end_node(&self) -> bool {
        self.name[2] == 'Z'
    }
}

impl FromStr for Label {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Label {
            name: s
                .chars()
                .collect::<Vec<_>>()
                .try_into()
                .map_err(|_| anyhow!("Invalid length for label: {s}"))?,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Input {
    directions: Vec<Direction>,
    map: BTreeMap<Label, (Label, Label)>,
}

impl Input {
    fn next_label(&self, current: Label, direction: Direction) -> Result<Label> {
        let (left, right) = self
            .map
            .get(&current)
            .copied()
            .ok_or_else(|| anyhow!("Invalid label: {current}"))?;

        Ok(match direction {
            Direction::Left => left,
            Direction::Right => right,
        })
    }

    fn solve_one(&self, start: Label) -> Result<usize> {
        let mut location = start;

        for (steps, direction) in self.directions.iter().copied().cycle().enumerate() {
            if location.is_end_node() {
                return Ok(steps);
            }

            location = self.next_label(location, direction)?;
        }

        // The for loop never finishes.
        unreachable!();
    }

    fn solve_part1(&self) -> Result<usize> {
        self.solve_one(AAA)
    }

    fn solve_part2(&self) -> Result<usize> {
        let start_locations = self
            .map
            .keys()
            .copied()
            .filter(|l| l.is_start_node())
            .collect::<Vec<_>>();

        // We assume that all paths have the same cycle lengths. So we
        // just compute them individually and check when they line up.
        let solutions = start_locations
            .iter()
            .copied()
            .map(|l| self.solve_one(l))
            .collect::<Result<Vec<_>>>()?;

        let lcm = solutions.into_iter().fold(1, num_integer::lcm);

        Ok(lcm)
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let directions = lines
            .next()
            .unwrap()
            .chars()
            // TODO This could be more efficient.
            .map(|c| Direction::from_str(&c.to_string()))
            .collect::<Result<Vec<_>>>()?;

        let re = Regex::new(r"^(...) = \((...), (...)\)$")?;
        let mut map: BTreeMap<Label, (Label, Label)> = BTreeMap::default();

        for l in lines {
            if l.is_empty() {
                continue;
            }

            let captures = re
                .captures(l)
                .ok_or_else(|| anyhow!("Input line doesn't match: {l}"))?;

            let label_from = |group| Label::from_str(captures.get(group).unwrap().as_str());

            map.insert(label_from(1)?, (label_from(2)?, label_from(3)?));
        }

        Ok(Self { directions, map })
    }
}

pub fn solve() -> Result<()> {
    let input = Input::from_str(DAY8_INPUT)?;

    println!("🎁 Part 1 Solution: {}", input.solve_part1()?);
    println!("🎁 Part 2 Solution: {}", input.solve_part2()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() -> Result<()> {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

        let input = Input::from_str(input)?;

        assert_eq!(input.solve_part1()?, 6);

        Ok(())
    }

    #[test]
    fn example2_works() -> Result<()> {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let input = Input::from_str(input)?;

        assert_eq!(input.solve_part2()?, 6);

        Ok(())
    }
}
