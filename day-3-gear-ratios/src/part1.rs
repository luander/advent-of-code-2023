fn is_symbol(c: char) -> bool {
    // in this case, everything that's not either a number or a '.'
    // will be considered as a symbol
    !c.is_numeric() && '.' != c
}

struct Schematic {
    parts: Vec<u32>,
}

impl Schematic {
    fn from_str(input: &str) -> Self {
        let mut schematic = Schematic { parts: vec![] };
        input.lines().enumerate().for_each(|(line_idx, line)| {
            let mut chars = line.chars().enumerate();
            println!("{line}");
            while let Some((i, c)) = chars.next() {
                let mut is_part = false;
                if c.is_numeric() {
                    let mut num = String::new();
                    num.push(c);

                    println!("next char is {} idx: {}", c, i);
                    is_part |= schematic.is_part(input, line_idx, i);
                    while let Some((i, n)) = chars.next() {
                        if n.is_numeric() {
                            println!("next char is {} idx: {}", n, i);
                            is_part |= schematic.is_part(input, line_idx, i);
                            num.push(n);
                        } else {
                            // println!("next char is {} idx: {}", n, i);
                            // is_part |= schematic.is_part(input, line_idx, i);
                            break;
                        }
                    }
                    if is_part {
                        print!("{} ", num);
                        schematic
                            .parts
                            .push(num.parse().expect("should be parseable to int"));
                    }
                }
            }
            println!("");
        });
        schematic
    }

    fn is_part(&self, input: &str, line_idx: usize, i: usize) -> bool {
        let mut result = false;
        let prev_line;
        if line_idx != 0 {
            prev_line = input.lines().nth(line_idx - 1);
        } else {
            prev_line = None;
        }
        let cur_line = input
            .lines()
            .nth(line_idx)
            .expect("current line should always succeed");
        let next_line = input.lines().nth(line_idx + 1);

        // x . .
        // . i .
        // . . .
        //
        if let Some(line) = prev_line {
            if i != 0 {
                if let Some(c) = line.chars().nth(i - 1) {
                    println!(
                        "looking at {} ({}, {}): is_symbol -> {}",
                        c,
                        i,
                        line_idx,
                        is_symbol(c)
                    );
                    result |= is_symbol(c);
                }
            }
        }
        // . x .
        // . i .
        // . . .
        //
        if let Some(line) = prev_line {
            if let Some(c) = line.chars().nth(i) {
                println!(
                    "looking at {} ({}, {}): is_symbol -> {}",
                    c,
                    i,
                    line_idx,
                    is_symbol(c)
                );
                result |= is_symbol(c);
            }
        }
        // . . x
        // . i .
        // . . .
        //
        if let Some(line) = prev_line {
            if i <= line.len() {
                if let Some(c) = line.chars().nth(i + 1) {
                    println!(
                        "looking at {} ({}, {}): is_symbol -> {}",
                        c,
                        i,
                        line_idx,
                        is_symbol(c)
                    );
                    result |= is_symbol(c);
                }
            }
        }
        // . . .
        // x i .
        // . . .
        //
        if i != 0 {
            if let Some(c) = cur_line.chars().nth(i - 1) {
                println!(
                    "looking at {} ({}, {}): is_symbol -> {}",
                    c,
                    i,
                    line_idx,
                    is_symbol(c)
                );
                result |= is_symbol(c);
            }
        }
        // . . .
        // . i x
        // . . .
        //
        if i <= cur_line.len() {
            if let Some(c) = cur_line.chars().nth(i + 1) {
                println!(
                    "looking at {} ({}, {}): is_symbol -> {}",
                    c,
                    i + 1,
                    line_idx,
                    is_symbol(c)
                );
                result |= is_symbol(c);
            }
        }
        // . . .
        // . i .
        // x . .
        //
        if let Some(line) = next_line {
            if i != 0 {
                if let Some(c) = line.chars().nth(i - 1) {
                    println!(
                        "looking at {} ({}, {}): is_symbol -> {}",
                        c,
                        i - 1,
                        line_idx + 1,
                        is_symbol(c)
                    );
                    result |= is_symbol(c);
                }
            }
        }
        // . . .
        // . i .
        // . x .
        //
        if let Some(line) = next_line {
            if let Some(c) = line.chars().nth(i) {
                println!(
                    "looking at {} ({}, {}): is_symbol -> {}",
                    c,
                    i,
                    line_idx,
                    is_symbol(c)
                );
                result |= is_symbol(c);
            }
        }
        // . . .
        // . i .
        // . . x
        //
        if let Some(line) = next_line {
            if i <= line.len() {
                if let Some(c) = line.chars().nth(i + 1) {
                    println!(
                        "looking at {} ({}, {}): is_symbol -> {}",
                        c,
                        i,
                        line_idx,
                        is_symbol(c)
                    );
                    result |= is_symbol(c);
                }
            }
        }
        result
    }
}

fn part1(input: &str) -> u32 {
    let sch = Schematic::from_str(input);
    dbg!(&sch.parts);
    sch.parts.iter().sum()
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
        let test_input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
.*..423.*.";

        let result = part1(test_input);
        assert_eq!(4361, result);
    }
}
