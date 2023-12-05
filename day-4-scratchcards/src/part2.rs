use std::collections::HashSet;
use std::str::FromStr;

fn parse_id(s: &str, separator: char) -> Option<usize> {
    match s.rfind(separator) {
        None => None,
        Some(index) => match usize::from_str(&s[index + 1..]) {
            Ok(l) => Some(l),
            _ => None,
        },
    }
}

fn parse_nums(s: &str, separator: char) -> HashSet<usize> {
    let mut set: HashSet<usize> = s
        .trim()
        .split(separator)
        .map(|n| usize::from_str(n).unwrap_or_default())
        .collect();
    set.remove(&0);
    set
}

#[derive(Debug)]
struct Part2 {
    /// pile contains all the matches for all cards, in order
    pile: Vec<usize>,
}

impl Part2 {
    fn new() -> Self {
        Self { pile: Vec::new() }
    }

    pub fn add_card(&mut self, card: Scracthcard) {
        self.pile.push(card.matches);
    }

    pub fn process(&mut self) -> usize {
        let mut counts: Vec<usize> = vec![1; self.pile.len()];
        for (idx, new_cards) in self.pile.iter().enumerate().rev() {
            counts[idx] += counts[idx + 1..][..*new_cards]
                .iter()
                .copied()
                .sum::<usize>();
        }
        counts.into_iter().sum()
    }

    // pub fn _process(&mut self) -> u32 {
    //     let mut i = 0;
    //     loop {
    //         if i >= self.pile.len() {
    //             break;
    //         }
    //
    //         let card = &self.pile[i].clone();
    //         for j in 1..=card.matches {
    //             if let Some(new_card) = self
    //                 .pile
    //                 .iter()
    //                 .find(|other_card| other_card.card_id == card.card_id + j)
    //             {
    //                 self.pile.push(new_card.clone());
    //             }
    //         }
    //         i += 1;
    //     }
    //     self.pile.len() as u32
    // }
}

#[derive(Debug, Clone)]
struct Scracthcard {
    matches: usize,
    card_id: usize,
}

impl Scracthcard {
    fn from_str(input: &str) -> Self {
        let parts: Vec<&str> = input.split(':').collect();
        let card_id = parse_id(parts[0], ' ');
        let number_parts: Vec<&str> = parts[1].split('|').collect();
        let winning_numbers = parse_nums(number_parts[0], ' ');
        let my_numbers = parse_nums(number_parts[1], ' ');
        let matches = my_numbers
            .intersection(&winning_numbers)
            .cloned()
            .collect::<Vec<usize>>()
            .len();
        Self {
            matches,
            card_id: card_id.unwrap(),
        }
    }
}

fn part1(input: &str) -> usize {
    let mut copies = Copies::new();
    input.lines().for_each(|line| {
        let card = Scracthcard::from_str(line);
        copies.add_card(card);
    });
    copies.process()
}

pub fn main() {
    let input1 = include_str!("../input1.txt");
    let result = part1(input1);
    println!("part2: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2_works() {
        let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = part1(test_input);
        assert_eq!(30, result);
    }
}
