use day2::round::Round;
use itertools::Itertools;
use std::str::FromStr;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let score: u64 = itertools::process_results(
        include_str!("input.txt")
            .lines()
            .map(Round::from_str)
            .map_ok(|round| round.rhs_score()),
        |game| game.sum(),
    )?;
    println!("{score}");

    Ok(())
}
