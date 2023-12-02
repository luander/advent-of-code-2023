use std::str::FromStr;

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn parse_id(s: &str, separator: char) -> Option<u32> {
    match s.find(separator) {
        None => None,
        Some(index) => match u32::from_str(&s[index + 1..]) {
            Ok(l) => Some(l),
            _ => None,
        },
    }
}

fn parse_data(s: &str, separator: char) -> Option<u32> {
    match s.find(separator) {
        None => None,
        Some(index) => match u32::from_str(&s[..index]) {
            Ok(l) => Some(l),
            _ => None,
        },
    }
}

fn is_game_possible(s: &str) -> bool {
    s.split(';').all(|s| {
        let subset = parse_subset(s);
        subset.0 <= RED && subset.1 <= GREEN && subset.2 <= BLUE
    })
}

fn parse_subset(s: &str) -> (u32, u32, u32) {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    s.split(',').for_each(|s| {
        let data = parse_data(s.trim(), ' ').expect("parse_data to succeed");
        if s.to_lowercase().contains("red") {
            red += data;
        } else if s.to_lowercase().contains("green") {
            green += data;
        } else if s.to_lowercase().contains("blue") {
            blue += data;
        }
    });
    (red, green, blue)
}

fn process_line(line: &str) -> u32 {
    let split: Vec<&str> = line.split(':').collect();
    if let Some(id) = parse_id(split[0], ' ') {
        // println!("Game {:?}", id);
        if is_game_possible(split[1]) {
            // println!("possible {:?}", split[1]);
            return id;
        }
    };
    0
}

fn part1(input: &str) -> u32 {
    input.lines().map(process_line).sum::<u32>()
}

fn main() {
    let input1 = include_str!("../input1.txt");
    let result = part1(input1);
    println!("part1: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let result = part1(test_input);
        assert_eq!(8, result);
    }
}
