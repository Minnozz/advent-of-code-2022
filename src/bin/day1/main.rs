use itertools::Itertools;

fn main() {
    let lines = include_str!("input.txt").lines().collect_vec();
    let elf_calories = lines
        .split(|line| line.is_empty())
        .map(|lines| {
            lines
                .iter()
                .filter_map(|s| s.parse::<u32>().ok())
                .sum::<u32>()
        })
        .sorted()
        .collect_vec();

    // Part 1
    let max_total = elf_calories.last().unwrap();
    println!("The Elf carrying the most calories carries {}", max_total);

    // Part 2
    let top_3_total = elf_calories.iter().rev().take(3).sum::<u32>();
    println!("The top 3 Elves are carrying {} calories", top_3_total);
}
