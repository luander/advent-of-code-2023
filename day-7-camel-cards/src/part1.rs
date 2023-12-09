use std::{cmp::Ordering, collections::BTreeMap};

fn get_char_value(c: char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => 0,
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    Five,
    Four,
    FullHouse,
    Three,
    TwoPair,
    Pair,
    Card,
}

impl HandType {
    fn from_str(hand: &str) -> Self {
        let mut uniq: BTreeMap<char, u8> = BTreeMap::new();
        hand.chars().for_each(|c| {
            uniq.entry(c).and_modify(|e| *e += 1).or_insert(1);
        });
        let mut v = Vec::from_iter(uniq.iter());
        v.sort_by(|&(_, a), &(_, b)| a.cmp(b));
        match v.pop().expect("vector should have enough items") {
            (_, 2) => {
                let mut res = Self::Pair;
                if let Some((_, a)) = v.pop() {
                    if *a == 2 {
                        res = Self::TwoPair
                    }
                }
                res
            }
            (_, 3) => {
                let mut res = Self::Three;
                if let Some((_, a)) = v.pop() {
                    if *a == 2 {
                        res = Self::FullHouse
                    }
                }
                res
            }
            (_, 4) => Self::Four,
            (_, 5) => Self::Five,
            _ => Self::Card,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<'a> {
    hand: &'a str,
    kind: HandType,
}

impl<'a> Hand<'a> {
    fn from_str(input: &'a str) -> Self {
        Self {
            hand: input,
            kind: HandType::from_str(input),
        }
    }
}

impl<'a> PartialOrd for Hand<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> Ord for Hand<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => {
                for i in 0..self.hand.len() {
                    let char_a = self.hand.chars().nth(i).expect("character to be available");
                    let char_b = other
                        .hand
                        .chars()
                        .nth(i)
                        .expect("character to be available");
                    if char_a != char_b {
                        return get_char_value(char_b).cmp(&get_char_value(char_a));
                    }
                }
                Ordering::Equal
            }
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    }
}

pub fn process(input: &str) -> anyhow::Result<u32> {
    let mut hands = input
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<&str>>();
            let hand = Hand::from_str(parts[0]);
            let bid = parts[1].parse().expect("bid is a number");
            (hand, bid)
        })
        .collect::<Vec<(Hand, u32)>>();
    hands.sort_by(|(a, _), (b, _)| b.partial_cmp(a).expect("comparison to succeed"));
    Ok(hands
        .iter()
        .enumerate()
        .map(|(i, (_, bid))| (i as u32 + 1) * *bid)
        .sum::<u32>())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let test_input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let result = process(test_input)?;
        assert_eq!(6440, result);
        Ok(())
    }
}
