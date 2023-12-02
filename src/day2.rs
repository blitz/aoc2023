use std::{cmp::max, str::FromStr};

use anyhow::{anyhow, bail, Context, Result};
use colored::Colorize;

const DAY2_INPUT: &str = std::include_str!("day2.input");

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

impl Reveal {
    fn is_superset_of(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn merge_max(&self, other: &Self) -> Self {
        Reveal {
            red: max(self.red, other.red),
            green: max(self.green, other.green),
            blue: max(self.blue, other.blue),
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

impl FromStr for Reveal {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for m in s.trim().split(',') {
            let (num_str, color_str) = m
                .trim()
                .split_once(' ')
                .ok_or_else(|| anyhow!("No comma?"))?;
            let num = u32::from_str(num_str).context("Can't parse number")?;

            match color_str {
                "red" => red = num,
                "green" => green = num,
                "blue" => blue = num,
                _ => bail!("Unknown color name!"),
            }
        }

        Ok(Reveal { red, green, blue })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_id, reveal_str) = s
            .split_once(':')
            .ok_or_else(|| anyhow!("Game string doesn't contain ':'"))?;

        let (prefix, game_id) = game_id
            .split_once(' ')
            .ok_or_else(|| anyhow!("Couldn't split of game ID"))?;

        if prefix != "Game" {
            bail!("Game doesn't start with the string 'Game '");
        }

        let reveals = reveal_str
            .split(';')
            .map(Reveal::from_str)
            .collect::<Result<Vec<_>>>()?;

        Ok(Game {
            id: game_id.parse()?,
            reveals,
        })
    }
}

fn sum_of_possibles(total: &str, input: &str) -> Result<u32> {
    let total = Reveal::from_str(total)?;
    let games = input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok(games
        .into_iter()
        .filter(|g| g.reveals.iter().all(|r| total.is_superset_of(r)))
        .map(|g| g.id)
        .sum())
}

fn minimal_bag(reveals: &[Reveal]) -> Reveal {
    reveals
        .iter()
        .fold(Reveal::default(), |acc, val| acc.merge_max(val))
}

fn sum_power_of_input(input: &str) -> Result<u32> {
    let games = input
        .lines()
        .map(Game::from_str)
        .collect::<Result<Vec<_>>>()?;

    Ok(games
        .into_iter()
        .map(|g| minimal_bag(&g.reveals).power())
        .sum())
}

pub fn solve() -> Result<()> {
    println!(
        "🎁 Part 1 Solution: {}",
        sum_of_possibles("12 red, 13 green, 14 blue", DAY2_INPUT)?
            .to_string()
            .bold()
    );

    println!(
        "🎁 Part 2 Solution: {}",
        sum_power_of_input(DAY2_INPUT)?.to_string().bold()
    );

    Ok(())
}

#[cfg(test)]
mod testests {
    use super::*;

    #[test]
    fn can_parse_reveals() {
        assert_eq!(
            Reveal::from_str("1 blue, 2 green").unwrap(),
            Reveal {
                blue: 1,
                green: 2,
                red: 0,
            }
        );

        assert_eq!(
            Reveal::from_str("12 red, 13 green, 14 blue").unwrap(),
            Reveal {
                red: 12,
                green: 13,
                blue: 14,
            }
        )
    }

    #[test]
    fn can_parse_games() {
        assert_eq!(
            Game::from_str("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red").unwrap(),
            Game {
                id: 2,
                reveals: vec![
                    Reveal {
                        red: 0,
                        green: 2,
                        blue: 1,
                    },
                    Reveal {
                        red: 1,
                        green: 3,
                        blue: 4,
                    }
                ]
            }
        )
    }

    #[test]
    fn example1_works() {
        assert_eq!(
            sum_of_possibles(
                "12 red, 13 green, 14 blue",
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )
            .unwrap(),
            8
        );
    }

    #[test]
    fn example2_works() {
        assert_eq!(
            sum_power_of_input(
                "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
            )
            .unwrap(),
            2286
        );
    }
}
