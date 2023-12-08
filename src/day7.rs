use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::Debug;

// N.B. Order is important
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::enum_variant_names)]
enum Kind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

const CARDS_PER_HAND: usize = 5;

#[derive(PartialEq, Eq, Copy, Clone)]
struct Card(u8 /* ASCII value */);

impl Debug for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        (self.0 as char).fmt(f)
    }
}

impl Card {
    pub const JOKER: Self = Self(b'J');
    fn to_index(self) -> usize {
        match self.0 {
            b'0'..=b'9' => self.0 as usize - b'0' as usize,
            b'A' => 10,
            b'K' => 11,
            b'Q' => 12,
            b'J' => 13,
            b'T' => 14,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct CardOrdPart1(Card);
impl PartialOrd for CardOrdPart1 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardOrdPart1 {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0, other.0) {
            (Card(b'0'..=b'9'), Card(b'A' | b'K' | b'Q' | b'J' | b'T')) => Ordering::Less,
            (Card(b'0'..=b'9'), Card(b'0'..=b'9')) => self.0 .0.cmp(&other.0 .0),
            (Card(b'A'), Card(b'K' | b'Q' | b'J' | b'T')) => Ordering::Greater,
            (Card(b'K'), Card(b'Q' | b'J' | b'T')) => Ordering::Greater,
            (Card(b'K'), Card(b'A')) => Ordering::Less,
            (Card(b'Q'), Card(b'J' | b'T')) => Ordering::Greater,
            (Card(b'Q'), Card(b'K' | b'A')) => Ordering::Less,
            (Card(b'J'), Card(b'T')) => Ordering::Greater,
            (Card(b'J'), Card(b'K' | b'A' | b'Q')) => Ordering::Less,
            (Card(b'T'), Card(o)) if o.is_ascii_digit() => Ordering::Greater,
            (Card(b'T'), Card(b'K' | b'A' | b'Q' | b'J')) => Ordering::Less,
            (left, right) if left == right => Ordering::Equal,
            _ => other.cmp(self).reverse(),
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct CardOrdPart2(Card);
impl PartialOrd for CardOrdPart2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CardOrdPart2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.0, other.0) {
            (Card(b'J'), Card(b'J')) => Ordering::Equal,
            (Card(b'J'), _) => Ordering::Less,
            (_, Card(b'J')) => Ordering::Greater,
            _ => CardOrdPart1(self.0).cmp(&CardOrdPart1(other.0)),
        }
    }
}

pub fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (raw_cards, bid) = line.split_once(' ').unwrap();

            let mut card_counts = [0; 15 /* 0 - 9 + AKQJT */];
            let mut cards = [Card(0); CARDS_PER_HAND];

            for (raw_card, card) in raw_cards.bytes().zip(&mut cards) {
                card_counts[Card(raw_card).to_index()] += 1;
                *card = Card(raw_card);
            }

            card_counts.sort_unstable_by(|a, b| b.cmp(a));
            let kind = match card_counts[0] {
                5 => Kind::FiveOfAKind,
                4 => Kind::FourOfAKind,
                3 if card_counts[1] == 2 => Kind::FullHouse,
                3 => Kind::ThreeOfAKind,
                2 if card_counts[1] == 2 => Kind::TwoPair,
                2 => Kind::OnePair,
                1 => Kind::HighCard,
                _ => unreachable!(),
            };

            (kind, cards, bid.parse::<i64>().unwrap())
        })
        .sorted_unstable_by(|(k1, h1, ..), (k2, h2, ..)| {
            (k1, h1.map(CardOrdPart1))
                .cmp(&(k2, h2.map(CardOrdPart1)))
                .reverse()
        })
        .rev()
        .enumerate()
        .map(|(rank, (.., bid))| (rank + 1) as i64 * bid)
        .sum()
}

pub fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (raw_cards, bid) = line.split_once(' ').unwrap();

            let mut card_counts = [0; 15 /* 0 - 9 + AKQJT */];
            let mut cards = [Card(0); CARDS_PER_HAND];

            for (raw_card, card) in raw_cards.bytes().zip(&mut cards) {
                card_counts[Card(raw_card).to_index()] += 1;
                *card = Card(raw_card);
            }

            let jokers = card_counts[Card::JOKER.to_index()];
            card_counts[Card::JOKER.to_index()] = 0;
            card_counts.sort_unstable_by(|a, b| b.cmp(a));
            let kind = match card_counts[0] + jokers {
                5 => Kind::FiveOfAKind,
                4 => Kind::FourOfAKind,
                3 if card_counts[1] == 2 => Kind::FullHouse,
                3 => Kind::ThreeOfAKind,
                2 if card_counts[1] == 2 => Kind::TwoPair,
                2 => Kind::OnePair,
                1 => Kind::HighCard,
                _ => unreachable!(),
            };

            (kind, cards, bid.parse::<i64>().unwrap())
        })
        .sorted_unstable_by(|(k1, h1, ..), (k2, h2, ..)| {
            (k1, h1.map(CardOrdPart2))
                .cmp(&(k2, h2.map(CardOrdPart2)))
                .reverse()
        })
        .rev()
        .enumerate()
        .map(|(rank, (.., bid))| (rank + 1) as i64 * bid)
        .sum()
}

#[cfg(test)]
#[test]
fn p7t() {
    const INPUT: &str = include_str!("../inputs/day7.txt");
    let example = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
        .trim();

    assert_eq!(part1(example), 6440);
    assert_eq!(part1(INPUT), 250474325);
    assert_eq!(part2(example), 5905);
    assert_eq!(part2(INPUT), 248909434);
}
