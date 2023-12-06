pub fn process(input: &str) -> u32 {
    let time_line = &input.lines().next().unwrap()[9..];
    let time = time_line
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    let distance_line = &input.lines().nth(1).unwrap()[9..];
    let distance = distance_line
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut result = 1;
    for i in 0..time.len() {
        let mut record_beaten = 0;
        // zero always leads to zero distance
        for speed in 1..time[i] {
            let travel_time = time[i] - speed;
            if (speed * travel_time) > distance[i] {
                record_beaten += 1;
            }
        }
        result *= record_beaten;
    }
    result
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
        let test_input = "Time:      7  15   30
Distance:  9  40  200";
        let result = process(test_input);
        assert_eq!(288, result);
    }
}
