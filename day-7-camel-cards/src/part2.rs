use std::{cmp::Ordering, collections::BTreeMap};

fn get_char_value(c: char) -> u8 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'J' => 1,
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
        let joker = *uniq.get(&'J').unwrap_or(&0);
        let mut v = Vec::from_iter(uniq.iter());
        v.sort_by(|&(_, a), &(_, b)| a.cmp(b));
        //
        // No J -> do nothing
        // All Js -> do nothing
        let binding: u8;
        if joker != 0 && joker != 5 {
            if let Some((c, a)) = v.pop() {
                // x Js -> add X to most common value
                if *c != 'J' {
                    binding = a + joker;

                    v.push((c, &binding));
                // J is most common -> add J to second most common
                } else if *c == 'J' {
                    let tmp = v.pop().expect("second most common is available");
                    binding = tmp.1 + joker;
                    v.push((tmp.0, &binding));
                    v.push((c, a));
                } else {
                    v.push((c, a));
                }
            }
            v = v
                .iter_mut()
                .map(|(c, a)| if **c == 'J' { (*c, &0) } else { (*c, *a) })
                .collect::<Vec<(&char, &u8)>>();
        }

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
    fn part2_works() -> anyhow::Result<()> {
        let test_input = "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483";
        //         let test_input = "2345A 1
        // Q2KJJ 13
        // Q2Q2Q 19
        // T3T3J 17
        // T3Q33 11
        // 2345J 3
        // J345A 2
        // 32T3K 5
        // T55J5 29
        // KK677 7
        // KTJJT 34
        // QQQJA 31
        // JJJJJ 37
        // JAAAA 43
        // AAAAJ 59
        // AAAAA 61
        // 2AAAA 23
        // 2JJJJ 53
        // JJJJ2 41";

        let result = process(test_input)?;
        assert_eq!(5905, result);
        // assert_eq!(6839, result);
        Ok(())
    }
}
