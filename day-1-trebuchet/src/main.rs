fn part1(input: &str) -> usize {
    let mut values = vec![];
    input.lines().for_each(|line| {
        let mut numeric = String::new();
        line.chars().for_each(|c| {
            if c.is_numeric() {
                if numeric.is_empty() {
                    numeric.push(c);
                    numeric.push(c);
                } else {
                    numeric.pop();
                    numeric.push(c);
                }
            }
        });
        let value: usize = numeric.parse::<usize>().unwrap();
        values.push(value);
    });
    values.iter().sum()
}

fn part2(input: &str) -> usize {
    let numbers = vec![
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ];
    let mut lines = vec![];
    input.lines().for_each(|line| {
        let mut numeric = String::new();
        line.char_indices().for_each(|(i, c)| {
            numbers.iter().for_each(|(name, value)| {
                if line[i..].starts_with(name) {
                    numeric.push(*value)
                }
            });
            if c.is_numeric() {
                numeric.push(c);
            }
        });
        lines.push(numeric);
    });

    let mut values = vec![];
    lines.iter().for_each(|line| {
        let mut numeric = String::new();
        line.chars().for_each(|c| {
            if c.is_numeric() {
                if numeric.is_empty() {
                    numeric.push(c);
                    numeric.push(c);
                } else {
                    numeric.pop();
                    numeric.push(c);
                }
            }
        });
        let value: usize = numeric.parse::<usize>().unwrap();
        values.push(value);
    });

    values.iter().sum()
}

fn main() {
    let input1 = include_str!("../input1.txt");
    let result = part1(input1);
    println!("part1: {}", result);
    let result = part2(input1);
    println!("part2: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let result = part1(test_input);
        assert_eq!(142, result);
    }

    #[test]
    fn part2_works() {
        let test_input = "two1nine
eighthree
sevenine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = part2(test_input);
        assert_eq!(443, result);
    }
}
