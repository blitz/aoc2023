use std::{cmp::min, collections::BTreeMap, ops::Range, str::FromStr};

use anyhow::{anyhow, bail, Context, Result};
use itertools::Itertools;

const DAY5_INPUT: &str = std::include_str!("day5.input");

#[derive(Debug, PartialEq, Eq, Clone)]
struct MapEntry {
    dst_range_start: u64,
    src_range_start: u64,
    len: u64,
}

impl MapEntry {
    fn src_range(&self) -> Range<u64> {
        self.src_range_start..(self.src_range_start + self.len)
    }

    fn map_value(&self, v: u64) -> Option<u64> {
        self.src_range().contains(&v).then_some(
            v.overflowing_sub(self.src_range_start)
                .0
                .overflowing_add(self.dst_range_start)
                .0,
        )
    }
}

impl FromStr for MapEntry {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = s
            .split_ascii_whitespace()
            .map(|s| u64::from_str(s).context("Failed to parse integer"))
            .collect::<Result<Vec<u64>>>()?;

        if numbers.len() != 3 {
            bail!("Invalid map entry: {s}");
        }

        Ok(MapEntry {
            dst_range_start: numbers[0],
            src_range_start: numbers[1],
            len: numbers[2],
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct MapEntries(Vec<MapEntry>);

impl From<Vec<MapEntry>> for MapEntries {
    fn from(mut value: Vec<MapEntry>) -> Self {
        value.sort_by_key(|me| me.src_range_start);
        Self(value)
    }
}

impl MapEntries {
    fn map_value(&self, v: u64) -> u64 {
        self.0.iter().find_map(|me| me.map_value(v)).unwrap_or(v)
    }

    fn map_range(&self, r: Range<u64>) -> Range<u64> {
        assert!(!r.is_empty());

        for me in &self.0 {
            let me_range = me.src_range();
            assert!(!me_range.is_empty());

            // r is completely before me_range
            if r.end <= me_range.start {
                return r;
            }

            // r ends in me_range or covers all of me_range. (Could be
            // unified with the case above.)
            assert!(r.end >= me_range.start);
            if r.start < me_range.start {
                return r.start..me_range.start;
            }

            // r starts in me_range.
            if me_range.contains(&r.start) {
                let me_range_offset = r.start - me_range.start;
                let new_len = min(
                    me_range.end - me_range.start - me_range_offset,
                    r.end - r.start,
                );
                assert!(new_len != 0);

                let dst_start = me.dst_range_start + me_range_offset;

                return dst_start..(dst_start + new_len);
            }
        }

        // The range doesn't intersect with any map entry.
        r
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Input {
    seeds: Vec<u64>,

    maps: [MapEntries; 7],
}

impl Input {
    // TODO We could translate a whole range here. This would make part 2 of the problem much more efficient.
    fn seed_to_location(&self, seed: u64) -> u64 {
        self.maps
            .iter()
            .fold(seed, |value, map| map.map_value(value))
    }
}

impl FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let seeds_str = lines
            .next()
            .ok_or_else(|| anyhow!("Failed to read seeds"))?;

        if !seeds_str.starts_with("seeds: ") {
            bail!("Invalid seeds string");
        }

        let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut current_key: Option<String> = None;

        for l in lines {
            if l.trim().is_empty() {
                continue;
            } else if l.contains(':') {
                current_key = Some(l.to_owned())
            } else {
                let current_key: String = current_key.clone().unwrap();

                if let Some(value) = map.get_mut(&current_key) {
                    value.push(l.to_owned())
                } else {
                    map.insert(current_key, vec![l.to_owned()]);
                }
            }
        }

        let parse_map = |name: &str| -> MapEntries {
            map.get(name)
                .unwrap()
                .iter()
                .map(|l| MapEntry::from_str(l).unwrap())
                .collect::<Vec<_>>()
                .into()
        };

        Ok(Input {
            seeds: seeds_str
                .split_ascii_whitespace()
                .skip(1)
                .map(|s| u64::from_str(s).context("Can't parse seed number"))
                .collect::<Result<Vec<_>>>()?,
            maps: [
                parse_map("seed-to-soil map:"),
                parse_map("soil-to-fertilizer map:"),
                parse_map("fertilizer-to-water map:"),
                parse_map("water-to-light map:"),
                parse_map("light-to-temperature map:"),
                parse_map("temperature-to-humidity map:"),
                parse_map("humidity-to-location map:"),
            ],
        })
    }
}

fn find_closest_seed_location(input: &Input) -> Option<u64> {
    input.seeds.iter().map(|s| input.seed_to_location(*s)).min()
}

fn find_closest_seed_location_2(input: &Input) -> Option<u64> {
    input
        .seeds
        .iter()
        .copied()
        .tuples::<(u64, u64)>()
        .map(|(start, len)| {
            assert!(len != 0);
            let seed_range = start..(start + len);

            let mut cur = start;
            let mut candidate_location = u64::MAX;
            loop {
                let range = input
                    .maps
                    .iter()
                    .fold(cur..seed_range.end, |r, map| map.map_range(r));

                // We've managed to translate some
                cur += range.end - range.start;

                candidate_location = min(candidate_location, range.start);

                if !seed_range.contains(&cur) {
                    break;
                }
            }

            candidate_location
        })
        .min()
}

pub fn solve() -> Result<()> {
    let input = Input::from_str(DAY5_INPUT)?;

    println!(
        "🎁 Part 1 Solution: {}",
        find_closest_seed_location(&input).unwrap()
    );
    println!(
        "🎁 Part 2 Solution: {}",
        find_closest_seed_location_2(&input).unwrap()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY5_EXAMPLE: &str = std::include_str!("day5.example");

    #[test]
    fn can_parse_map_entry() -> Result<()> {
        assert_eq!(
            MapEntry::from_str("1 2 3")?,
            MapEntry {
                dst_range_start: 1,
                src_range_start: 2,
                len: 3
            }
        );

        Ok(())
    }

    #[test]
    fn can_map_values() -> Result<()> {
        let me = MapEntry {
            dst_range_start: 10,
            src_range_start: 20,
            len: 5,
        };

        assert_eq!(me.map_value(22), Some(12));
        assert_eq!(me.map_value(25), None);

        Ok(())
    }

    #[test]
    fn can_parse_example() -> Result<()> {
        let example = Input::from_str(DAY5_EXAMPLE)?;

        eprintln!("{example:?}");

        Ok(())
    }
}
