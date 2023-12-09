pub fn process(input: &str) -> anyhow::Result<u32> {
    Ok(todo!())
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[test]
    fn part1_works() -> anyhow::Result<()> {
        let test_input = "";

        let result = process(test_input)?;
        assert_eq!(todo!(), result);
        Ok(())
    }
}
