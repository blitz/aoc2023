use std::{
    cmp::{Ordering, Reverse},
    str::FromStr,
};

use anyhow::{anyhow, bail, Context, Result};
use itertools::{partition, Itertools};

const DAY7_INPUT: &str = std::include_str!("day7.input");

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    Joker,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Card::*;

        Ok(match s {
            "2" => N2,
            "3" => N3,
            "4" => N4,
            "5" => N5,
            "6" => N6,
            "7" => N7,
            "8" => N8,
            "9" => N9,
            "T" => T,
            "J" => J,
            "Q" => Q,
            "K" => K,
            "A" => A,
            _ => bail!("Unknown card face"),
        })
    }
}

impl Card {
    fn j_to_toker(self) -> Self {
        if self == Card::J {
            Card::Joker
        } else {
            self
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

impl From<[Card; 5]> for Hand {
    fn from(value: [Card; 5]) -> Self {
        Self { cards: value }
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO The extra string allocations could be optimized away if we iterate over slices of length 1.
        let card_vector: Vec<Card> = s
            .chars()
            .map(|c| Card::from_str(&c.to_string()))
            .collect::<Result<Vec<_>>>()?;

        let card_array: [Card; 5] = card_vector
            .try_into()
            .map_err(|_| anyhow!("Failed array conversion"))?;

        Ok(Self { cards: card_array })
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn as_joker_hand(&self) -> Hand {
        Hand {
            cards: self.cards.map(Card::j_to_toker),
        }
    }

    fn count_card(&self, card: Card) -> usize {
        self.cards.into_iter().filter(|c| *c == card).count()
    }

    fn sorted_cards(&self) -> [Card; 5] {
        let mut cards_to_sort = self.cards;
        cards_to_sort.sort();
        cards_to_sort
    }

    fn kind(&self) -> Kind {
        debug_assert!(self.cards.iter().all(|c| *c != Card::Joker));

        let sorted_cards = self.sorted_cards();
        let card_counts = sorted_cards.map(|c| self.count_card(c));

        // Highest to lowest
        let mut sorted_card_counts = card_counts;
        sorted_card_counts.sort_by_key(|w| Reverse(*w));

        if sorted_card_counts[0] == 5 {
            return Kind::FiveOfAKind;
        }

        if sorted_card_counts[0] == 4 {
            return Kind::FourOfAKind;
        }

        if sorted_card_counts[0] == 3 && sorted_card_counts[3] == 2 {
            return Kind::FullHouse;
        }

        if sorted_card_counts[0] == 3 {
            return Kind::ThreeOfAKind;
        }

        if sorted_card_counts[0] == 2 && sorted_card_counts[2] == 2 {
            return Kind::TwoPair;
        }

        if sorted_card_counts[0] == 2 {
            return Kind::OnePair;
        }

        Kind::HighCard
    }

    fn kind_with_jokers(&self) -> Kind {
        let mut cards = self.cards;
        let split_point = partition(&mut cards, |c| *c == Card::Joker);
        let (jokers, normal_cards) = cards.split_at_mut(split_point);

        debug_assert!(normal_cards.iter().all(|c| *c != Card::Joker));
        debug_assert!(jokers.iter().all(|c| *c == Card::Joker));

        let jokers = jokers.len();

        normal_cards.sort();

        let mut card_counts = normal_cards
            .iter()
            .copied()
            .map(|c| self.count_card(c))
            .collect::<Vec<_>>();
        card_counts.sort_by_key(|w| Reverse(*w));

        if jokers == 5 || jokers + card_counts[0] == 5 {
            return Kind::FiveOfAKind;
        }

        if jokers == 4 || jokers + card_counts[0] == 4 {
            return Kind::FourOfAKind;
        }

        debug_assert!(jokers < 4);

        if jokers == 3 {
            // This will always be a Five of a Kind or Four of a Kind.
            unreachable!();
        }

        if jokers == 2 {
            debug_assert!(card_counts[0] <= 2);

            if card_counts[0] == 2 {
                // Four of a kind.
                return Kind::FullHouse;
            }

            if card_counts[0] == 1 {
                // We can match the jokers to any other card.
                return Kind::ThreeOfAKind;
            }
        }

        if jokers == 1 {
            debug_assert!(card_counts[0] <= 2);

            if card_counts[0] == 2 {
                if card_counts[2] == 2 {
                    return Kind::FullHouse;
                } else {
                    return Kind::ThreeOfAKind;
                }
            } else {
                return Kind::OnePair;
            }
        }

        self.kind()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_kind = self.kind_with_jokers();
        let other_kind = other.kind_with_jokers();

        match self_kind.cmp(&other_kind) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self.cards.cmp(&other.cards),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct HandBid {
    hand: Hand,
    bid: u32,
}

impl HandBid {
    fn as_joker_hand(&self) -> Self {
        HandBid {
            hand: self.hand.as_joker_hand(),
            bid: self.bid,
        }
    }
}

impl FromStr for HandBid {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split_ascii_whitespace().collect::<Vec<_>>();
        if parts.len() != 2 {
            bail!("Need two space separated strings: {}", s);
        }

        Ok(HandBid {
            hand: Hand::from_str(parts[0])?,
            bid: u32::from_str(parts[1])?,
        })
    }
}

pub fn solve() -> Result<()> {
    let input = DAY7_INPUT
        .lines()
        .map(HandBid::from_str)
        .collect::<Result<Vec<_>>>()
        .context("Can't parse input")?;

    println!(
        "🎁 Part 1 Solution: {}",
        input
            .iter()
            .sorted_by_key(|hb| hb.hand)
            .enumerate()
            .map(|(i, hb)| (i + 1) * usize::try_from(hb.bid).unwrap())
            .sum::<usize>()
    );

    println!(
        "🎁 Part 2 Solution: {}",
        input
            .iter()
            .map(|hb| hb.as_joker_hand())
            .sorted_by_key(|hb| hb.hand)
            .enumerate()
            .map(|(i, hb)| (i + 1) * usize::try_from(hb.bid).unwrap())
            .sum::<usize>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cards_are_ordered_correctly() {
        use Card::*;

        assert!(N2 == N2);
        assert!(N2 < N3);
        assert!(N2 < A);
    }

    #[test]
    fn can_parse_hand() -> Result<()> {
        use Card::*;

        assert_eq!(Hand::from_str("T55J5")?, [T, N5, N5, J, N5].into());

        Ok(())
    }

    #[test]
    fn recognizes_kind() -> Result<()> {
        use Kind::*;

        assert_eq!(Hand::from_str("55555")?.kind(), FiveOfAKind);
        assert_eq!(Hand::from_str("5A555")?.kind(), FourOfAKind);
        assert_eq!(Hand::from_str("5A5A5")?.kind(), FullHouse);
        assert_eq!(Hand::from_str("5A5K5")?.kind(), ThreeOfAKind);
        assert_eq!(Hand::from_str("5A5AK")?.kind(), TwoPair);
        assert_eq!(Hand::from_str("J5Q5A")?.kind(), OnePair);
        assert_eq!(Hand::from_str("J5Q2A")?.kind(), HighCard);

        Ok(())
    }

    #[test]
    fn recognizes_kind_with_jokers() -> Result<()> {
        use Kind::*;

        assert_eq!(
            Hand::from_str("32T3K")?.as_joker_hand().kind_with_jokers(),
            OnePair
        );
        assert_eq!(
            Hand::from_str("KK677")?.as_joker_hand().kind_with_jokers(),
            TwoPair
        );
        assert_eq!(
            Hand::from_str("T55J5")?.as_joker_hand().kind_with_jokers(),
            FourOfAKind
        );
        assert_eq!(
            Hand::from_str("KTJJT")?.as_joker_hand().kind_with_jokers(),
            FourOfAKind
        );
        assert_eq!(
            Hand::from_str("QQQJA")?.as_joker_hand().kind_with_jokers(),
            FourOfAKind
        );

        Ok(())
    }

    #[test]
    fn can_compare_hands() -> Result<()> {
        assert!(Hand::from_str("55555")? > Hand::from_str("5A555")?);
        assert!(Hand::from_str("55554")? > Hand::from_str("55553")?);

        Ok(())
    }

    #[test]
    fn example_works() -> Result<()> {
        let example = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let hand_bids = example
            .lines()
            .map(HandBid::from_str)
            .collect::<Result<Vec<_>>>()
            .context("Can't parse hand-bid")?;

        let sorted_hand_bids = hand_bids
            .into_iter()
            .sorted_by_key(|hb| hb.hand)
            .collect::<Vec<_>>();

        eprintln!("{:?}", sorted_hand_bids);

        assert_eq!(
            sorted_hand_bids
                .into_iter()
                .enumerate()
                .map(|(i, hb)| (i + 1) * usize::try_from(hb.bid).unwrap())
                .sum::<usize>(),
            6440
        );

        Ok(())
    }

    #[test]
    fn example2_works() -> Result<()> {
        let example = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let hand_bids = example
            .lines()
            .map(|l| HandBid::from_str(l).map(|hb| hb.as_joker_hand()))
            .collect::<Result<Vec<_>>>()
            .context("Can't parse hand-bid")?;

        let sorted_hand_bids = hand_bids
            .into_iter()
            .sorted_by_key(|hb| hb.hand)
            .collect::<Vec<_>>();

        eprintln!("{:?}", sorted_hand_bids);

        assert_eq!(
            sorted_hand_bids
                .into_iter()
                .enumerate()
                .map(|(i, hb)| (i + 1) * usize::try_from(hb.bid).unwrap())
                .sum::<usize>(),
            5905
        );

        Ok(())
    }
}
