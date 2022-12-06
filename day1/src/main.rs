use itertools::Itertools;

fn part_one() -> u64 {
    include_str!("input.txt")
        .lines()
        .map(|cal| cal.parse::<u64>().ok())
        .batching(|elf| {
            let mut sum = None;
            while let Some(Some(cal)) = elf.next() {
                sum = Some(sum.unwrap_or_default() + cal)
            }
            sum
        })
        .max()
        .unwrap()
}

fn part_two() -> u64 {
    include_str!("input.txt")
        .lines()
        .map(|cal| cal.parse::<u64>().ok())
        .batching(|mut elf| (&mut elf).map_while(|cal| cal).sum1::<u64>())
        .map(std::cmp::Reverse)
        .k_smallest(3)
        .map(|x| x.0)
        .sum::<u64>()
}

fn main() {
    let total_calories = part_one();
    println!("The leading elf is carrying: {total_calories}");

    let top_three_calories = part_two();
    println!("The top three elves are carrying: {top_three_calories}");
}
