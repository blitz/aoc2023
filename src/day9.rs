use anyhow::{Context, Result};
use itertools::Itertools;

const DAY9_INPUT: &str = include_str!("day9.input");

fn parse_input(input: &str) -> Result<Vec<Vec<i64>>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|token| token.parse::<i64>().context("Can't parse number"))
                .collect::<Result<Vec<_>>>()
        })
        .collect::<Result<Vec<Vec<_>>>>()
}

fn extrapolation_vectors(input: &[i64]) -> Vec<Vec<i64>> {
    let mut derivations: Vec<Vec<i64>> = vec![input.to_owned()];

    loop {
        let next = derivations
            .last()
            .unwrap()
            .iter()
            .copied()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect::<Vec<_>>();

        if next.iter().copied().all(|v| v == 0) {
            break;
        }

        derivations.push(next);
    }

    derivations
}

fn extrapolate_fwd(input: &[i64]) -> i64 {
    let derivations = extrapolation_vectors(input);

    derivations
        .into_iter()
        .map(|v| *v.last().unwrap())
        .rev()
        .sum::<i64>()
}

fn extrapolate_bwd(input: &[i64]) -> i64 {
    let derivations = extrapolation_vectors(input);

    derivations
        .into_iter()
        .map(|v| *v.first().unwrap())
        .rev()
        .fold(0, |last, val| val - last)
}

pub fn solve() -> Result<()> {
    let input = parse_input(DAY9_INPUT)?;

    println!(
        "🎁 Part 1 Solution: {}",
        input.iter().map(|v| extrapolate_fwd(v)).sum::<i64>()
    );

    println!(
        "🎁 Part 2 Solution: {}",
        input.iter().map(|v| extrapolate_bwd(v)).sum::<i64>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_works() -> Result<()> {
        assert_eq!(extrapolate_fwd(&[0, 3, 6, 9, 12, 15]), 18);
        assert_eq!(extrapolate_fwd(&[1, 3, 6, 10, 15, 21]), 28);

        Ok(())
    }

    #[test]
    fn example2_works() -> Result<()> {
        assert_eq!(extrapolate_bwd(&[10, 13, 16, 21, 30, 45]), 5);

        Ok(())
    }
}
