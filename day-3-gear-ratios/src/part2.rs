use itertools::Itertools;

const INPUT: &str = include_str!("../input1.txt");

#[derive(Clone, Copy)]
enum Entry {
    Empty,
    Symbol(char),
    Digit(u32),
}

impl From<char> for Entry {
    fn from(value: char) -> Self {
        let parsed_value = value.to_digit(10);

        if let Some(x) = parsed_value {
            Self::Digit(x)
        } else {
            match value {
                '.' => Entry::Empty,
                x => Entry::Symbol(x),
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct PositionNumber {
    start_col: usize,
    line: usize,
    len: usize,
    number: u32,
}

impl PositionNumber {
    fn is_close(&self, symbol: &PositionSymbol) -> bool {
        for i in 0..self.len {
            if (self.start_col + i).abs_diff(symbol.col) <= 1
                && (self.line).abs_diff(symbol.line) <= 1
            {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Clone, Copy)]
struct PositionSymbol {
    col: usize,
    line: usize,
    symbol: char,
}

impl PositionSymbol {
    fn get_numbers_close(
        self,
        pns: &[PositionNumber],
    ) -> impl Iterator<Item = PositionNumber> + '_ {
        pns.iter().copied().filter(move |pn| pn.is_close(&self))
    }
}

#[derive(Debug)]
struct Schematic {
    numbers: Vec<PositionNumber>,
    symbols: Vec<PositionSymbol>,
}

impl Schematic {
    fn get_part_numbers(&self) -> impl Iterator<Item = PositionNumber> + '_ {
        self.numbers
            .iter()
            .copied()
            .filter(|pn| self.symbols.iter().any(|s| pn.is_close(s)))
    }
}

fn parse_contents(contents: &str) -> Schematic {
    let mut numbers = vec![];
    let mut symbols = vec![];

    let columns = contents
        .lines()
        .next()
        .expect("File should not be empty")
        .len();

    let mut digits_buffer = Vec::with_capacity(columns);

    for (i, line) in contents.lines().enumerate() {
        let entries = parse_line(line);

        let pos_symbols = entries.iter().enumerate().filter_map(|(j, e)| {
            if let Entry::Symbol(x) = e {
                Some(PositionSymbol {
                    line: i,
                    col: j,
                    symbol: *x,
                })
            } else {
                None
            }
        });

        let grouped_digits = entries
            .iter()
            .copied()
            .enumerate()
            .group_by(|(_, e)| matches!(e, Entry::Digit(_)));

        let pos_numbers = grouped_digits.into_iter().flat_map(|(k, g)| {
            if !k {
                None
            } else {
                digits_buffer.extend(g);

                let g = &digits_buffer;
                let n = g.len();

                let start_col = g[0].0;

                let mut number = 0;
                for (k, entry) in g.iter().enumerate() {
                    match entry.1 {
                        Entry::Digit(x) => number += 10_usize.pow((n - k - 1) as u32) * x as usize,
                        _ => unreachable!("Groups have been filtered to only contain digits"),
                    }
                }

                digits_buffer.clear();

                Some(PositionNumber {
                    len: n,
                    start_col,
                    line: i,
                    number: number as u32,
                })
            }
        });

        numbers.extend(pos_numbers);
        symbols.extend(pos_symbols);
    }

    Schematic { numbers, symbols }
}

fn parse_line(line: &str) -> Vec<Entry> {
    line.chars().map(Entry::from).collect()
}

fn part2(contents: &str) -> u32 {
    let schematic = parse_contents(contents);

    let symbols = schematic.symbols;

    symbols
        .iter()
        .filter(|symbol| symbol.symbol == '*')
        .filter_map(|symbol| {
            let numbers_close = symbol.get_numbers_close(&schematic.numbers);

            if let Some((x, y)) = numbers_close.collect_tuple() {
                Some((x, y))
            } else {
                None
            }
        })
        .map(|numbers_close| numbers_close.0.number * numbers_close.1.number)
        .sum()
}

pub fn main() {
    println!("{}", part2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn it_works2() {
        assert_eq!(part2(TEST_INPUT), 467835);
    }
}
