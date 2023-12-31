use itertools::Itertools;

fn predict_line(history: &str) -> Option<i32> {
    let mut values: Vec<Vec<i32>> = vec![history
        .split_whitespace()
        .map(|a| a.parse().expect("all values are parseable"))
        .collect()];

    let mut next_values: Vec<i32> = vec![];
    while values
        .last()
        .expect("last element in values available")
        .iter()
        .any(|a| *a != 0)
    {
        next_values.push(values.last()?.first()?.to_owned());
        let difference = values
            .last()?
            .iter()
            .tuple_windows()
            .map(|(a, b)| a - b)
            .collect();
        values.push(difference);
    }
    Some(next_values.into_iter().sum())
}

pub fn process(input: &str) -> anyhow::Result<i32> {
    let result = input.lines().flat_map(predict_line).sum();
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
        assert_eq!(2, result);
        Ok(())
    }
}
