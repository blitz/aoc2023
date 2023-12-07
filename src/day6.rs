use anyhow::Result;

struct Race {
    time_ms: u64,
    record_distance_ms: u64,
}

impl Race {
    const fn new(time_ms: u64, record_distance_ms: u64) -> Self {
        Self {
            time_ms,
            record_distance_ms,
        }
    }

    fn distance(&self, button_press_ms: u64) -> u64 {
        assert!(button_press_ms <= self.time_ms);

        (self.time_ms - button_press_ms) * button_press_ms
    }

    fn winning_moves(&self) -> usize {
        (0..=self.time_ms)
            .map(|bp_ms| self.distance(bp_ms))
            .filter(|d| *d > self.record_distance_ms)
            .count()
    }
}

const DAY6_INPUT: [Race; 4] = [
    Race::new(45, 295),
    Race::new(98, 1734),
    Race::new(83, 1278),
    Race::new(73, 1210),
];

const DAY6_INPUT_PART2: Race = Race::new(45988373, 295173412781210);

pub fn solve() -> Result<()> {
    println!(
        "🎁 Part 1 Solution: {}",
        DAY6_INPUT
            .iter()
            .map(|r| r.winning_moves())
            .product::<usize>()
    );

    // TODO This is pretty slow. There are symmetries that we could use to make this faster.
    println!("🎁 Part 2 Solution: {}", DAY6_INPUT_PART2.winning_moves());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distnace_calculation_works() {
        let example = Race::new(7, 9);

        assert_eq!(example.distance(0), 0);
        assert_eq!(example.distance(4), 12);
        assert_eq!(example.distance(7), 0);

        assert_eq!(example.winning_moves(), 4);
    }
}
