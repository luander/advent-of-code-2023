fn part1(input: &str) -> u32 {
    0
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
        assert_eq!(0, result);
    }
}
