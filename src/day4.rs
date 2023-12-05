use std::{collections::BTreeSet, str::FromStr};

use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use regex::Regex;

const DAY4_INPUT: &str = std::include_str!("day4.input");

#[derive(Debug, PartialEq, Eq, Clone)]
struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    your_numbers: Vec<u32>,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"^Card +([0-9]+): ([ 0-9]+) \| ([ 0-9]+)$")?;

        let captures = re
            .captures(s)
            .ok_or_else(|| anyhow!("Failed to match regex"))?;

        let capture_group_to_int_vec = |g| {
            captures
                .get(g)
                .unwrap()
                .as_str()
                .split_ascii_whitespace()
                .map(|n| u32::from_str(n).context("Failed to parse integer"))
                .collect::<Result<Vec<_>>>()
        };

        Ok(Self {
            id: u32::from_str(captures.get(1).unwrap().as_str())?,
            winning_numbers: capture_group_to_int_vec(2)?,
            your_numbers: capture_group_to_int_vec(3)?,
        })
    }
}

impl Card {
    /// Returns the number of wins.
    fn wins(&self) -> usize {
        BTreeSet::from_iter(self.winning_numbers.iter())
            .intersection(&BTreeSet::from_iter(self.your_numbers.iter()))
            .count()
    }

    /// The number of points this card is worth.
    fn win_points(&self) -> usize {
        match self.wins() {
            0 => 0,
            n => 1 << (n - 1),
        }
    }
}

fn part2_solve(cards: &[Card]) -> Result<usize> {
    let card_values = cards
        .iter()
        .sorted_by_key(|c| c.id)
        .map(|c| c.wins())
        .collect::<Vec<_>>();

    // For each of the cards, we remember how many copies we have. We
    // start with one of each.
    let mut card_counts: Vec<usize> = vec![1; card_values.len()];

    for pos in 0..card_values.len() {
        let new_cards = card_values[pos];
        let new_copies = card_counts[pos];

        for c in &mut card_counts[(pos + 1)..(pos + 1 + new_cards)] {
            *c = *c + new_copies;
        }
    }

    Ok(card_counts.into_iter().sum())
}

pub fn solve() -> Result<()> {
    let cards = DAY4_INPUT
        .lines()
        .map(Card::from_str)
        .collect::<Result<Vec<_>>>()?;

    println!(
        "🎁 Part 1 Solution: {}",
        cards.iter().map(Card::win_points).sum::<usize>()
    );

    println!("🎁 Part 2 Solution: {}", part2_solve(&cards)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_works() -> Result<()> {
        assert_eq!(
            Card::from_str("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1")?,
            Card {
                id: 3,
                winning_numbers: vec![1, 21, 53, 59, 44],
                your_numbers: vec![69, 82, 63, 72, 16, 21, 14, 1],
            }
        );

        Ok(())
    }

    #[test]
    fn example_works() -> Result<()> {
        let example = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let cards = example
            .lines()
            .map(Card::from_str)
            .collect::<Result<Vec<_>>>()?;

        assert_eq!(cards.iter().map(Card::win_points).sum::<usize>(), 13);

        Ok(())
    }
}
