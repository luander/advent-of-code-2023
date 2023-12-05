use std::collections::HashSet;
use std::str::FromStr;

fn parse_id(s: &str, separator: char) -> Option<u32> {
    match s.find(separator) {
        None => None,
        Some(index) => match u32::from_str(&s[index + 1..]) {
            Ok(l) => Some(l),
            _ => None,
        },
    }
}

fn parse_nums(s: &str, separator: char) -> HashSet<u32> {
    let mut set: HashSet<u32> = s
        .trim()
        .split(separator)
        .map(|n| u32::from_str(n).unwrap_or_default())
        .collect();
    set.remove(&0);
    set
}

struct Scracthcard {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
    matches: u32,
    card_id: u32,
}

impl Scracthcard {
    fn from_str(input: &str) -> Self {
        let parts: Vec<&str> = input.split(':').collect();
        // let card_id = parse_id(parts[0], ' ');
        let number_parts: Vec<&str> = parts[1].split('|').collect();
        let winning_numbers = parse_nums(number_parts[0], ' ');
        let my_numbers = parse_nums(number_parts[1], ' ');
        Self {
            winning_numbers,
            my_numbers,
            matches: 0,
            card_id: 0,
        }
    }

    pub fn intersection(&self) -> Vec<u32> {
        dbg!(self
            .my_numbers
            .intersection(&self.winning_numbers)
            .map(|x| *x)
            .collect())
    }
}

fn part1(input: &str) -> i32 {
    let mut sum = 0;
    input.lines().for_each(|line| {
        let card = Scracthcard::from_str(line);
        let len = card.intersection().len() as u32;
        println!("len {:?}", len);
        if len != 0 {
            sum += 2_i32.pow(len - 1);
        }
    });
    sum
}

pub fn main() {
    let input1 = include_str!("../input1.txt");
    let result = part1(input1);
    println!("part1: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let result = part1(test_input);
        assert_eq!(13, result);
    }
}
