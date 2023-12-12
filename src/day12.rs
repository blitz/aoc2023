use std::{iter::repeat, str::FromStr};

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use rayon::prelude::*;

const DAY12_INPUT: &str = include_str!("day12.input");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum SpringState {
    Operational,
    Broken,
}

#[must_use]
fn broken_groups(states: &[SpringState]) -> Vec<usize> {
    states
        .iter()
        .copied()
        .group_by(|state| *state == SpringState::Broken)
        .into_iter()
        .filter_map(|(is_broken, group)| is_broken.then_some(group.count()))
        .collect()
}

#[must_use]
fn missing_broken(a: &[usize], b: &[usize]) -> usize {
    a.iter().sum::<usize>().abs_diff(b.iter().sum::<usize>())
}

struct SpringStateIterator<'a, 'b> {
    input: &'a [Option<SpringState>],
    broken_groups: &'b [usize],

    backtrack_stack: Vec<Vec<SpringState>>,
}

impl<'a, 'b> SpringStateIterator<'a, 'b> {
    fn new(input: &'a [Option<SpringState>], broken_groups: &'b [usize]) -> Self {
        Self {
            input,
            broken_groups,
            backtrack_stack: vec![vec![]],
        }
    }
}

impl<'a, 'b> SpringStateIterator<'a, 'b> {
    fn maybe_backtrack(&mut self, backtrack_candidate: Vec<SpringState>) {
        let mut candidate_groups = broken_groups(&backtrack_candidate);

        // Full input, everything must match!
        if backtrack_candidate.len() == self.input.len() {
            if candidate_groups == self.broken_groups {
                self.backtrack_stack.push(backtrack_candidate);
            }

            return;
        }

        let last_is_operational = backtrack_candidate
            .last()
            .map(|c| *c == SpringState::Operational)
            .unwrap_or(false);

        // More groups than the target?
        if candidate_groups.len() > self.broken_groups.len() {
            return;
        }

        // Can we fit all remaining groups?
        let remaining_items = self.input.len() - backtrack_candidate.len();
        let missing_broken = missing_broken(&candidate_groups, self.broken_groups);
        let missing_groups = self.broken_groups.len() - candidate_groups.len();
        let missing_operational = if missing_groups != 0 {
            // Each group needs to be terminated by one operational tile except the last.
            missing_groups - 1
        } else {
            0
        };

        if missing_broken + missing_operational > remaining_items {
            return;
        }

        // The last group might not be complete.
        if !last_is_operational && !candidate_groups.is_empty() {
            candidate_groups.pop();
        }

        // Everything we found so far has to match.
        if candidate_groups != self.broken_groups[0..candidate_groups.len()] {
            return;
        }

        self.backtrack_stack.push(backtrack_candidate);
    }
}

impl<'a, 'b> Iterator for SpringStateIterator<'a, 'b> {
    type Item = Vec<SpringState>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let mut cur = self.backtrack_stack.pop()?;

            if cur.len() == self.input.len() {
                return Some(cur);
            } else {
                match self.input[cur.len()] {
                    Some(s) => {
                        cur.push(s);
                        self.maybe_backtrack(cur);
                    }
                    None => {
                        let mut other = cur.clone();

                        cur.push(SpringState::Operational);
                        other.push(SpringState::Broken);

                        self.maybe_backtrack(cur);
                        self.maybe_backtrack(other);
                    }
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Line {
    states: Vec<Option<SpringState>>,
    broken_groups: Vec<usize>,
}

impl Line {
    #[must_use]
    fn solutions(&self) -> usize {
        SpringStateIterator::new(&self.states, &self.broken_groups)
            .inspect(|st| debug_assert_eq!(broken_groups(st), self.broken_groups))
            .count()
    }

    #[must_use]
    fn unfold(&self) -> Line {
        Line {
            states: Itertools::intersperse(repeat(&self.states).take(5), &vec![None])
                .flatten()
                .copied()
                .collect::<Vec<Option<SpringState>>>(),
            broken_groups: repeat(&self.broken_groups)
                .take(5)
                .flatten()
                .copied()
                .collect::<Vec<usize>>(),
        }
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (states_str, groups_str) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Invalid input line: {s}"))?;

        let states = states_str
            .chars()
            .map(|c| match c {
                '.' => Ok(Some(SpringState::Operational)),
                '#' => Ok(Some(SpringState::Broken)),
                '?' => Ok(None),
                c => Err(anyhow!("Invalid state: {c}")),
            })
            .collect::<Result<Vec<_>>>()?;

        let broken_groups = groups_str
            .split(',')
            .map(|s| s.parse::<usize>().context("Failed to parse integer"))
            .collect::<Result<Vec<_>>>()?;

        Ok(Self {
            states,
            broken_groups,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Input {
    lines: Vec<Line>,
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            lines: s.lines().map(Line::from_str).collect::<Result<Vec<_>>>()?,
        })
    }
}

pub fn solve() -> Result<()> {
    let input = Input::from_str(DAY12_INPUT)?;

    println!(
        "🎁 Part 1 Solution: {}",
        input.lines.iter().map(|l| l.solutions()).sum::<usize>()
    );

    println!(
        "🎁 Part 2 Solution: {}",
        input
            .lines
            .par_iter()
            .map(|l| {
                eprintln!("{l:?}");
                l.unfold().solutions()
            })
            .sum::<usize>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn broken_groups_works() {
        use SpringState::*;

        assert_eq!(broken_groups(&[]), []);
        assert_eq!(
            broken_groups(&[Operational, Broken, Broken, Operational, Broken]),
            [2, 1]
        );
        assert_eq!(broken_groups(&[Operational]), []);
        assert_eq!(broken_groups(&[Broken]), [1]);
    }

    #[test]
    fn spring_state_iterator_works() {
        use SpringState::*;

        let ex1 = [Some(Operational)];
        let bg1: [usize; 0] = [];
        let re1 = vec![vec![Operational]];
        assert_eq!(
            SpringStateIterator::new(&ex1, &bg1).collect::<Vec<_>>(),
            re1
        );

        let ex2 = [None];
        let bg2: [usize; 0] = [];
        let re2 = vec![vec![Operational]];
        assert_eq!(
            SpringStateIterator::new(&ex2, &bg2).collect::<Vec<_>>(),
            re2
        );

        let ex3 = [None];
        let bg3: [usize; 1] = [1];
        let re3 = vec![vec![Broken]];
        assert_eq!(
            SpringStateIterator::new(&ex3, &bg3).collect::<Vec<_>>(),
            re3
        );
    }

    #[test]
    fn unfold_works() {
        use SpringState::*;

        let line = Line {
            states: vec![Some(Operational), Some(Broken)],
            broken_groups: vec![1],
        };

        assert_eq!(
            line.unfold(),
            Line {
                states: vec![
                    Some(Operational),
                    Some(Broken),
                    None,
                    Some(Operational),
                    Some(Broken),
                    None,
                    Some(Operational),
                    Some(Broken),
                    None,
                    Some(Operational),
                    Some(Broken),
                    None,
                    Some(Operational),
                    Some(Broken)
                ],
                broken_groups: vec![1, 1, 1, 1, 1],
            }
        );
    }
}
