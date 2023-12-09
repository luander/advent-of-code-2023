use itertools::Itertools;

fn predict_line(history: &str) -> i32 {
    let mut values: Vec<Vec<i32>> = vec![];
    values.push(
        history
            .split_whitespace()
            .map(|a| a.parse().expect("all values are parseable"))
            .collect::<Vec<i32>>(),
    );

    let mut lasts: Vec<i32> = vec![];
    while values.last().unwrap().iter().any(|a| *a != 0) {
        lasts.push(values.last().unwrap().last().unwrap().to_owned());
        let tmp = values
            .last()
            .unwrap()
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect::<Vec<i32>>();
        values.push(tmp);
    }
    lasts.into_iter().sum()
}

pub fn process(input: &str) -> anyhow::Result<i32> {
    let result = input.lines().map(predict_line).sum();
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let test_input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let result = process(test_input)?;
        assert_eq!(114, result);
        Ok(())
    }
}
