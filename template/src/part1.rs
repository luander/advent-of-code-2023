pub fn process(input: &str) -> u32 {
    todo!()
}

pub fn main() {
    let input = include_str!("../input.txt");
    let result = process(input);
    println!("part1: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() {
        let test_input = "";

        let result = process(test_input);
        assert_eq!(todo!(), result);
    }
}
