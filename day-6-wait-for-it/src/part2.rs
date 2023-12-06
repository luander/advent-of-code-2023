fn part2(input: &str) -> u64 {
    let time_line = &input.lines().next().unwrap()[9..];
    let time = String::from(time_line)
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    let distance_line = &input.lines().nth(1).unwrap()[9..];
    let distance = String::from(distance_line)
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    let mut from = 0;
    let mut to = 0;
    // zero always leads to zero distance
    for speed in 1..time {
        let travel_time = time - speed;
        if (speed * travel_time) > distance {
            from = speed;
            break;
        }
    }
    for speed in (1..time).rev() {
        let travel_time = time - speed;
        if (speed * travel_time) > distance {
            to = speed;
            break;
        }
    }
    to - from + 1
}

pub fn main() {
    let input1 = include_str!("../input1.txt");
    let result = part2(input1);
    println!("part2: {}", result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2_works() {
        let test_input = "Time:      7  15   30
Distance:  9  40  200";
        let result = part2(test_input);
        assert_eq!(71503, result);
    }
}
