use anyhow::Context;
use {{ crate_name }}::part2::process;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> anyhow::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("../../input.txt");
    let result = process(input).context("process part2")?;
    println!("part2: {}", result);
    Ok(())
}
