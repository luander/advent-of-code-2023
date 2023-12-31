use anyhow::Context;
use day_7_camel_cards::{part1, part2};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("../input.txt");
    let result = part1::process(input).context("process part1")?;
    println!("part1: {}", result);

    let result = part2::process(input).context("process part2")?;
    println!("part2: {}", result);
    Ok(())
}
