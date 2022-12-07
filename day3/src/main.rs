pub mod item;

use crate::item::Item;
use itertools::Itertools;
use std::collections::HashSet;

fn part_one() -> color_eyre::Result<u64> {
    include_str!("input.txt")
        .lines()
        .map(|line| -> color_eyre::Result<u64> {
            let (first, second) = line.split_at(line.len() / 2);
            let first = first
                .bytes()
                .map(Item::try_from)
                .collect::<Result<HashSet<_>, _>>()?;
            itertools::process_results(second.bytes().map(Item::try_from), |mut it| {
                it.find(|&item| first.contains(&item))
                    .map(|item| item.priority())
                    .ok_or_else(|| {
                        color_eyre::eyre::eyre!("rucksacks compartments have no item in common")
                    })
            })?
        })
        .sum::<color_eyre::Result<u64>>()
}

fn part_two() -> color_eyre::Result<u64> {
    let rucksacks = include_str!("input.txt").lines().map(|line| {
        line.bytes()
            .map(Item::try_from)
            .collect::<Result<HashSet<_>, _>>()
    });

    itertools::process_results(rucksacks, |rs| {
        rs.tuples()
            .map(|(a, b, c)| {
                a.iter()
                    .copied()
                    .find(|i| b.contains(i) && c.contains(i))
                    .map(|i| i.priority())
                    .ok_or_else(|| {
                        color_eyre::eyre::eyre!("rucksacks compartments have no item in common")
                    })
            })
            .sum::<color_eyre::Result<u64>>()
    })?
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let total_priority = part_one()?;
    println!("{total_priority}");

    let total_priority = part_two()?;
    println!("{total_priority}");

    Ok(())
}
