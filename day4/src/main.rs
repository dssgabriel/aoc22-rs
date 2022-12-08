use itertools::Itertools;

fn part_one() -> usize {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split(",")
                .map(|sections| {
                    sections
                        .split("-")
                        .map(|s| s.parse().expect("sector must be a `u64`"))
                        .collect_tuple::<(u64, u64)>()
                        .map(|(start, end)| start..=end)
                        .expect("range of sections should have a start and an end")
                })
                .collect_tuple::<(_, _)>()
                .expect("each pair of elves should have a range of sections")
        })
        .filter(|(a, b)| {
            a.contains(b.start()) && a.contains(b.end())
                || b.contains(a.start()) && b.contains(a.end())
        })
        .count()
}

fn part_two() -> usize {
    include_str!("input.txt")
        .lines()
        .map(|line| {
            line.split(",")
                .map(|sections| {
                    sections
                        .split("-")
                        .map(|s| s.parse().expect("sector must be a `u64`"))
                        .collect_tuple::<(u64, u64)>()
                        .map(|(start, end)| start..=end)
                        .expect("range of sections should have a start and an end")
                })
                .collect_tuple::<(_, _)>()
                .expect("each pair of elves should have a range of sections")
        })
        .filter(|(a, b)| {
            a.contains(b.start())
                || a.contains(b.end())
                || b.contains(a.start())
                || b.contains(a.end())
        })
        .count()
}

fn main() {
    let redundant = part_one();
    println!("{redundant}");

    let overlapping = part_two();
    println!("{overlapping}");
}
